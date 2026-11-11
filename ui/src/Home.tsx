// The intranet's home: the announcements meant for the reader, pinned first, with a button for
// those that ask to be acknowledged; and the spaces they may see, with their pages and links. What
// is shown is what the server lets this person read.

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { api, docPath, formatMoment, t, isolate } from "@ferrum/client";
import { Badge, Button, ButtonLink, HomeSection, HomeSkeleton, LoadFailed, Markdown, PageHead, PageStack, Queue, QueueGrid, QueueItem, toast, useShellTitle } from "@ferrum/kit";
import { useMe } from "@ferrum/records";
import { acknowledgedAt, feed, linkOk, owes, type Row } from "./feed";

const list = (entity: string, filters: unknown[], fields: string) =>
  api.get<{ rows: Row[] }>(docPath(entity), { filters: JSON.stringify(filters), fields, limit: 200, titles: true }).then((r) => r.rows);

/** A moment's day on the workspace's clock, as members read times. */
const day = (at: unknown) => (typeof at === "string" ? formatMoment(at, { day: "numeric", month: "short", year: "numeric" }) : "");

export function Home() {
  useShellTitle(t("Intranet"));
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
  const head = (
    <PageHead
      title={t("Intranet")}
      actions={<ButtonLink to="/e/Announcement">{t("All announcements")}</ButtonLink>}
    />
  );
  if (q.isPending) return <PageStack>{head}<HomeSkeleton queues={2} /></PageStack>;
  if (q.isError) return <PageStack>{head}<LoadFailed error={q.error} onRetry={() => void q.refetch()} /></PageStack>;

  const acknowledge = async (a: Row) => {
    setBusy(a.id);
    try {
      await api.post(docPath("Announcement", a.id, "actions", "acknowledge"), {});
      toast(t("Acknowledged."));
      await qc.invalidateQueries({ queryKey: ["intranet", "home"] });
    } catch (e) {
      toast(e instanceof Error ? e.message : t("That did not work."), "error");
    } finally {
      setBusy(null);
    }
  };
  const titles = (r: Row) => (r._titles as Record<string, string> | undefined) ?? {};

  return (
    <PageStack>
      {head}
      <div className="grid gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(0,24rem)] lg:items-start">
        <HomeSection title={t("News")} empty={t("Nothing new.")}>
          <div className="flex flex-col gap-3">
          {q.data.news.map((a) => (
            <article key={a.id} className="card card-pad grid gap-2" aria-label={String(a.title)}>
              <header className="flex flex-wrap items-start justify-between gap-x-3 gap-y-1">
                <h3 className="text-base font-semibold">{isolate(String(a.title))}</h3>
                <span className="flex flex-wrap items-center gap-1.5">
                  {a.pinned ? <Badge tone="blue">{t("Pinned")}</Badge> : null}
                  {a.space ? <Badge tone="neutral">{titles(a).space ?? t("A space")}</Badge> : null}
                </span>
              </header>
              <p className="text-xs text-muted-foreground">{isolate(day(a.published_at))}</p>
              <Markdown source={String(a.body ?? "")} />
              {owes(a, email) ? (
                <div>
                  <Button size="sm" disabled={busy === a.id} onClick={() => void acknowledge(a)}>
                    {t("Acknowledge")}
                  </Button>
                </div>
              ) : acknowledgedAt(a, email) ? (
                <p className="text-xs text-muted-foreground">{t("You acknowledged it on")} {isolate(day(acknowledgedAt(a, email)))}.</p>
              ) : null}
            </article>
          ))}
          </div>
        </HomeSection>
        <QueueGrid columns={1}>
          <Queue title={t("Spaces")} count={q.data.spaces.length} empty={t("No spaces yet.")}>
            {q.data.spaces.map((s) => {
              const links = (Array.isArray(s.links) ? (s.links as { label?: string; url?: string }[]) : []).filter((l) => linkOk(l.url ?? ""));
              const pages = typeof s.pages_folder === "string" && s.pages_folder;
              return (
                <QueueItem
                  key={s.id}
                  to={`/e/IntranetSpace/${s.id}`}
                  title={isolate(String(s.space_name))}
                  meta={s.description ? String(s.description) : undefined}
                  trailing={s.visibility === "Members" ? <Badge tone="neutral">{t("Members")}</Badge> : undefined}
                  action={
                    pages || links.length > 0 ? (
                      <span className="flex flex-wrap justify-end gap-1">
                        {pages && <ButtonLink href={`/drive/${encodeURIComponent(String(s.pages_folder))}`}>{t("Pages")}</ButtonLink>}
                        {links.map((l) => (
                          <ButtonLink key={l.url} href={l.url ?? ""}>
                            {isolate(l.label ?? "")}
                          </ButtonLink>
                        ))}
                      </span>
                    ) : undefined
                  }
                />
              );
            })}
          </Queue>
        </QueueGrid>
      </div>
    </PageStack>
  );
}
