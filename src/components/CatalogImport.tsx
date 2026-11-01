import { useState } from "react";
import { useTranslation } from "react-i18next";
import { open } from "@tauri-apps/plugin-dialog";
import { importCatalog, type CatalogEntry } from "../lib/tauri";
import { getErrorMessage } from "../lib/error";

export function CatalogImport({ onImported }: { onImported: () => Promise<void> }) {
  const { t } = useTranslation();
  const [root, setRoot] = useState("");
  const [category, setCategory] = useState("");
  const [excluded, setExcluded] = useState("");
  const [entries, setEntries] = useState<CatalogEntry[]>([]);
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [done, setDone] = useState(false);
  const reset = () => { setEntries([]); setSelected(new Set()); setDone(false); setError(""); };
  const run = async (apply: boolean) => {
    setBusy(true); setError(""); setDone(false);
    try {
      const exclusions = excluded.split(",").map(s => s.trim()).filter(Boolean);
      if (apply) {
        await importCatalog(root, category.trim() || null, exclusions, [...selected]);
        await onImported();
        setDone(true);
      }
      setEntries(await importCatalog(root, category.trim() || null, exclusions));
      setSelected(new Set());
    } catch (e) { setError(getErrorMessage(e, t("catalog.failed"))); }
    finally { setBusy(false); }
  };
  return <section className="app-panel space-y-3 p-4">
    <h2 className="text-[14px] font-semibold text-secondary">{t("catalog.title")}</h2>
    <p className="text-[13px] text-muted">{t("catalog.description")}</p>
    <fieldset disabled={busy} className="space-y-3 min-w-0">
      <label className="block text-[13px]">{t("catalog.root")}
        <input className="app-input w-full" value={root} onChange={e => { setRoot(e.target.value); reset(); }} />
      </label>
      <button className="app-button-secondary" onClick={async () => {
        try { const path = await open({ directory: true, multiple: false }); if (typeof path === "string") { setRoot(path); reset(); } }
        catch (e) { setError(getErrorMessage(e, t("catalog.failed"))); }
      }}>{t("catalog.browse")}</button>
      <label className="block text-[13px]">{t("catalog.category")}
        <input className="app-input w-full" value={category} onChange={e => { setCategory(e.target.value); reset(); }} />
      </label>
      <label className="block text-[13px]">{t("catalog.exclude")}
        <input className="app-input w-full" value={excluded} onChange={e => { setExcluded(e.target.value); reset(); }} />
      </label>
      <button className="app-button-secondary" disabled={!root.trim()} onClick={() => void run(false)}>{t("catalog.preview")}</button>
      {entries.length > 0 && <div className="max-h-80 overflow-auto space-y-2">
        <label className="flex gap-2 text-[13px]"><input type="checkbox" checked={entries.some(e => !e.conflict && !e.existing_id) && entries.filter(e => !e.conflict && !e.existing_id).every(e => selected.has(e.source))} onChange={e => setSelected(new Set(e.target.checked ? entries.filter(x => !x.conflict && !x.existing_id).map(x => x.source) : []))} />{t("catalog.selectAll")}</label>
        {entries.map(e => <label key={e.source + e.name} className="flex gap-2 border-t border-border-subtle py-2 text-[13px]">
          <input type="checkbox" disabled={!!e.conflict || !!e.existing_id} checked={selected.has(e.source)} onChange={event => setSelected(prev => { const next = new Set(prev); if (event.target.checked) next.add(e.source); else next.delete(e.source); return next; })} />
          <span className="min-w-0 break-words"><strong>{e.name}</strong> · {e.category || t("catalog.untagged")}<span className="block text-muted break-all">{e.source}</span>{e.conflict || (e.existing_id ? t("catalog.existing") : "")}</span>
        </label>)}
      </div>}
      <button className="app-button-primary" disabled={selected.size === 0} onClick={() => void run(true)}>{t("catalog.import", { count: selected.size })}</button>
    </fieldset>
    {busy && <p role="status">{t("catalog.working")}</p>}
    {done && <p role="status">{t("catalog.done")}</p>}
    {error && <p role="alert" className="text-[13px] break-words">{error}</p>}
  </section>;
}
