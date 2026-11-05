//! Announcements.
//!
//! * Written as drafts. Published by an Intranet Editor, one goes to everyone, or, in a
//!   members-only space, to its owner and members as they are then (kept as its recipients, who
//!   alone may read it); those it goes to are told in the app.
//! * Published, it stays as it was posted: only whether it is pinned and when it expires change.
//!   Archived by hand, or the day after it expires.
//! * One that asks is acknowledged by each reader once, with when.

use async_trait::async_trait;
use chrono::Utc;
use ferrum_domain::util::{b, id, rows, s, sys};
use ferrum_kernel::{Ctx, Doc, DocHook, Error, HookCtx, Kernel, Result, Tx, notify};
use serde_json::{Value, json};
use uuid::Uuid;

/// What a published announcement keeps as it was posted.
const POSTED: &[&str] = &["title", "body", "space", "needs_acknowledgement"];

pub struct AnnouncementHook;

#[async_trait]
impl DocHook for AnnouncementHook {
    fn name(&self) -> &'static str {
        "intranet.announcement"
    }

    async fn validate_change(&self, _hc: &mut HookCtx<'_>, old: Option<&Doc>, doc: &Doc) -> Result<()> {
        let Some(old) = old else { return Ok(()) };
        let changed = |f: &&str| match *f {
            "needs_acknowledgement" => b(old, f) != b(doc, f),
            _ => old.get(*f).unwrap_or(&Value::Null) != doc.get(*f).unwrap_or(&Value::Null),
        };
        if s(old, "status").is_some_and(|st| st != "Draft") && POSTED.iter().any(changed) {
            return Err(Error::invalid("a published announcement stays as it was posted; archive it and post another"));
        }
        Ok(())
    }

    async fn action(&self, hc: &mut HookCtx<'_>, name: &str, doc: &Doc, _params: &Value) -> Result<Option<Value>> {
        let announcement = id(doc, "id").ok_or_else(|| Error::internal("an announcement without an id"))?;
        let status = s(doc, "status").unwrap_or("Draft");
        let out = match name {
            "publish" => {
                if status != "Draft" {
                    return Err(Error::invalid(format!("this announcement is {}; only a draft is published", status.to_lowercase())));
                }
                publish(hc, doc, announcement).await?
            }
            "archive" => {
                if status != "Published" {
                    return Err(Error::invalid("only a published announcement is archived"));
                }
                let mut v = Doc::new();
                v.insert("status".into(), json!("Archived"));
                hc.kernel.set_values_tx(&sys(hc.ctx), hc.tx, "Announcement", announcement, v).await?;
                json!({"status": "Archived"})
            }
            "acknowledge" => {
                if status != "Published" || !b(doc, "needs_acknowledgement") {
                    return Err(Error::invalid("this announcement asks for no acknowledgement"));
                }
                let me = hc.ctx.email.trim().to_lowercase();
                if me.is_empty() {
                    return Err(Error::Forbidden("an acknowledgement is a person's".into()));
                }
                let had = rows(doc, "acknowledgements");
                if let Some(done) = had.iter().find(|a| s(a, "user_email").is_some_and(|e| e.eq_ignore_ascii_case(&me))) {
                    return Ok(Some(json!({"acknowledged_at": done.get("acknowledged_at"), "already": true})));
                }
                // The rows as they are, by their own fields, with this one after them.
                let keep = ["id", "user_email", "acknowledged_at"];
                let mut acks: Vec<Doc> = had.into_iter().map(|a| a.into_iter().filter(|(k, _)| keep.contains(&k.as_str())).collect()).collect();
                let at = json!(Utc::now().to_rfc3339());
                acks.push(Doc::from_iter([("user_email".to_string(), json!(me)), ("acknowledged_at".to_string(), at.clone())]));
                hc.kernel.set_child_rows_tx(&sys(hc.ctx), hc.tx, "Announcement", "acknowledgements", announcement, acks).await?;
                json!({"acknowledged_at": at, "already": false})
            }
            _ => return Ok(None),
        };
        Ok(Some(out))
    }
}

/// Publishes a draft to its audience and tells them.
async fn publish(hc: &mut HookCtx<'_>, doc: &Doc, announcement: Uuid) -> Result<Value> {
    let mut recipients: Vec<String> = Vec::new();
    let mut audience = "Everyone";
    if let Some(space) = id(doc, "space") {
        let space = hc.kernel.get_tx(&sys(hc.ctx), hc.tx, "IntranetSpace", space, false).await?;
        if !b(&space, "active") {
            return Err(Error::field("space", "is not active"));
        }
        if s(&space, "visibility") == Some("Members") {
            audience = "Space members";
            let mut people: Vec<String> = s(&space, "owner").into_iter().map(str::to_string).collect();
            people.extend(rows(&space, "members").iter().filter_map(|m| s(m, "user_email").map(str::to_string)));
            for p in people.into_iter().map(|p| p.trim().to_lowercase()) {
                if !p.is_empty() && !recipients.contains(&p) {
                    recipients.push(p);
                }
            }
        }
    }
    let ctx = sys(hc.ctx);
    let mut v = Doc::new();
    v.insert("status".into(), json!("Published"));
    v.insert("audience".into(), json!(audience));
    v.insert("published_by".into(), json!(hc.ctx.email.to_lowercase()));
    v.insert("published_at".into(), json!(Utc::now().to_rfc3339()));
    hc.kernel.set_values_tx(&ctx, hc.tx, "Announcement", announcement, v).await?;
    let rows: Vec<Doc> = recipients.iter().map(|e| Doc::from_iter([("user_email".to_string(), json!(e))])).collect();
    hc.kernel.set_child_rows_tx(&ctx, hc.tx, "Announcement", "recipients", announcement, rows).await?;
    let title = format!("Announcement: {}", s(doc, "title").unwrap_or_default());
    let told = tell(hc.tx, hc.ctx.tenant, (audience == "Space members").then_some(recipients.as_slice()), &title, announcement).await?;
    Ok(json!({"status": "Published", "audience": audience, "recipients": recipients.len(), "told": told}))
}

/// In-app notifications for the active people an announcement goes to: those with these
/// addresses, or everyone. Returns how many were told.
async fn tell(tx: &mut Tx, tenant: Uuid, only: Option<&[String]>, title: &str, announcement: Uuid) -> Result<usize> {
    let users: Vec<Uuid> = match only {
        Some(emails) => {
            sqlx::query_scalar("select id from app_user where tenant_id = $1 and is_active and lower(email::text) = any($2) order by id")
                .bind(tenant)
                .bind(emails)
                .fetch_all(&mut **tx)
                .await?
        }
        None => sqlx::query_scalar("select id from app_user where tenant_id = $1 and is_active order by id").bind(tenant).fetch_all(&mut **tx).await?,
    };
    for user in &users {
        notify::send(tx, tenant, *user, title, None, Some("Announcement"), Some(announcement)).await?;
    }
    Ok(users.len())
}

/// Archives the published announcements whose last day has passed in the workspace's time zone.
pub(crate) async fn archive_expired(k: &Kernel, ctx: &Ctx, tx: &mut Tx) -> Result<usize> {
    let due: Vec<Uuid> = sqlx::query_scalar("select id from e_announcement where tenant_id = $1 and status = 'Published' and expires_on < $2 order by id")
        .bind(ctx.tenant)
        .bind(ctx.today())
        .fetch_all(&mut **tx)
        .await?;
    for announcement in &due {
        let mut v = Doc::new();
        v.insert("status".into(), json!("Archived"));
        k.set_values_tx(&sys(ctx), tx, "Announcement", *announcement, v).await?;
    }
    Ok(due.len())
}
