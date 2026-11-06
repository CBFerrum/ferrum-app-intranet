# Changelog

## 0.1.2

From a browser walk on a running server (announcements to everyone and to a members-only space,
published, acknowledged by a member, and archived):

- A published or archived announcement's title, body, space and Ask to Acknowledge show read-only
  on its page. Before, they looked editable until saving was refused.
- Archive asks first, since an archived announcement is not published again.
- The news says when you acknowledged an announcement that asked, in place of the button.

## 0.1.1

Rebuilt with Ferrum-Frontend bb60115: a record page reads an empty number or check as the server does, so an action whose condition needs one shows, and a workspace's settings open as their one record.

## 0.1.0

The company intranet (Ferrum's FERRUM-ECOSYSTEM-001).

- **Spaces** for teams and topics:
  - who keeps each one and who is in it;
  - its pages, a Drive folder written in Docs;
  - its links, to the web or this workspace's pages. A link that would run a script, or reach another site through `//`, is refused.
  - A members-only space is seen by its owner and members alone.
- **Announcements**, written as drafts and published by an Intranet Editor:
  - to everyone, or in a members-only space to its owner and members as they are then, who alone may read it;
  - those it goes to are told in the app;
  - published, an announcement stays as it was posted, and only whether it is pinned and when it expires change;
  - one that asks is acknowledged by each reader once, with when;
  - archived by hand, or the day after it expires.
- **Its own interface at /intranet/:** the news meant for the reader, pinned first, with a button for those that ask to be acknowledged; and the spaces they may see, with their pages and links.
