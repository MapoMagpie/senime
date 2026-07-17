import { useRef, useState, useEffect, useCallback } from "react";
import type { DictConfig, DictStatus } from "../hooks/useDictLoader";

const KEY_PRESETS: { label: string; keys: string[] }[] = [
  { label: "1-9", keys: ["1", "2", "3", "4", "5", "6", "7", "8", "9"] },
  { label: ";'+数字", keys: [";", "'", "3", "4", "5", "6", "7", "8", "9"] },
  { label: "UIOP+数字", keys: ["U", "I", "O", "P", "5", "6", "7", "8", "9"] },
];

interface Props {
  status: DictStatus;
  setStatus: (status: DictStatus) => void;
  config: DictConfig;
  setConfig: (config: DictConfig) => void;
  onConfirm: (resolved: () => void) => void;
  onCollapse?: () => void;
}

/** 根据 DictStatus.state 映射到对应的 CSS 类名 */
function statusToClass(state: DictStatus["state"]): string {
  switch (state) {
    case "ready": return "status-ready";
    case "error": return "status-error";
    case "none": return "status-none";
    default: return "status-loading"; // loading, file_downloading, file_selected
  }
}

/** 解析 index.txt 的一行，格式：码表名|文件名。 */
function parsePresetLine(parent: string, line: string): { label: string; url: string } | null {
  const trimmed = line.trim();
  if (!trimmed) return null;
  const idx = trimmed.indexOf("|");
  if (idx === -1) return null;
  const label = trimmed.slice(0, idx).trim();
  const addr = trimmed.slice(idx + 1).trim();
  if (!label || !addr) return null;
  const url = parent + addr;
  return { label, url };
}

export function DictLoader({ status, setStatus, config, setConfig, onConfirm, onCollapse }: Props) {
  const fileRef = useRef<HTMLInputElement>(null);
  const menuRef = useRef<HTMLDivElement>(null);
  const [expanded, setExpanded] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [presets, setPresets] = useState<{ label: string; url: string }[] | null>(null);

  const contentVisible = status.state !== "ready" || expanded;
  const animated = status.state === "ready"; // 码表加载后启用过渡动画，首次渲染不播动画

  // 点击外部关闭悬浮菜单
  useEffect(() => {
    if (!menuOpen) return;
    const handleClick = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setMenuOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [menuOpen]);

  const handlePresetBtn = useCallback(async () => {
    // 已缓存则直接切换菜单，否则请求 index.txt
    if (presets !== null) {
      setMenuOpen((v) => !v);
      return;
    }
    setMenuOpen(true);
    try {
      const path = import.meta.env.BASE_URL + "assets/tables/";
      const resp = await fetch(path + "index.txt");
      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const text = await resp.text();
      const list = text
        .split("\n")
        .map(line => parsePresetLine(path, line))
        .filter((p): p is { label: string; url: string } => p !== null);
      setPresets(list);
    } catch {
      setPresets([]);
      setMenuOpen(false);
    }
  }, [presets]);

  const handlePresetSelect = (url: string, label: string) => {
    setMenuOpen(false);
    setConfig({ ...config, file: url, dict_name: label });
    setStatus({ state: "file_selected", message: `已选择${label}` });
    // 选择了预设则清除文件选择
    if (fileRef.current) fileRef.current.value = "";
  };

  const handleFileChange = () => {
    const file = fileRef.current?.files?.[0];
    let dict_name: string | undefined;
    if (file) {
      dict_name = file.name.split(".").shift();
    }
    setConfig({ ...config, file, dict_name });
    setStatus({ state: "file_selected", message: `已选择${dict_name}` });
  };

  const handleSlotChange = (index: number, value: string) => {
    const ch = value.slice(-1);
    const selection_keys = [...config.selection_keys];
    selection_keys[index] = ch;
    setConfig({ ...config, selection_keys })
  };

  return (
    <section className="dict-loader">
      {/* 收起栏：码表加载后显示，点击展开配置面板 */}
      {status.state === "ready" && (
        <div className="dict-collapsed-bar" onClick={() => { let v = !expanded; setExpanded(v); !v && onCollapse?.(); }}>
          <span className="status-ready">已加载{config.dict_name} ✓</span>
          <span className="dict-expand-hint">{expanded ? "收起配置" : "展开配置"}</span>
        </div>
      )}

      {/* 可展开/收起的内容区域：通过 max-height + opacity 过渡 */}
      <div className={`dict-content${animated ? ' dict-content-animated' : ''}${contentVisible ? ' dict-content-open' : ''}`}>
        <p className="dict-desc">选择你的码表，自定义候选键(可选)，然后点击确认加载。</p>

        <div className="selection-keys-section">
          <h2>码表加载</h2>
          <div className="dict-controls">
            <input ref={fileRef} type="file" accept=".txt" onChange={handleFileChange} />
            <div className="preset-dropdown" ref={menuRef}>
              <button
                className="preset-toggle-btn"
                disabled={status.state === "loading"}
                onClick={handlePresetBtn}
              >
                预设码表 ▾
              </button>
              {typeof config.file === "string" && (
                <span className="preset-selected">已选择:{config.dict_name}</span>
              )}
              {menuOpen && (
                <div className="preset-menu">
                  {presets === null ? (
                    <span className="preset-menu-loading">加载中...</span>
                  ) : presets.length === 0 ? (
                    <span className="preset-menu-empty">暂无预设</span>
                  ) : (
                    presets.map((p) => (
                      <button
                        key={p.url}
                        className="preset-menu-item"
                        onClick={() => handlePresetSelect(p.url, p.label)}
                      >
                        {p.label}
                      </button>
                    ))
                  )}
                </div>
              )}
            </div>
          </div>
        </div>

        <div className="selection-keys-section">
          <h2>候选键配置</h2>
          <div className="presets">
            {KEY_PRESETS.map((p) => (
              <button
                key={p.label}
                className="preset-btn"
                onClick={() => setConfig({ ...config, selection_keys: p.keys })}
              >
                {p.label}
              </button>
            ))}
          </div>
          <div className="custom-keys">
            {config.selection_keys.map((k, i) => (
              <input
                key={i}
                type="text"
                className="key-slot"
                maxLength={2}
                value={k}
                onChange={(e) => handleSlotChange(i, e.target.value)}
              />
            ))}
          </div>
        </div>

        <div className="selection-keys-section">
          <h2>候选数量</h2>
          <div className="page-count-row">
            <input
              type="range"
              className="page-count-slider"
              min={1}
              max={9}
              value={config.page_count}
              onChange={(e) => setConfig({ ...config, page_count: Number(e.target.value) })}
            />
            <span className="page-count-value">{config.page_count}</span>
          </div>
        </div>

        <div className="selection-keys-section">
          <div className="dict-bottom">
            <button onClick={() => onConfirm(() => setExpanded(false))} disabled={status.state === "loading" || status.state === "file_downloading" || !config.file}>
              确认加载
            </button>
            <div className="dict-status">
              <span className={statusToClass(status.state)}>{status.message}</span>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
