import { useState, useCallback, useRef } from "react";
import { completion } from "senime-wasm";

export interface CandidateItem {
  text: string;
  order: number;
  selectKey: string;
  code: string;
  origin: string;
}

type Segment = { text: string; origin: string; tagName: string };

export interface PreeditState {
  /** 引擎解析后的文本（高亮显示） */
  text: string;
  /** 剩余未确认的原始编码（用于显示） */
  origin: string;
  /** 最后一段的标签名 */
  tag: string;
  candidates: CandidateItem[];
}

/** 光标像素位置（相对于编辑器父容器 .input-area） */
export interface CaretPosition {
  top: number;
  left: number;
  showAbove: boolean;
}

// ═══════════════════════════════════════════════════════════════
// 回退剪贴板
// ═══════════════════════════════════════════════════════════════

function fallbackCopy(text: string) {
  const el = document.createElement("textarea");
  el.value = text;
  el.style.cssText = "position:fixed;left:-9999px";
  document.body.appendChild(el);
  el.select();
  document.execCommand("copy");
  document.body.removeChild(el);
}

// ═══════════════════════════════════════════════════════════════
// Hook
// ═══════════════════════════════════════════════════════════════

const IME_PREEDIT_SPAN_ID = "ime_preedit_span";
export function useIme(
  imeReady: boolean,
  editorRef: React.RefObject<HTMLElement | null>,
) {
  const [candidates, setCandidates] = useState<CandidateItem[]>([]);
  const [caretPos, setCaretPos] = useState<CaretPosition>({
    top: 0, left: 0, showAbove: false,
  });

  const inputRef = useRef("");

  function createPreeditSpan(seg: Segment): HTMLSpanElement {
    const span = document.createElement("span");
    span.id = IME_PREEDIT_SPAN_ID;
    span.className = "preedit-container";
    // span.setAttribute("contenteditable", "false");

    // 内部结构: <span class="preedit-text">text</span> <span class="preedit-code">origin</span>
    if (seg.text) {
      const textSpan = document.createElement("span");
      textSpan.className = "preedit-text";
      textSpan.textContent = seg.text;
      span.appendChild(textSpan);
    }
    if (seg.tagName === "Code") {
      const codeSpan = document.createElement("span");
      codeSpan.className = "preedit-code";
      codeSpan.textContent = seg.origin;
      span.appendChild(codeSpan);
    }
    return span;
  }

  // ── 核心：运行 completion，管理 preedit span ──
  const runCompletion = useCallback(
    () => {
      const editor = editorRef.current;
      if (!editor) return;
      // 1. 删除旧的 preedit span
      editor.querySelector("#" + IME_PREEDIT_SPAN_ID)?.remove();
      if (!inputRef.current) {
        setCandidates([]);
        return;
      };
      const sel = window.getSelection();
      if (!sel) return;
      let range = sel.getRangeAt(0);
      try {
        const result = completion(inputRef.current);
        const segCount = result.segment_count;
        const pending = result.pending;

        const segments: Segment[] = [];
        for (let i = 0; i < segCount; i++) {
          const seg = result.segment(i);
          segments.push({ text: seg.text, origin: seg.origin, tagName: seg.tag_name });
          seg.free();
        }

        console.log("completion pending: ", result.pending, " segments:", segments);
        const cands: CandidateItem[] = [];
        if (result.has_candidates) {
          for (let i = 0; i < result.candidate_count; i++) {
            const c = result.candidate(i);
            cands.push({
              text: c.text,
              order: c.order,
              selectKey: c.select_key,
              code: c.code,
              origin: c.origin,
            });
            c.free();
          }
        }

        let pre_text = "";
        for (let i = 0; i < segments.length - 1; i++) {
          pre_text += segments[i].text
        }
        if (pre_text) {
          range.insertNode(document.createTextNode(pre_text));
          range.collapse();
        }
        const lastSeg = segments[segments.length - 1];

        // 无分段：整段提交原始输入
        if (!lastSeg) {
          inputRef.current = "";
          return;
        }
        if (pending) {
          let preeditSpan = createPreeditSpan(lastSeg);
          range.insertNode(preeditSpan);
          range.collapse(true);
          preeditSpan.scrollIntoView()
          // let rect = preeditSpan.getBoundingClientRect();
          let rect = range.getBoundingClientRect();
          setCaretPos({ left: rect.left, top: rect.top + rect.height + 2, showAbove: false });
          setCandidates(cands);
          inputRef.current = lastSeg.origin;
        } else {
          inputRef.current = "";
          range.insertNode(document.createTextNode(lastSeg.text));
          range.collapse(false);
          if (range.commonAncestorContainer.nodeType == Node.TEXT_NODE) {
            range.commonAncestorContainer.parentElement?.scrollIntoView();
          } else {
            (range.commonAncestorContainer as HTMLElement).scrollIntoView();
          };
          setCandidates([]);
        }
      } catch (err) {
        console.error(err);
      }

    },
    [editorRef],
  );

  // ── 键盘事件处理 ──
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLElement>) => {
      if (!imeReady) return;
      const editor = editorRef.current;
      if (!editor) return;

      // Ctrl+Shift+X: 复制并清空
      if (e.key === "X" && e.ctrlKey && e.shiftKey) {
        e.preventDefault();
        copyAndClearInner(editor);
        return;
      }

      // Ctrl+C: 有 preedit 时拦截
      if (e.key === "c" && e.ctrlKey && window.getSelection()?.type == "Caret") {
        e.preventDefault();
        copyTextInner(editor);
        return;
      }

      // Ctrl+X: 清空
      if (e.key === "x" && e.ctrlKey) {
        e.preventDefault();
        clearInner(editor);
        return;
      }

      if (e.ctrlKey || e.metaKey) return;

      // ── 有 preedit 时的处理 ──
      if (inputRef.current) {
        if (e.key === "Enter") {
          e.preventDefault();
          inputRef.current += " ";
          console.log("enter key input: ", inputRef);
          runCompletion();
          return;
        }

        if (e.key === "Backspace") {
          e.preventDefault();
          inputRef.current = inputRef.current.slice(0, -1);
          runCompletion();
          return;
        }

        if (e.key === "PageUp" || e.key === "PageDown") {
          e.preventDefault();
          const ch = e.key === "PageUp" ? "\u21DE" : "\u21DF";
          inputRef.current += ch;
          runCompletion();
          return;
        }

        if (
          e.key === "ArrowLeft" || e.key === "ArrowRight" ||
          e.key === "ArrowUp" || e.key === "ArrowDown" ||
          e.key === "Home" || e.key === "End"
        ) {
          inputRef.current = "";
          runCompletion();
          return;
        }
      }

      // ── 无 preedit 时：可打印字符开始新的编码 ──
      if (e.key.length === 1) {
        e.preventDefault();
        if (!inputRef.current) {
          inputRef.current = e.key;
        } else {
          inputRef.current += e.key;
        }
        runCompletion();
        return;
      }
    },
    [imeReady, editorRef, runCompletion],
  );

  // ── 工具函数 ──
  function clearInner(editor: HTMLElement | null) {
    if (!editor) return;
    inputRef.current = "";
    setCandidates([]);
    editor.textContent = "";
  }

  function copyTextInner(editor: HTMLElement | null) {
    const text = editor?.textContent ?? "";
    if (!text) return;
    if (navigator.clipboard?.writeText) {
      navigator.clipboard.writeText(text).catch(() => fallbackCopy(text));
    } else {
      fallbackCopy(text);
    }
  }

  function copyAndClearInner(editor: HTMLElement | null) {
    if (!editor) return;
    copyTextInner(editor);
    clearInner(editor);
  }

  const clear = useCallback(() => clearInner(editorRef.current), [editorRef]);
  const copyText = useCallback(() => copyTextInner(editorRef.current), [editorRef]);
  const copyAndClear = useCallback(() => copyAndClearInner(editorRef.current), [editorRef]);

  return {
    candidates,
    caretPos,
    handleKeyDown,
    clear,
    copyText,
    copyAndClear,
  };
}
