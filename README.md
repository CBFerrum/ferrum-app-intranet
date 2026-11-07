# Ferrum Intranet

**A native app for [Ferrum](https://github.com/CBFerrum/Ferrum), the open-source, AI-first business platform.**

## Overview

The company intranet: spaces for teams and topics, and announcements to everyone or to a team.

- **Spaces:** each holds who is in it, its pages (a Drive folder, written in Ferrum Docs) and its links.
- **Announcements:**
  - go to everyone, or to a members-only space's members alone;
  - stay as they were posted once published;
  - are acknowledged once by each reader they ask;
  - are archived after the day they expire.

| At a glance | |
|---|---|
| 📦 Version | `0.1.0` |
| 🧱 Platform | Ferrum `>=0.3.2, <0.4` |
| 🖥️ Interface | its own, at `/intranet/` |
| 🗃️ Records | 2 entities (and 4 tables) in 1 package |
| 👥 Roles | 2: Intranet Editor, Intranet Member |

## How it works

| Record | What it is |
|---|---|
| `IntranetSpace` | A team's or topic's space: `space_name`, `description`, `visibility` (Everyone or Members), `owner` (whoever made it, unless given), `pages_folder` (a Drive folder's id), `members` (`user_email`, `member_role`), `links` (`label`, `url`: `https://…`, `http://…` or `/…`; never a script or `//`), `active`. |
| `Announcement` | `title`, `body` (Markdown), `space` (empty: for everyone), `pinned`, `needs_acknowledgement`, `expires_on`. Set when it is published: `status` (Draft, Published, Archived), `audience` (Everyone or Space members), `published_by`, `published_at`, `recipients`. `acknowledgements` (`user_email`, `acknowledged_at`). |

| Action | Who | What it does |
|---|---|---|
| `publish` | Intranet Editor (write) | Publishes a draft. In a members-only space, it goes to the space's owner and members as they are then (its `recipients`); otherwise to everyone. Those it goes to are told in the app. |
| `archive` | Intranet Editor (write) | Takes a published announcement off the news. |
| `acknowledge` | whoever may read it | Records the reader's acknowledgement once, with when. A second time says when the first was. |

Who sees what:
- **Intranet Members** read published announcements meant for everyone, and those of members-only spaces they were published to.
- They see spaces open to everyone, and the members-only spaces they own or belong to.
- **Intranet Editors** read and write all of it.
- A published announcement keeps its title, body, space and whether it asks to be acknowledged. Archive it and post another to change them.
- Upkeep archives announcements the day after they expire, in the workspace's time zone.

## Limits

- **Recipients:** a members-only space's announcement goes to its members as they are when it is published. People who join later do not see earlier news.
- **Everyone notified:** announcements for everyone notify every active person in the workspace.
- **Pages:** these are Docs documents in the space's Drive folder. Sharing that folder is Drive's job, not the space's.
- **Directory:** there is no people directory here. HR's employees are the directory.

## Changelog

| Version | Highlights |
|---|---|
| **0.1.11** | The assistant knows its records. |
| **0.1.10** | Rebuilt with Ferrum-Frontend 5b22e7b: its own words in Arabic and Hindi. |
| **0.1.9** | Rebuilt with Ferrum-Frontend 6537d65: once-shown answers kept; English isolated right to left; its own words translatable by a workspace. |
| **0.1.8** | Rebuilt with Ferrum-Frontend d5d98f2: a workspace's own interface words; pictures in attachment fields; a duplicate value said so. |
| **0.1.7** | Rebuilt with Ferrum-Frontend 63280d3: right to left, pages mirror; the records pages' words in Arabic and Hindi. |
| **0.1.6** | Rebuilt with Ferrum-Frontend a64416a: a box whose default is 1 shows ticked. |
| **0.1.5** | Rebuilt with Ferrum-Frontend 922fcfb: action dialogs say every refusal and ask once. |
| **0.1.4** | Rebuilt with Ferrum-Frontend a76295e: times on a person's own clock when they chose one. |
| **0.1.3** | The news's days are the workspace's; rebuilt with Ferrum-Frontend 24c3951 (times on the workspace's clock). |
| **0.1.2** | From a browser walk: a published announcement's kept fields show read-only, archiving asks first, and the news says when you acknowledged. |
| **0.1.1** | Rebuilt with Ferrum-Frontend bb60115: empty numbers read as the server reads them; settings open as their record. |
| **0.1.0** | Team spaces with their pages and links, and announcements to everyone or a team, acknowledged where asked. |

## Develop

```bash
cargo test   # needs PostgreSQL: FERRUM_INTRANET_TEST_DATABASE_URL
cd ui && npm run typecheck && npm test && npm run build   # with Ferrum-Frontend at ../../ferrum/web
```

## License

AGPL-3.0-only.
