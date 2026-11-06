import { describe, expect, it } from "vitest";
import { acknowledgedAt, feed, linkOk, owes, type Row } from "./feed";

describe("the intranet's home", () => {
  it("shows pinned news first, then the newest", () => {
    const rows: Row[] = [
      { id: "a", published_at: "2026-11-01T09:00:00Z" },
      { id: "b", published_at: "2026-11-03T09:00:00Z" },
      { id: "c", published_at: "2026-10-01T09:00:00Z", pinned: true },
    ];
    expect(feed(rows).map((r) => r.id)).toEqual(["c", "b", "a"]);
  });

  it("asks for an acknowledgement until the reader has given it", () => {
    const a: Row = { id: "a", status: "Published", needs_acknowledgement: true, acknowledgements: [{ user_email: "Ana@acme.test" }] };
    expect([owes(a, "ana@acme.test"), owes(a, "ben@acme.test"), owes({ ...a, needs_acknowledgement: false }, "ben@acme.test"), owes({ ...a, status: "Archived" }, "ben@acme.test")]).toEqual([
      false,
      true,
      false,
      false,
    ]);
  });

  it("says when the reader acknowledged", () => {
    const a: Row = { id: "a", acknowledgements: [{ user_email: "Ana@acme.test", acknowledged_at: "2026-11-06T06:50:00Z" }] };
    expect([acknowledgedAt(a, "ana@acme.test"), acknowledgedAt(a, "ben@acme.test"), acknowledgedAt(a, "")]).toEqual(["2026-11-06T06:50:00Z", undefined, undefined]);
  });

  it("shows links to the web or this workspace, never a script", () => {
    expect(["https://handbook.acme.test", "http://x.test/a", "/docs/f/policies"].every(linkOk)).toBe(true);
    expect(["javascript:alert(1)", "//evil.test", "data:text/html,hi", " https://x", "https://x y", ""].some(linkOk)).toBe(false);
  });
});
