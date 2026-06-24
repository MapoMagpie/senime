import { useRef, useState } from "react";
import type { DictStatus } from "../hooks/useDictLoader";

const PRESETS: { label: string; keys: string[] }[] = [
  { label: "1-9", keys: ["1", "2", "3", "4", "5", "6", "7", "8", "9"] },
  { label: ";'+数字", keys: [";", "'", "3", "4", "5", "6", "7", "8", "9"] },
  { label: "UIOP+数字", keys: ["U", "I", "O", "P", "5", "6", "7", "8", "9"] },
];

interface Props {
  status: DictStatus;
  imeReady: boolean;
  selectionKeys: string[];
  onSelectionKeysChange: (keys: string[]) => void;
  onUpload: (file: File, keys: string[]) => void;
}

export function DictLoader({ status, imeReady, selectionKeys, onSelectionKeysChange, onUpload }: Props) {
  const fileRef = useRef<HTMLInputElement>(null);
  const [localError, setLocalError] = useState<string | null>(null);
  const [expanded, setExpanded] = useState(false);

  const collapsed = imeReady && !expanded;

  const handleConfirm = () => {
    setLocalError(null);
    const files = fileRef.current?.files;
    if (!files || files.length === 0) {
      setLocalError("请先选择码表文件");
      return;
    }
    const file = files[0];
    if (file.size === 0) {
      setLocalError("码表文件为空");
      return;
    }
    onUpload(file, selectionKeys);
  };

  const handleSlotChange = (index: number, value: string) => {
    const ch = value.slice(-1); // 只取最后一个字符
    const next = [...selectionKeys];
    next[index] = ch;
    onSelectionKeysChange(next);
  };

  if (collapsed) {
    return (
      <section className="dict-loader dict-collapsed" onClick={() => setExpanded(true)}>
        <span className="status-ok">✓ 码表已加载</span>
        <span className="dict-expand-hint">点击重新配置</span>
      </section>
    );
  }

  return (
    <section className="dict-loader">
      <p className="dict-desc">选择你的码表，自定义候选键(可选)，然后点击确认加载。</p>

      <div className="selection-keys-section">
        <h2>码表加载</h2>
        <div className="dict-controls">
          <input ref={fileRef} type="file" accept=".txt" />
        </div>
      </div>

      <div className="selection-keys-section">
        <h2>候选键配置</h2>
        <div className="presets">
          {PRESETS.map((p) => (
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
        <div className="dict-bottom">
          <button onClick={handleConfirm} disabled={status.state === "loading"}>
            确认加载
          </button>
          {imeReady && (
            <button onClick={() => setExpanded(false)}>
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
    </section>
  );
}
