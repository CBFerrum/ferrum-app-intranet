// What the home page decides: the order announcements are read in, whether one still waits for the
// reader's acknowledgement, and which links may be shown (the same rule the server keeps).

export interface Row {
  id: string;
  [k: string]: unknown;
}

/** Pinned first, then the newest. */
export function feed(list: Row[]): Row[] {
  return [...list].sort((a, b) => Number(!!b.pinned) - Number(!!a.pinned) || String(b.published_at ?? "").localeCompare(String(a.published_at ?? "")));
}

/** Whether `email` is still to acknowledge an announcement that asks. */
export function owes(a: Row, email: string): boolean {
  if (!a.needs_acknowledgement || a.status !== "Published" || !email) return false;
  const acks = Array.isArray(a.acknowledgements) ? (a.acknowledgements as { user_email?: string }[]) : [];
  return !acks.some((r) => r.user_email?.toLowerCase() === email.toLowerCase());
}

/** A web address or a page of this workspace, never a script or another site by `//`. */
export function linkOk(url: string): boolean {
  return !!url && !/[\s\p{Cc}]/u.test(url) && (url.startsWith("https://") || url.startsWith("http://") || (url.startsWith("/") && !url.startsWith("//")));
}
