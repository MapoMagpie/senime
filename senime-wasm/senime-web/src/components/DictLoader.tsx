import { useRef, useState, useEffect, useCallback } from "react";
import type { DictStatus } from "../hooks/useDictLoader";

const KEY_PRESETS: { label: string; keys: string[] }[] = [
  { label: "1-9", keys: ["1", "2", "3", "4", "5", "6", "7", "8", "9"] },
  { label: ";'+数字", keys: [";", "'", "3", "4", "5", "6", "7", "8", "9"] },
  { label: "UIOP+数字", keys: ["U", "I", "O", "P", "5", "6", "7", "8", "9"] },
];

interface Props {
  status: DictStatus;
  imeReady: boolean;
  selectionKeys: string[];
  pageCount: number;
  onSelectionKeysChange: (keys: string[]) => void;
  onPageCountChange: (count: number) => void;
  onUpload: (text: string, keys: string[], count: number) => void;
  onCollapse?: () => void;
}

/** 解析 index.txt 的一行，格式：码表名|文件名。 */
function parsePresetLine(parent: string ,line: string): { label: string; url: string } | null {
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

export function DictLoader({ status, imeReady, selectionKeys, pageCount, onSelectionKeysChange, onPageCountChange, onUpload, onCollapse }: Props) {
  const fileRef = useRef<HTMLInputElement>(null);
  const menuRef = useRef<HTMLDivElement>(null);
  const [localError, setLocalError] = useState<string | null>(null);
  const [expanded, setExpanded] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [presets, setPresets] = useState<{ label: string; url: string }[] | null>(null);
  const [tableUrl, setTableUrl] = useState<string | null>(null);
  const [tableLabel, setTableLabel] = useState<string | null>(null);

  const contentVisible = !imeReady || expanded;
  const animated = imeReady; // 码表加载后启用过渡动画，首次渲染不播动画

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

  const handleConfirm = async () => {
    setLocalError(null);
    const files = fileRef.current?.files;
    if (files && files.length > 0 && files[0].size > 0) {
      // 用户手动上传了文件
      try {
        onUpload(await files[0].text(), selectionKeys, pageCount);
      } catch (e) {
        setLocalError(`读取文件失败: ${e}`);
      }
    } else if (tableUrl) {
      // 用户选择了预设码表
      try {
        const resp = await fetch(tableUrl);
        if (!resp.ok) throw new Error(`HTTP ${resp.status}: ${resp.statusText}`);
        onUpload(await resp.text(), selectionKeys, pageCount);
      } catch (e) {
        setLocalError(`加载预设码表失败: ${e}`);
      }
    } else {
      setLocalError("请先选择码表文件或预设码表");
    }
  };

  const handlePresetBtn = useCallback(async () => {
    setLocalError(null);
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
    setTableUrl(url);
    setTableLabel(label);
    // 选择了预设则清除文件选择
    if (fileRef.current) fileRef.current.value = "";
  };

  const handleFileChange = () => {
    // 用户选择了文件则清除预设选择
    setTableUrl(null);
    setTableLabel(null);
  };

  const handleSlotChange = (index: number, value: string) => {
    const ch = value.slice(-1);
    const next = [...selectionKeys];
    next[index] = ch;
    onSelectionKeysChange(next);
  };

  return (
    <section className="dict-loader">
      {/* 收起栏：码表加载后显示，点击展开配置面板 */}
      {imeReady && (
        <div className="dict-collapsed-bar" onClick={() => setExpanded(true)}>
          <span className="status-ok">✓ 码表已加载</span>
          <span className="dict-expand-hint">展开配置</span>
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
              {tableLabel && (
                <span className="preset-selected">已选择: {tableLabel}</span>
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
                onClick={() => onSelectionKeysChange(p.keys)}
              >
                {p.label}
              </button>
            ))}
          </div>
          <div className="custom-keys">
            {selectionKeys.map((k, i) => (
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
              value={pageCount}
              onChange={(e) => onPageCountChange(Number(e.target.value))}
            />
            <span className="page-count-value">{pageCount}</span>
          </div>
        </div>

        <div className="selection-keys-section">
          <div className="dict-bottom">
            <button onClick={handleConfirm} disabled={status.state === "loading"}>
              确认加载
            </button>
            {imeReady && (
              <button onClick={() => { setExpanded(false); onCollapse?.(); }}>
                收起
              </button>
            )}
            <div className="dict-status">
              {localError && <span className="status-error">✗ {localError}</span>}
              {status.state === "wasm_init" && <span className="status-loading">正在初始化 WASM...</span>}
              {status.state === "loading" && <span className="status-loading">正在加载码表...</span>}
              {status.state === "cached" && <span className="status-ok">✓ {status.message}</span>}
              {status.state === "uploaded" && <span className="status-ok">✓ {status.message}</span>}
              {status.state === "error" && !localError && <span className="status-error">✗ {status.message}</span>}
              {status.state === "idle" && !localError && <span className="status-idle">请选择 .txt 码表文件</span>}
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
