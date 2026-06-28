import { useRef, useCallback, useEffect } from "react";
import type { ImeState } from "../hooks/useIme.ts";
import type { CursorPos } from "../hooks/useCursorPos.ts";

interface Props {
  state: ImeState;
  imeReady: boolean;
  textareaRef: React.RefObject<HTMLTextAreaElement | null>;
  onKeyDown: (e: React.KeyboardEvent<HTMLTextAreaElement>) => void;
  cursorPos: CursorPos;
}

export function InputArea({ state, imeReady, textareaRef, onKeyDown, cursorPos }: Props) {
  const displayRef = useRef<HTMLDivElement>(null);

  // 同步 overlay → textarea 滚动
  const handleDisplayScroll = useCallback(() => {
    if (textareaRef.current && displayRef.current) {
      textareaRef.current.scrollTop = displayRef.current.scrollTop;
    }
  }, [textareaRef]);

  // 同步 textarea → overlay 滚动（方向键、Page Up/Down 等触发）
  useEffect(() => {
    const ta = textareaRef.current;
    const display = displayRef.current;
    if (!ta || !display) return;
    const sync = () => { display.scrollTop = ta.scrollTop; };
    ta.addEventListener("scroll", sync);
    return () => ta.removeEventListener("scroll", sync);
  }, [textareaRef]);

  const hasPreedit = !!(state.preeditText || state.preedit);
  const hasCandidates = state.candidates.length > 0;
  const ta = textareaRef.current;
  const sel = ta?.selectionStart ?? 0;
  const value = ta?.value ?? "";
  const hasContent = value.length > 0 || hasPreedit;
  const showPlaceholder = !hasContent && !hasPreedit;

  return (
    <section className="input-area">
      <textarea
        ref={textareaRef}
        className="ime-textarea"
        onKeyDown={onKeyDown}
        autoComplete="off"
        autoCorrect="off"
        autoCapitalize="off"
        spellCheck={false}
      />
      <div
        ref={displayRef}
        className="ime-display"
        onScroll={handleDisplayScroll}
        aria-hidden="true"
      >
        {showPlaceholder && (
          <span className="ime-placeholder">
            {imeReady ? "在此输入编码..." : "请先加载码表..."}
          </span>
        )}
        {hasContent && (
          <>
            {value.substring(0, sel)}
            {hasPreedit && (
              <>
                {state.preeditText && (
                  <span className="preedit-text">{state.preeditText}</span>
                )}
                {state.preedit && (
                  <span className="preedit-code">{state.preedit}</span>
                )}
              </>
            )}
            {value.substring(sel)}
          </>
        )}
      </div>
      {hasCandidates && (
        <div
          className={`ime-popup${cursorPos.showAbove ? " ime-popup-above" : ""}`}
          style={{
            top: cursorPos.top,
            left: cursorPos.left,
          }}
        >
          <div className="popup-candidates">
            {state.candidates.map((c, i) => (
              <span
                key={i}
                className={`candidate ${i === 0 ? "candidate-primary" : ""}`}
              >
                <span className="candidate-key">{c.selectKey}</span>
                <span className="candidate-text">{c.text}</span>
              </span>
            ))}
          </div>
        </div>
      )}
    </section>
  );
}
