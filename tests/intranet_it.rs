//! A company intranet (FERRUM-ECOSYSTEM-001) against a real PostgreSQL
//! (`FERRUM_INTRANET_TEST_DATABASE_URL`, default `postgres://ferrum:ferrum@localhost:5432/ferrum_intranet_test`;
//! reset first): spaces open to everyone or to their members, with links that are never scripts;
//! announcements published to everyone, or to a members-only space's members alone, who are told
//! in the app; acknowledged once each; kept as posted once published; archived by hand or after
//! they expire; and who may do what.

use chrono::Duration;
use ferrum_kernel::secret::SecretKey;
use ferrum_kernel::tenant::NewTenant;
use ferrum_kernel::{Ctx, Db, DbConfig, Doc, Error, HookRegistry, Kernel, KernelSettings, ListQuery};
use ferrum_meta::Catalog;
use serde_json::{Value, json};
use std::sync::Arc;
use uuid::Uuid;

fn obj(v: Value) -> Doc {
    v.as_object().cloned().unwrap()
}

fn uid(v: &Value) -> Uuid {
    v.as_str().unwrap_or_else(|| panic!("not an id: {v}")).parse().unwrap()
}

struct T {
    k: Arc<Kernel>,
    tenant: Uuid,
    admin: Ctx,
}

impl T {
    async fn insert(&self, ctx: &Ctx, entity: &str, v: Value) -> Doc {
        self.k.insert(ctx, entity, obj(v)).await.unwrap_or_else(|e| panic!("insert {entity}: {e:?}"))
    }
    async fn get(&self, entity: &str, id: Uuid) -> Doc {
        self.k.get(&self.admin, entity, id, false).await.unwrap_or_else(|e| panic!("get {entity}: {e:?}"))
    }
    async fn user(&self, email: &str, roles: &[&str]) -> Ctx {
        let roles: Vec<String> = roles.iter().map(|r| r.to_string()).collect();
        let id = self.k.create_user(&self.admin, email, email, Some("Correct-Horse-9"), &roles, false).await.expect("user");
        let s = self.k.login("acme", email, "Correct-Horse-9").await.expect("login");
        self.k.load_ctx(self.tenant, id, Some(s.token_version), "it".into()).await.expect("ctx")
    }
    async fn act(&self, ctx: &Ctx, id: Uuid, action: &str) -> Result<Value, Error> {
        self.k.run_action(ctx, "Announcement", id, action, Value::Null).await
    }
    /// The announcements `ctx` may list.
    async fn seen(&self, ctx: &Ctx) -> Vec<String> {
        let rows = self.k.list(ctx, "Announcement", &ListQuery::default()).await.expect("list").rows;
        let mut titles: Vec<String> = rows.iter().filter_map(|r| r.get("title").and_then(Value::as_str).map(str::to_string)).collect();
        titles.sort();
        titles
    }
    async fn told(&self, user: Uuid) -> i64 {
        let mut tx = self.k.db.tenant_tx(self.tenant).await.unwrap();
        let n: i64 = sqlx::query_scalar("select count(*) from notification where tenant_id = $1 and user_id = $2 and title like 'Announcement:%'")
            .bind(self.tenant)
            .bind(user)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        n
    }
}

fn refused<V: std::fmt::Debug>(r: Result<V, Error>, kind: &str, says: &str) {
    let msg = format!("{r:?}");
    let ok = matches!(
        (&r, kind),
        (Err(Error::Invalid(_) | Error::Validation(_)), "invalid") | (Err(Error::Forbidden(_)), "forbidden") | (Err(Error::NotFound(_)), "not found")
    );
    assert!(ok && msg.contains(says), "expected {kind} saying {says:?}, got {msg}");
}

#[test]
fn a_company_intranet() {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().thread_stack_size(16 << 20).build().unwrap();
    rt.block_on(async { tokio::spawn(scenario()).await.unwrap() });
}

async fn scenario() {
    let url = std::env::var("FERRUM_INTRANET_TEST_DATABASE_URL").unwrap_or_else(|_| "postgres://ferrum:ferrum@localhost:5432/ferrum_intranet_test".into());
    let db = Db::connect(DbConfig { url, max_connections: 16, ..Default::default() }).await.expect("connect");
    sqlx::raw_sql("drop schema if exists public cascade; create schema public;").execute(&db.pool).await.expect("reset");
    let mut packages = ferrum_domain::packages().expect("platform packages");
    packages.extend(ferrum_app_intranet::packages().expect("app packages"));
    let catalog = Catalog::from_packages(packages).expect("catalog");
    let mut hooks = HookRegistry::default();
    ferrum_domain::register(&mut hooks);
    ferrum_app_intranet::register(&mut hooks);
    let settings = KernelSettings { secret_key: Some(SecretKey::new([7u8; 32])), ..Default::default() };
    let k = Kernel::boot(db, catalog, hooks, settings, None).await.expect("boot");
    ferrum_domain::maintenance(&k).await.expect("partitions");
    let tenant = k
        .create_tenant(NewTenant {
            slug: "acme".into(),
            name: "Acme".into(),
            admin_email: "admin@acme.test".into(),
            admin_password: "Correct-Horse-9".into(),
            admin_name: None,
            packages: vec!["intranet".into()],
            settings: Some(json!({"timezone": "UTC"})),
        })
        .await
        .expect("tenant");
    let session = k.login("acme", "admin@acme.test", "Correct-Horse-9").await.expect("login");
    let admin = k.load_ctx(tenant, session.user, Some(session.token_version), "it".into()).await.expect("ctx");
    let t = T { k: k.clone(), tenant, admin };
    let editor = t.user("comms@acme.test", &["Intranet Editor"]).await;
    let ana = t.user("ana@acme.test", &["Intranet Member"]).await;
    let ben = t.user("ben@acme.test", &["Intranet Member"]).await;

    // ───────────── spaces ─────────────
    let script = t.k.insert(&editor, "IntranetSpace", obj(json!({"space_name": "Bad", "links": [{"label": "Click", "url": "javascript:alert(1)"}]}))).await;
    refused(script, "invalid", "must be a web address");
    let folder = t.k.insert(&editor, "IntranetSpace", obj(json!({"space_name": "Bad", "pages_folder": "not-an-id"}))).await;
    refused(folder, "invalid", "Drive folder");
    let company = t
        .insert(&editor, "IntranetSpace", json!({"space_name": "Company", "links": [{"label": "Handbook", "url": "https://handbook.acme.test"}, {"label": "Policies", "url": "/docs/f/policies"}]}))
        .await;
    assert_eq!(company["owner"], "comms@acme.test", "whoever made it keeps it");
    let eng = t
        .insert(
            &editor,
            "IntranetSpace",
            json!({"space_name": "Engineering", "visibility": "Members", "owner": "Lead@Acme.test",
                   "members": [{"user_email": "Ana@acme.test"}, {"user_email": "ana@acme.test", "member_role": "Editor"}]}),
        )
        .await;
    assert_eq!((eng["owner"].clone(), eng["members"].as_array().unwrap().len()), (json!("lead@acme.test"), 1), "kept once each, in lower case");
    let eng_id = uid(&eng["id"]);
    assert!(t.k.get(&ana, "IntranetSpace", eng_id, false).await.is_ok(), "a member sees a members-only space");
    refused(t.k.get(&ben, "IntranetSpace", eng_id, false).await, "not found", "");
    assert!(t.k.get(&ben, "IntranetSpace", uid(&company["id"]), false).await.is_ok(), "everyone sees an open space");
    refused(t.k.insert(&ana, "IntranetSpace", obj(json!({"space_name": "Mine"}))).await, "forbidden", "");

    // ───────────── announcements ─────────────
    let news = |title: &str, space: Option<Uuid>, ack: bool| json!({"title": title, "body": "**Read me**", "space": space.map(|s| s.to_string()), "needs_acknowledgement": ack});
    let all = uid(&t.insert(&editor, "Announcement", news("Office closed Friday", None, true)).await["id"]);
    let team = uid(&t.insert(&editor, "Announcement", news("Release freeze", Some(eng_id), false)).await["id"]);
    assert!(t.seen(&ana).await.is_empty() && t.seen(&ben).await.is_empty(), "drafts are the editors'");
    refused(t.act(&ana, all, "publish").await, "forbidden", "");
    let published = t.act(&editor, all, "publish").await.expect("published");
    assert_eq!((published["audience"].clone(), published["told"].clone()), (json!("Everyone"), json!(4)), "everyone is told");
    let to_team = t.act(&editor, team, "publish").await.expect("published to the team");
    assert_eq!((to_team["audience"].clone(), to_team["recipients"].clone()), (json!("Space members"), json!(2)), "its owner and members");
    assert_eq!(t.seen(&ana).await, ["Office closed Friday", "Release freeze"]);
    assert_eq!(t.seen(&ben).await, ["Office closed Friday"], "a members-only space's news is its members' alone");
    refused(t.k.get(&ben, "Announcement", team, false).await, "not found", "");
    assert_eq!((t.told(ana.user).await, t.told(ben.user).await), (2, 1));
    refused(t.act(&editor, all, "publish").await, "invalid", "is not available for this document");

    // Kept as posted: the words do not change, whether it is pinned does.
    let k2 = t.k.update(&editor, "Announcement", all, obj(json!({"body": "Open after all"})), None).await;
    refused(k2, "invalid", "stays as it was posted");
    t.k.update(&editor, "Announcement", all, obj(json!({"pinned": true})), None).await.expect("pinned");

    // Acknowledged once each, by those it asks.
    let first = t.act(&ana, all, "acknowledge").await.expect("acknowledged");
    assert_eq!(first["already"], json!(false));
    assert_eq!(t.act(&ana, all, "acknowledge").await.expect("again")["already"], json!(true));
    t.act(&ben, all, "acknowledge").await.expect("acknowledged");
    let acks = t.get("Announcement", all).await["acknowledgements"].as_array().unwrap().iter().map(|a| a["user_email"].clone()).collect::<Vec<_>>();
    assert_eq!(acks, [json!("ana@acme.test"), json!("ben@acme.test")]);
    refused(t.act(&ana, team, "acknowledge").await, "invalid", "is not available for this document");
    refused(t.act(&ben, team, "acknowledge").await, "not found", "");

    // Archived by hand, or once its last day has passed.
    t.act(&editor, team, "archive").await.expect("archived");
    assert_eq!(t.seen(&ana).await, ["Office closed Friday"], "archived news leaves the feed");
    let today = chrono::Utc::now().date_naive();
    t.k.update(&editor, "Announcement", all, obj(json!({"expires_on": (today - Duration::days(1)).to_string()})), None).await.expect("expires");
    ferrum_app_intranet::maintenance(&k).await.expect("upkeep");
    assert_eq!(t.get("Announcement", all).await["status"], "Archived");
    assert!(t.seen(&ben).await.is_empty());
}
