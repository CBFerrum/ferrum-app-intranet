//! The company intranet (FERRUM-ECOSYSTEM-001): spaces for teams and topics, with who is in them,
//! their pages (a Drive folder, written in Docs) and their links; and announcements to everyone or
//! to a space's members, which stay as they were posted once published, are acknowledged once by
//! those they ask, and are archived after the day they expire.
//!
//! * `spaces`: who keeps a space and who is in it, and links that are web addresses or this
//!   workspace's pages, never scripts.
//! * `announcements`: publishing to an audience (a members-only space's members, told in the app),
//!   archiving, and acknowledgements.
//!
//! This app's own design; no code was copied (docs/PROVENANCE.md). Its hooks are registered per
//! entity, so a workspace without the `intranet` package never runs them.

mod announcements;
mod spaces;

use ferrum_domain::reports::ReportSource;
use ferrum_kernel::{Ctx, Error, HookRegistry, Kernel, Result};
use std::sync::Arc;
use uuid::Uuid;

include!(concat!(env!("OUT_DIR"), "/packages.rs"));
include!(concat!(env!("OUT_DIR"), "/ui.rs"));

/// This app's manifest (`app.yaml`).
pub const MANIFEST: &str = include_str!("../app.yaml");

/// This app's metadata packages, embedded at build time.
pub fn packages() -> std::result::Result<Vec<ferrum_meta::Package>, ferrum_meta::MetaError> {
    ferrum_meta::catalog::load_embedded(EMBEDDED)
}

/// Registers this app's document hooks.
pub fn register(hooks: &mut HookRegistry) {
    hooks.register("IntranetSpace", Arc::new(spaces::SpaceHook)).register("Announcement", Arc::new(announcements::AnnouncementHook));
}

/// This app's reports: none of its own yet.
pub fn reports() -> Vec<Arc<dyn ReportSource>> {
    vec![]
}

/// Upkeep: announcements past the day they expire are archived.
pub async fn maintenance(k: &Kernel) -> Result<()> {
    let mut tx = k.db.system_tx().await?;
    let tenants: Vec<Uuid> = sqlx::query_scalar("select id from tenant where status = 'active' order by id").fetch_all(&mut *tx).await?;
    tx.commit().await?;
    let mut first_error: Option<Error> = None;
    for tenant in tenants {
        let run = async {
            let schema = k.registry.schema(tenant).await?;
            if !schema.packages.iter().any(|p| p == "intranet") {
                return Ok(());
            }
            let ctx = Ctx::system(tenant, schema);
            let mut tx = k.db.tenant_tx(tenant).await?;
            announcements::archive_expired(k, &ctx, &mut tx).await?;
            tx.commit().await?;
            Ok::<(), Error>(())
        };
        if let Err(e) = run.await {
            first_error.get_or_insert(e);
        }
    }
    first_error.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_interface_is_a_release_the_platform_takes() {
        let m = ferrum_apps::AppManifest::parse(MANIFEST).expect("app.yaml");
        assert_eq!(m.routes(), vec![("intranet".to_string(), vec![])]);
        let files = UI.iter().map(|(p, b)| (p.to_string(), b.to_vec())).collect();
        let ui = ferrum_apps::ui::prepare("intranet", files).expect("ui/dist is an interface release");
        assert!(UI.iter().any(|(p, _)| *p == "index.html") && !UI.iter().any(|(p, _)| p.starts_with('.')), "{}", ui.digest);
    }

    #[test]
    fn links_go_to_the_web_or_this_workspace() {
        use spaces::link_ok;
        assert!(link_ok("https://example.com/handbook") && link_ok("http://intranet.local/x") && link_ok("/docs/d/42"));
        for bad in ["javascript:alert(1)", "//evil.example", "data:text/html,hi", "ftp://x", " https://x", "https://x y", "/docs/\n", ""] {
            assert!(!link_ok(bad), "{bad:?}");
        }
    }
}
