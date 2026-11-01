// @vitest-environment jsdom
import { act } from "react";
import { createRoot } from "react-dom/client";
import { expect, it, vi } from "vitest";
import { CatalogImport } from "./CatalogImport";
const api = vi.hoisted(() => ({ importCatalog: vi.fn() }));
vi.mock("../lib/tauri", () => api);
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: async () => "/catalog" }));
vi.mock("react-i18next", () => ({ useTranslation: () => ({ t: (key: string) => key }) }));
it("selects only available packages and sends exactly the checked sources", async () => {
  vi.stubGlobal("IS_REACT_ACT_ENVIRONMENT", true);
  const entries = [
    { source: "/catalog/a", name: "a", category: "devops", existing_id: null, conflict: null },
    { source: "/catalog/b", name: "b", category: null, existing_id: null, conflict: null },
    { source: "/catalog/c", name: "c", category: null, existing_id: null, conflict: "duplicate" },
  ];
  api.importCatalog.mockResolvedValue(entries);
  const host = document.createElement("div"); document.body.append(host);
  const root = createRoot(host);
  const refreshed = vi.fn().mockResolvedValue(undefined);
  try {
    await act(async () => root.render(<CatalogImport onImported={refreshed} />));
    const button = (text: string) => [...host.querySelectorAll("button")].find(b => b.textContent === text)!;
    await act(async () => button("catalog.browse").click());
    await act(async () => button("catalog.preview").click());
    const boxes = [...host.querySelectorAll<HTMLInputElement>('input[type="checkbox"]')];
    await act(async () => boxes[0].click());
    expect(boxes.map(b => b.checked)).toEqual([true, true, true, false]);
    await act(async () => boxes[2].click());
    await act(async () => button("catalog.import").click());
    expect(api.importCatalog).toHaveBeenCalledWith("/catalog", null, [], ["/catalog/a"]);
    expect(refreshed).toHaveBeenCalledOnce();
  } finally { await act(async () => root.unmount()); host.remove(); vi.unstubAllGlobals(); }
});
