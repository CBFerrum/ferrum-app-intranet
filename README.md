<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/assets/banner-dark.svg">
  <img alt="Ferrum Intranet: news for everyone or a team, and the teams' spaces" src="docs/assets/banner-light.svg" width="100%">
</picture>

<br>

[![Version](https://img.shields.io/badge/version-0.1.14-7c3aed?style=flat-square)](CHANGELOG.md)
[![Licence: AGPL-3.0](https://img.shields.io/badge/licence-AGPL--3.0-blue?style=flat-square)](LICENSE)
[![Ferrum platform](https://img.shields.io/badge/runs%20on-Ferrum-2f5bea?style=flat-square)](https://github.com/CBFerrum/Ferrum)
![Rust](https://img.shields.io/badge/Rust-server-b7410e?style=flat-square&logo=rust)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-data-336791?style=flat-square&logo=postgresql&logoColor=white)
![React](https://img.shields.io/badge/React-TypeScript-0ea5e9?style=flat-square&logo=react&logoColor=white)
![Tests](https://img.shields.io/badge/tests-3%20Rust%20%2B%204%20UI-16a34a?style=flat-square)

### Share company news with everyone or one team, and know who has read what they must.

Intranet gives a company a news page and a place for each team. Editors post announcements to everyone or to the members of a space, readers acknowledge the ones that ask, and every space keeps its members, its pages (written in Ferrum Docs) and its links. It replaces the all-staff email and the pinned message nobody can find again.

[💡 Why](#-why-intranet) · [📸 Screenshots](#-screenshots) · [✨ Features](#-features) · [📊 Compare](#-how-it-compares) · [📍 Roadmap](#-roadmap) · [🚀 Install](#-install) · [🔧 Develop](#-develop)

</div>

## 💡 Why Intranet

- 📣 **News that stays as posted.** A published announcement keeps its title, body, space and whether it asks to be acknowledged. To change it, archive it and post another, so nobody can say later that it read differently.
- ✅ **Acknowledgements that count.** An announcement can ask each reader to acknowledge it once. The reader's name and the time are kept on it.
- 🎯 **Right audience.** News goes to everyone, or only to the owner and members of a members-only space, and only they can read it.
- 🗂️ **Spaces with their own pages.** A space's pages are Ferrum Docs documents in a Drive folder, so they have the editor, history and sharing of Docs.
- 🧩 **One data set.** People are the workspace's people, and HR's employees are the directory. Nothing is copied into a separate system.
- 🔓 **Open source and self-hosted.** AGPL, on your servers, with your data.
- 🌍 **Arabic, Hindi and English,** with right-to-left mirroring.

## 🎯 At a glance

| | |
|---|---|
| 📦 Version | `0.1.14` |
| 🧱 Kind | Own interface at `/intranet/` |
| 🗃️ What it adds | 2 entities (Announcement, Space) and 4 child tables (Member, Link, Recipient, Acknowledgement) in the `intranet` package |
| 👥 Roles | 2: Intranet Editor, Intranet Member |
| 📊 Reports and print formats | none of its own |
| ⏱️ Upkeep | archives expired announcements |
| 🔗 Depends on | Ferrum `>=0.3.2, <0.4` |
| 🤝 Works with | Docs and Drive (a space's pages), HR (the directory), Ferrum notifications |
| 🌍 Languages | English, Arabic and Hindi (translated by a model, to be reviewed by native speakers) |
| 🧪 Tests | 3 Rust tests (one against a real PostgreSQL), 4 interface tests |
| 📄 Licence | AGPL-3.0-only |

## 📸 Screenshots

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/intranet-home-dark.webp">
  <img alt="Intranet home" src="docs/assets/screens/intranet-home-light.webp" width="100%">
</picture>

<p align="center"><b>📰 Intranet home</b><br><sub>Read the news meant for you, pinned first, and see the spaces you may enter.</sub></p>

<table>
<tr>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/intranet-announcement-list-dark.webp"><img alt="Announcements list" src="docs/assets/screens/intranet-announcement-list-light.webp"></picture>
<p><b>📣 Announcements</b><br><sub>Browse, search and filter announcements by space, audience and status.</sub></p>
</td>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/intranet-announcement-detail-dark.webp"><img alt="One announcement" src="docs/assets/screens/intranet-announcement-detail-light.webp"></picture>
<p><b>📝 One announcement</b><br><sub>Write, publish or archive one announcement, and see who acknowledged it.</sub></p>
</td>
</tr>
<tr>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/intranet-intranet-space-list-dark.webp"><img alt="Spaces list" src="docs/assets/screens/intranet-intranet-space-list-light.webp"></picture>
<p><b>🗂️ Spaces</b><br><sub>Browse, search and filter the spaces teams and topics keep.</sub></p>
</td>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/intranet-spaces-dark.webp"><img alt="Intranet spaces" src="docs/assets/screens/intranet-spaces-light.webp"></picture>
<p><b>🧭 Intranet spaces</b><br><sub>Browse the spaces teams keep their pages in.</sub></p>
</td>
</tr>
</table>

<p align="center">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/intranet-home-phone-dark.webp"><img alt="Intranet home on a phone" src="docs/assets/screens/intranet-home-phone-light.webp" width="260"></picture>
&nbsp;&nbsp;
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/intranet-announcement-list-phone-dark.webp"><img alt="Announcements on a phone" src="docs/assets/screens/intranet-announcement-list-phone-light.webp" width="260"></picture>
</p>

<p align="center"><b>📱 On a phone</b><br><sub>The news and the announcements list on a narrow screen.</sub></p>

## ✨ Features

### 📰 The news page

- **News first.** The home page lists the published announcements meant for the reader, pinned ones first and then the newest. The server decides what a reader may see; the page only draws it.
- **Markdown body.** Each announcement shows its title, a *Pinned* badge, the space it belongs to, the day it was posted on the workspace's clock, and its Markdown body.
- **Acknowledge in place.** An announcement that asks shows an Acknowledge button until the reader has done it; afterwards it says "You acknowledged it on" and the day.
- **Spaces beside the news.** A side list shows each active space the reader may see, a *Members* badge for members-only ones, a **Pages** button to the space's Drive folder and a button for each link.
- **Always current.** The page refreshes every minute.
- **Honest states.** Loading shows skeletons, a failed load offers a retry, and empty lists say "Nothing new." and "No spaces yet."
- **All announcements** is one button away for the full list, with search and filters.

### 📣 Announcements

- **Announcement.** A title (up to 200 characters), a Markdown body (up to 50,000), an optional space, whether it is pinned, whether it asks to be acknowledged, and an expiry day.
- **Draft, Published, Archived.** A draft can be edited freely. Publishing asks first ("Once published it stays as it is").
- **Publish** (Intranet Editor). It goes to everyone, or to the members of a members-only space as they are at that moment, kept on it as its recipients. The audience is set from the space (Everyone or Space members) and cannot be edited by hand. A space that is not active cannot be posted to.
- **Told in the app.** Everyone it goes to gets an in-app notification titled "Announcement: …". An announcement for everyone notifies every active person.
- **Stays as posted.** After publishing, the title, body, space and acknowledgement request are read-only. Only whether it is pinned and its expiry day can change.
- **Archive.** Takes a published announcement off the news, after a confirmation. It is not published again.
- **Acknowledge.** A reader who may read it acknowledges once. The time is kept; a second try answers with when the first was. Acknowledgements are only a person's, never a service's.
- **Expiry.** Upkeep archives an announcement the day after it expires, in the workspace's time zone.
- **Pinned.** Shown first while it is published.

### 🗂️ Spaces

- **Space.** A name (unique), a description, visibility (Everyone or Members), an owner, a pages folder, members, links and an active box.
- **Owner.** Whoever makes a space keeps it unless another owner is given.
- **Members.** An email and a role (Member or Editor), each address kept once, in lower case.
- **Links.** A label and an address that is a web address (`https://`, `http://`) or a page of this workspace (`/docs/…`). Never a script, and never an address another site could be given through (`//`). The home page checks the same rule the server keeps.
- **Pages.** A Drive folder's id: the space's pages are Ferrum Docs documents in that folder, and sharing the folder is Drive's job.
- **Members-only spaces** are seen by their owner and members alone, and so is what is announced in them.

### 🔐 Who sees what

| Role | Can do |
|---|---|
| **Intranet Editor** | Reads, writes, creates, deletes and exports all announcements and spaces; publishes and archives. |
| **Intranet Member** | Reads published announcements meant for everyone, and those of members-only spaces they were published to; reads open spaces and members-only spaces they own or belong to; acknowledges announcements that ask. |

- **Assistant.** Ferrum's assistant knows the states of announcements and suggests "What was announced this week?", "Which announcements have I not acknowledged?" and "Who has not acknowledged an announcement yet?".
- **Create menu.** An *Announcement* entry in the Ferrum create menu starts a new draft.
- **Language.** English, Arabic and Hindi; pages mirror right to left.

## 🧭 How it works

```mermaid
flowchart LR
    ED["Intranet Editor"] --> DR["Announcement (Draft)"]
    DR -- "publish" --> PUB["Published: stays as posted"]
    PUB -- "everyone" --> ALL["Every active person is told"]
    PUB -- "members-only space" --> MEM["Owner and members as they are now"]
    ALL --> RD["Readers read the news"]
    MEM --> RD
    RD -- "if it asks" --> ACK["Acknowledged once, with the time"]
    PUB -- "archive, or the day after it expires" --> ARC["Archived"]
```

An editor writes a draft and publishes it. Publishing fixes what it says and who it goes to. Readers see it on their home page, acknowledge it if asked, and it leaves the news when archived or expired.

```mermaid
flowchart TD
    SP["Space"] --> MB["Members and owner"]
    SP --> LK["Links"]
    SP --> FD["Drive folder"]
    FD --> DOC["Docs documents: the space's pages"]
    SP -. "members-only: audience of its news" .-> AN["Announcements"]
    HR["HR employees"] -. "the directory" .-> SP
```

A space holds people, links and a Drive folder. The pages themselves are Docs documents, and the directory of people is HR's.

## 🧱 Data model

### Entities

| Entity | What it is | Key fields or states |
|---|---|---|
| Space | A team's or topic's place | name, visibility (Everyone / Members), owner, pages folder, active |
| Member (child) | Who is in a space | email, role (Member / Editor) |
| Link (child) | A link on a space | label, address |
| Announcement | News for everyone or a space's members | Draft / Published / Archived, audience, space, pinned, needs acknowledgement, expires on |
| Recipient (child) | Who a members-only announcement went to | email |
| Acknowledgement (child) | Who acknowledged and when | email, acknowledged at |

### Actions

| Action | Who | What it does |
|---|---|---|
| `publish` | Editor | Publishes a draft to its audience and tells them. |
| `archive` | Editor | Takes a published announcement off the news. |
| `acknowledge` | Anyone who may read it | Records the reader's acknowledgement once, with when. |

### Roles, reports and upkeep

Roles are in [Who sees what](#-features). The app has no reports or print formats of its own. One upkeep job archives announcements the day after they expire.

## 🔌 Integrations

| With | How | Needs |
|---|---|---|
| Docs and Drive | A space's pages are Docs documents in its Drive folder; the home page links to `/drive/{folder}` | Drive and Docs installed |
| HR | Its employees are the directory; the intranet has no directory of its own | HR installed |
| Ferrum notifications | In-app notices when an announcement is published | Nothing extra |
| Ferrum's assistant and MCP | The records are available to AI agents under each person's rights | Nothing extra |

## 📊 How it compares

The leaders are employee-communication platforms: Microsoft SharePoint with Viva Connections, Staffbase, Workvivo, Simpplr and Happeo. Intranet is deliberately small: news, acknowledgements and team spaces. It has no social feed, directory or analytics yet.

| Capability | Ferrum Intranet | SharePoint + Viva | Staffbase | Workvivo | Simpplr | Happeo |
|---|:-:|:-:|:-:|:-:|:-:|:-:|
| Company news with pinned posts | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| News to a team's members only | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Audience by dynamic group, role or location | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Posts that stay as posted once published | ✅ | 🟡 | 🟡 | 🟡 | 🟡 | 🟡 |
| Acknowledgement of must-read posts | ✅ | 🟡 | ✅ | 🟡 | ✅ | 🟡 |
| Expiry and automatic archive | ✅ | 🟡 | ✅ | 🟡 | ✅ | 🟡 |
| Team spaces or sites | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Pages and wiki content | 🟡 | ✅ | ✅ | 🟡 | ✅ | ✅ |
| Links per space | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Rich editor with images and video | 🟡 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Comments and reactions | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Social feed and communities | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Employee directory | 🟡 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Email newsletters and digests | ❌ | 🟡 | ✅ | 🟡 | ✅ | ✅ |
| Scheduled publishing | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Content analytics | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Mobile app and push | 🟡 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Search across content | 🟡 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Translation of content | ❌ | 🟡 | ✅ | 🟡 | 🟡 | ✅ |
| Integrates with the suite you already have | 🟡 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Same data as HR, documents and the rest of the business | ✅ | 🟡 | ❌ | ❌ | ❌ | ❌ |
| Open source | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Self-hosted | ✅ | 🟡 | ❌ | ❌ | ❌ | ❌ |

Notes: 🟡 for Ferrum means a part is there: pages are Docs documents in a folder, not a page builder; the body is Markdown; the directory is HR's employees; search is the platform's search of records; mobile is the responsive web app with in-app notices; integrations are with other Ferrum apps. 🟡 for SharePoint's self-hosting means its server edition on your own machines.

### 🏆 Where Intranet stands out

- ✅ **Acknowledgement is built in** and kept on the announcement with the time.
- 🔒 **Immutable posts.** What was posted cannot be quietly changed.
- 🧩 **No separate silo.** One workspace, one set of people, HR and Docs in reach.
- 🔓 **Open source and self-hosted.**

### 🚧 Where the leaders are still ahead

- 💬 **Engagement.** Comments, reactions, communities and a social feed.
- 🎨 **Authoring.** Rich editors, media, scheduled publishing and templates.
- 📈 **Reach and measurement.** Newsletters, push, analytics and translation.
- 🔗 **Ecosystem.** Many integrations and a mature mobile app.

> [!NOTE]
> Leader columns are from public product information as of November 2026 and may change.

## 📍 Roadmap

### ✅ Recently shipped

- [x] **0.1.14** Rebuilt on the tuned design system: better menu icons, a lighter first page, a visible scroll cue, cleaner print.
- [x] **0.1.13** The redesigned interface: shaped loading, told-apart states.
- [x] **0.1.12** Names in Arabic and Hindi.
- [x] **0.1.11** The assistant knows its records.
- [x] **0.1.3** The news's days are the workspace's.
- [x] **0.1.2** A published announcement's kept fields show read-only, archiving asks first, and the news says when you acknowledged.
- [x] **0.1.0** Team spaces with their pages and links, and announcements to everyone or a team, acknowledged where asked.

### 🔜 Next up

- [ ] **Comments and reactions** on an announcement.
- [ ] **Audiences by group, role or department,** beyond everyone and a space's members.
- [ ] **A rich editor with images.** The body is Markdown today.
- [ ] **A reader report.** Who has not acknowledged yet, in one view (today it is on each announcement).
- [ ] **Scheduled publishing.** Today an editor publishes by hand.

### 🔭 Later

- [ ] Email digests and newsletters.
- [ ] Reach analytics for news.
- [ ] A people directory view, over HR's employees.
- [ ] Late joiners see earlier news of a space (today only those who were members at publishing time are recipients).
- [ ] A page tree and templates for a space's pages.
- [ ] Translation of announcements.
- [ ] Mobile push.
- [ ] Communities and a social feed.

## 🚀 Install

It is compiled into the Ferrum server. A workspace administrator adds **Intranet** from **Apps** (`/store/`). Pages in spaces use **Docs** and **Drive**, and the directory is **HR**; install those to use them.

First steps:

1. Give **Intranet Editor** to the people who write news and keep spaces, and **Intranet Member** to everyone else.
2. Create a space for each team or topic; set its visibility, owner, members and links, and the Drive folder for its pages.
3. Write an announcement, choose a space or leave it empty for everyone, tick *Ask to Acknowledge* if needed, and publish.

Limits to know:
- A members-only space's announcement goes to its members as they are when it is published; people who join later do not see earlier news.
- There is no people directory here: HR's employees are the directory.

## 🔧 Develop

```bash
cargo test   # needs PostgreSQL: FERRUM_INTRANET_TEST_DATABASE_URL
cd ui && npm run typecheck && npm test && npm run build   # with Ferrum-Frontend at ../../ferrum/web
```

CI creates a non-superuser role (so row-level security applies) and the database `ferrum_intranet_test`, and sets `FERRUM_INTRANET_TEST_DATABASE_URL=postgres://ferrum:ferrum@localhost:5432/ferrum_intranet_test`. It runs `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test`, then the interface checks against a Ferrum-Frontend checkout.

## 📂 Project layout

```text
app.yaml          the manifest: id, version, what it adds to Ferrum
packages/intranet/   entities and roles
src/              announcements (publish, archive, acknowledge, expiry) and spaces (owner, members, links)
ui/src/           Home.tsx (news and spaces), feed.ts (order, acknowledgement, link rule), locales
tests/            intranet_it.rs: integration tests against a real PostgreSQL
docs/             PROVENANCE.md
```

## 📜 Changelog

Every version is in [CHANGELOG.md](CHANGELOG.md).

## 🤝 Contributing

Issues and pull requests are welcome at [github.com/CBFerrum/ferrum-app-intranet/issues](https://github.com/CBFerrum/ferrum-app-intranet/issues).

## 📄 License

[AGPL-3.0-only](LICENSE), like the platform.

<div align="center">

<sub>Part of <a href="https://github.com/CBFerrum/Ferrum">Ferrum</a>, the open business platform · Made with 🦀 Rust and 🐘 PostgreSQL</sub>

</div>
