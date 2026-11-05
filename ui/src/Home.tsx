// The intranet's home: the announcements meant for the reader, pinned first, with a button for
// those that ask to be acknowledged; and the spaces they may see, with their pages and links. What
// is shown is what the server lets this person read.

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { Link } from "react-router";
import { api, docPath } from "@ferrum/client";
import { Badge, Button, ErrorState, Markdown, PageLoading, toast, useShellTitle } from "@ferrum/kit";
import { useMe } from "@ferrum/records";
import { feed, linkOk, owes, type Row } from "./feed";

const list = (entity: string, filters: unknown[], fields: string) =>
  api.get<{ rows: Row[] }>(docPath(entity), { filters: JSON.stringify(filters), fields, limit: 200, titles: true }).then((r) => r.rows);

const day = (at: unknown) => (typeof at === "string" ? new Date(at).toLocaleDateString(undefined, { day: "numeric", month: "short", year: "numeric" }) : "");

export function Home() {
  useShellTitle("Intranet");
  const qc = useQueryClient();
  const email = useMe().data?.user.email ?? "";
  const [busy, setBusy] = useState<string | null>(null);
  const q = useQuery({
    queryKey: ["intranet", "home"],
    queryFn: async () => {
      const [news, spaces] = await Promise.all([
        list("Announcement", [["status", "=", "Published"]], "title,body,space,pinned,needs_acknowledgement,published_at,status"),
        list("IntranetSpace", [], "space_name,description,visibility,pages_folder,active"),
      ]);
      // Tables come with a record: the acknowledgements of those that ask, the links of each space.
      const full = (entity: string, rows: Row[]) => Promise.all(rows.map((r) => api.get<Row>(docPath(entity, r.id)).then((d) => ({ ...r, ...d }))));
      const [asking, open] = await Promise.all([full("Announcement", news.filter((n) => n.needs_acknowledgement)), full("IntranetSpace", spaces.filter((s) => s.active !== false))]);
      const byId = new Map(asking.map((a) => [a.id, a]));
      return { news: feed(news.map((n) => byId.get(n.id) ?? n)), spaces: open };
    },
    refetchInterval: 60_000,
  });
  if (q.isPending) return <PageLoading />;
  if (q.isError) return <ErrorState error={q.error} onRetry={() => void q.refetch()} />;

  const acknowledge = async (a: Row) => {
    setBusy(a.id);
    try {
      await api.post(docPath("Announcement", a.id, "actions", "acknowledge"), {});
      toast("Acknowledged.");
      await qc.invalidateQueries({ queryKey: ["intranet", "home"] });
    } catch (e) {
      toast(e instanceof Error ? e.message : "That did not work.", "error");
    } finally {
      setBusy(null);
    }
  };
  const titles = (r: Row) => (r._titles as Record<string, string> | undefined) ?? {};

  return (
    <div className="np-page">
      <header className="row" style={{ justifyContent: "space-between", flexWrap: "wrap", gap: 8 }}>
        <h2>Intranet</h2>
        <Link to="/e/Announcement">All announcements</Link>
      </header>
      <div className="np-columns in-columns">
        <section className="stack" aria-labelledby="in-news">
          <h3 id="in-news">News</h3>
          {q.data.news.length === 0 && <p className="faint">Nothing new.</p>}
          {q.data.news.map((a) => (
            <article key={a.id} className="card card-pad stack" aria-label={String(a.title)}>
              <header className="row" style={{ justifyContent: "space-between", flexWrap: "wrap", gap: 8 }}>
                <h4 className="font-semibold">{String(a.title)}</h4>
                <span className="row" style={{ gap: 6 }}>
                  {a.pinned ? <Badge tone="blue">Pinned</Badge> : null}
                  {a.space ? <Badge tone="neutral">{titles(a).space ?? "A space"}</Badge> : null}
                </span>
              </header>
              <p className="faint">{day(a.published_at)}</p>
              <Markdown source={String(a.body ?? "")} />
              {owes(a, email) && (
                <div>
                  <Button size="sm" disabled={busy === a.id} onClick={() => void acknowledge(a)}>
                    Acknowledge
                  </Button>
                </div>
              )}
            </article>
          ))}
        </section>
        <section className="stack" aria-labelledby="in-spaces">
          <h3 id="in-spaces">Spaces</h3>
          {q.data.spaces.length === 0 && <p className="faint">No spaces yet.</p>}
          {q.data.spaces.map((s) => (
            <section key={s.id} className="card card-pad stack" aria-label={String(s.space_name)}>
              <h4 className="font-semibold">
                <Link to={`/e/IntranetSpace/${s.id}`}>{String(s.space_name)}</Link> {s.visibility === "Members" ? <Badge tone="neutral">Members</Badge> : null}
              </h4>
              {s.description ? <p>{String(s.description)}</p> : null}
              <ul className="stack">
                {typeof s.pages_folder === "string" && s.pages_folder && (
                  <li>
                    <a href={`/drive/${encodeURIComponent(s.pages_folder)}`}>Pages</a>
                  </li>
                )}
                {(Array.isArray(s.links) ? (s.links as { label?: string; url?: string }[]) : [])
                  .filter((l) => linkOk(l.url ?? ""))
                  .map((l) => (
                    <li key={l.url}>
                      <a href={l.url} rel="noopener noreferrer" target={l.url?.startsWith("/") ? undefined : "_blank"}>
                        {l.label}
                      </a>
                    </li>
                  ))}
              </ul>
            </section>
          ))}
        </section>
      </div>
    </div>
  );
}
