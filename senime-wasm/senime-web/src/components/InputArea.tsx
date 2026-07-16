import { useCallback } from "react";
import type { CaretPosition, CandidateItem } from "../hooks/useIme.ts";

interface Props {
  candidates: CandidateItem[];
  imeReady: boolean;
  editorRef: React.RefObject<HTMLDivElement | null>;
  onKeyDown: (e: React.KeyboardEvent<HTMLDivElement>) => void;
  caretPos: CaretPosition;
}

/** 将光标放到编辑器文本末尾 */
function placeCaretAtEnd(editor: HTMLElement) {
  const sel = window.getSelection();
  if (!sel) return;
  const range = document.createRange();
  const lastChild = editor.lastChild;
  if (lastChild) {
    if (lastChild.nodeType === Node.TEXT_NODE) {
      range.setStart(lastChild, lastChild.textContent?.length ?? 0);
    } else {
      range.setStartAfter(lastChild);
    }
  } else {
    range.setStart(editor, 0);
  }
  range.collapse(true);
  sel.removeAllRanges();
  sel.addRange(range);
}

export function InputArea({ candidates, imeReady, editorRef, onKeyDown, caretPos }: Props) {
  // 聚焦时将光标放到文本末尾
  const handleFocus = useCallback(() => {
    const editor = editorRef.current;
    if (!editor) return;
    requestAnimationFrame(() => placeCaretAtEnd(editor));
  }, [editorRef]);

  // 粘贴时只保留纯文本
  const handlePaste = useCallback((e: React.ClipboardEvent) => {
    e.preventDefault();
    const text = e.clipboardData.getData("text/plain");
    if (text) {
      const sel = window.getSelection();
      if (sel && sel.rangeCount) {
        const range = sel.getRangeAt(0);
        range.deleteContents();
        range.insertNode(document.createTextNode(text));
        range.collapse(false);
        sel.removeAllRanges();
        sel.addRange(range);
      }
    }
  }, []);

  return (
    <section className="input-area">
      {/* 编辑器：包含已提交文本 + 内联 preedit span（由 useIme 管理 DOM） */}
      <div
        ref={editorRef}
        className="ime-editor"
        contentEditable="true"
        suppressContentEditableWarning
        onKeyDown={onKeyDown}
        onPaste={handlePaste}
        onFocus={handleFocus}
        data-placeholder={imeReady ? "在此输入编码..." : "请先加载码表..."}
      />

      {/* 候选弹出框：定位在 preedit span 之后的光标位置 */}
      {candidates.length > 0 && (
        <div
          className={`ime-popup${caretPos.showAbove ? " ime-popup-above" : ""}`}
          style={{
            top: caretPos.top,
            left: caretPos.left,
          }}
        >
          <div className="popup-candidates">
            {candidates.map((c, i) => (
              <span
                key={i}
                className={`candidate ${i === 0 ? "candidate-primary" : ""}`}
              >
                <span className="candidate-key">{c.selectKey}</span>
                <span className="candidate-text">{c.text}</span>
                {c.code.startsWith(c.origin) && c.code.length > c.origin.length && (
                  <span className="candidate-hint">
                    {c.code.slice(c.origin.length)}
                  </span>
                )}
              </span>
            ))}
          </div>
        </div>
      )}
    </section>
  );
}
