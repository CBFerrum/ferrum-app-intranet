//! Spaces: whoever makes one keeps it unless another owner is given; members are kept once each,
//! by their address in lower case; links are web addresses or this workspace's own pages (never a
//! script, nor an address another site could be given through); its pages folder is a Drive
//! folder's id.

use async_trait::async_trait;
use ferrum_domain::util::{rows, s};
use ferrum_kernel::{Doc, DocHook, Error, FieldError, HookCtx, Result};
use serde_json::{Value, json};
use std::collections::HashSet;
use uuid::Uuid;

/// Whether a link may be shown: `https://` or `http://` to another site, or a path of this
/// workspace (`/docs/...`), with nothing that could carry a second meaning.
pub(crate) fn link_ok(url: &str) -> bool {
    let plain = !url.is_empty() && !url.chars().any(|c| c.is_whitespace() || c.is_control());
    plain && (url.starts_with("https://") || url.starts_with("http://") || (url.starts_with('/') && !url.starts_with("//")))
}

pub struct SpaceHook;

#[async_trait]
impl DocHook for SpaceHook {
    fn name(&self) -> &'static str {
        "intranet.space"
    }

    fn fills(&self) -> &'static [&'static str] {
        &["owner"]
    }

    async fn before_validate(&self, hc: &mut HookCtx<'_>, doc: &mut Doc) -> Result<()> {
        let owner = s(doc, "owner").map(str::to_string).unwrap_or_else(|| hc.ctx.email.clone());
        if !owner.is_empty() {
            doc.insert("owner".into(), json!(owner.trim().to_lowercase()));
        }
        let mut seen = HashSet::new();
        let members: Vec<Value> = rows(doc, "members")
            .into_iter()
            .filter_map(|mut m| {
                let email = s(&m, "user_email")?.trim().to_lowercase();
                if !seen.insert(email.clone()) {
                    return None;
                }
                m.insert("user_email".into(), json!(email));
                Some(Value::Object(m))
            })
            .collect();
        if doc.contains_key("members") {
            doc.insert("members".into(), Value::Array(members));
        }
        Ok(())
    }

    async fn validate(&self, _hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        let mut errors = Vec::new();
        for (i, link) in rows(doc, "links").iter().enumerate() {
            if !link_ok(s(link, "url").unwrap_or_default()) {
                errors.push(FieldError::new("url", "must be a web address (https://…) or a page of this workspace (/…)").in_row("links", i));
            }
        }
        if s(doc, "pages_folder").is_some_and(|f| f.parse::<Uuid>().is_err()) {
            errors.push(FieldError::new("pages_folder", "must be a Drive folder's id"));
        }
        if errors.is_empty() { Ok(()) } else { Err(Error::Validation(errors)) }
    }
}
