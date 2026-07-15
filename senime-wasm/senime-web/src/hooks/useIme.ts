import { useState, useCallback, useRef } from "react";
import { completion } from "senime-wasm";

export interface CandidateItem {
  text: string;
  order: number;
  selectKey: string;
  /** 该候选项对应的完整编码 */
  code: string;
  /** 用户当前已输入的原始编码 */
  origin: string;
}

export interface ImeState {
  /** 当前正在编码但未确认的文本（类似 fcitx5 的 preedit） */
  preedit: string;
  /** 最后一段的查询结果文本（高亮显示） */
  preeditText: string;
  candidates: CandidateItem[];
  /** 最后一段的标签名 */
  lastTag: string;
}

const EMPTY_STATE: ImeState = {
  preedit: "",
  preeditText: "",
  candidates: [],
  lastTag: "",
};

/** execCommand 回退，用于 navigator.clipboard 不可用时（如 HTTP 环境） */
function fallbackCopy(text: string) {
  const el = document.createElement("textarea");
  el.value = text;
  el.style.cssText = "position:fixed;left:-9999px";
  document.body.appendChild(el);
  el.select();
  document.execCommand("copy");
  document.body.removeChild(el);
}

/**
 * 对 preedit 调用 wasm completion，更新 state。
 * 中间段自动提交到 textarea，最后一段作为 preedit。
 * textarea.setRangeText 不会触发 keydown/input 事件，无循环依赖。
 */
function runCompletion(
  preedit: string,
  textareaRef: React.RefObject<HTMLTextAreaElement | null>,
  setState: React.Dispatch<React.SetStateAction<ImeState>>,
  recalc: () => void,
) {
  if (!preedit) {
    setState((s) => {
      if (s.candidates.length > 0) return { preedit: "", preeditText: "", candidates: [], lastTag: "" };
      return s;
    });
    return;
  }

  const result = completion(preedit);
  const segCount = result.segment_count;

  let pending = result.pending;
  const segments: { text: string; origin: string; tagName: string }[] = [];
  for (let i = 0; i < segCount; i++) {
    const seg = result.segment(i);
    segments.push({ text: seg.text, origin: seg.origin, tagName: seg.tag_name });
    seg.free();
  }

  const cands: CandidateItem[] = [];
  if (result.has_candidates) {
    for (let i = 0; i < result.candidate_count; i++) {
      const c = result.candidate(i);
      cands.push({ text: c.text, order: c.order, selectKey: c.select_key, code: c.code, origin: c.origin });
      c.free();
    }
  }

  // 中间段的文本自动提交到 textarea
  let autoCommit = "";
  for (let i = 0; i < segments.length - 1; i++) {
    autoCommit += segments[i].text;
  }
  const ta = textareaRef.current;
  if (autoCommit && ta) {
    const pos = ta.selectionStart;
    ta.setRangeText(autoCommit, pos, pos, "end");
  }
  const lastSeg = segments[segments.length - 1];

  // 无分段：整段提交
  if (!lastSeg) {
    if (ta) {
      const pos = ta.selectionStart;
      ta.setRangeText(preedit, pos, pos, "end");
    }
    setState({ preedit: "", preeditText: "", candidates: [], lastTag: "" });
    recalc();
    return;
  }

  // 未决中
  if (pending) {
    // 有候选：中间段提交到 textarea，最后一段作为 preedit
    setState({ preedit: lastSeg.origin, preeditText: lastSeg.text, candidates: cands, lastTag: lastSeg.tagName });
    recalc();
  } else {
    if (ta) {
      const pos = ta.selectionStart;
      ta.setRangeText(lastSeg.text, pos, pos, "end");
    }
    setState({ preedit: "", preeditText: "", candidates: [], lastTag: "" });
    recalc();

  }

}

export function useIme(imeReady: boolean, textareaRef: React.RefObject<HTMLTextAreaElement | null>, recalc: () => void) {
  const [state, setState] = useState<ImeState>(EMPTY_STATE);
  const stateRef = useRef(state);
  stateRef.current = state;

  /** 清空 textarea 和 IME 状态 */
  const clear = useCallback(() => {
    setState(EMPTY_STATE);
    if (textareaRef.current) textareaRef.current.value = "";
  }, []);

  /** 写入剪贴板，兼容 HTTP 等 navigator.clipboard 不可用的环境 */
  const writeClipboard = useCallback((text: string) => {
    if (navigator.clipboard?.writeText) {
      navigator.clipboard.writeText(text).catch(() => fallbackCopy(text));
    } else {
      fallbackCopy(text);
    }
  }, []);

  /** 复制全部文本到剪贴板 */
  const copyText = useCallback(() => {
    const text = textareaRef.current?.value ?? "";
    if (!text) return;
    writeClipboard(text);
  }, [writeClipboard]);

  const copyAndClear = useCallback(() => {
    const ta = textareaRef.current;
    const text = ta?.value ?? "";
    if (!text) return;
    writeClipboard(text);
    setState(EMPTY_STATE);
    if (ta) ta.value = "";
  }, [writeClipboard]);

  /** 拦截 textarea 的 keydown，IME 相关键 preventDefault */
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
      if (!imeReady) return;

      // Ctrl+Shift+X: 复制并清空
      if (e.key === "X" && e.ctrlKey && e.shiftKey) {
        e.preventDefault();
        copyAndClear();
        return;
      }

      // Ctrl+C: 有 preedit 时拦截，避免复制到 preedit
      if (e.key === "c" && e.ctrlKey) {
        if (stateRef.current.preedit) {
          e.preventDefault();
          copyText();
        }
        return;
      }

      // Ctrl+X: 清空
      if (e.key === "x" && e.ctrlKey) {
        e.preventDefault();
        clear();
        return;
      }

      if (e.ctrlKey || e.metaKey) return;

      const s = stateRef.current;

      // 有 preedit 时的特殊键处理
      if (s.preedit) {
        // Enter: 提交 preedit 原始编码
        if (e.key === "Enter") {
          e.preventDefault();
          const ta = textareaRef.current;
          if (ta) {
            const pos = ta.selectionStart;
            ta.setRangeText(s.preedit, pos, pos, "end");
          }
          setState({ preedit: "", preeditText: "", candidates: [], lastTag: "" });
          recalc();
          return;
        }

        // Backspace: 删除 preedit 最后一个字符
        if (e.key === "Backspace") {
          e.preventDefault();
          const newPreedit = s.preedit.slice(0, -1);
          runCompletion(newPreedit, textareaRef, setState, recalc);
          return;
        }

        // Escape: 清空 preedit
        if (e.key === "Escape") {
          e.preventDefault();
          setState({ preedit: "", preeditText: "", candidates: [], lastTag: "" });
          return;
        }

        // PageUp / PageDown: 翻页（追加翻页字符到 preedit）
        if (e.key === "PageUp" || e.key === "PageDown") {
          e.preventDefault();
          const ch = e.key === "PageUp" ? "\u21DE" : "\u21DF"; // ⇞ / ⇟
          runCompletion(s.preedit + ch, textareaRef, setState, recalc);
          return;
        }

        // 字母/数字/空格/符号等可打印字符：追加到 preedit，由 completion API 处理
        if (e.key.length === 1) {
          e.preventDefault();
          runCompletion(s.preedit + e.key, textareaRef, setState, recalc);
          return;
        }

        // 其他键（方向键等）：不拦截，让 textarea 原生处理
        return;
      }

      // 无 preedit 时：可打印字符开始新的编码（字母、数字、标点等）
      if (e.key.length === 1) {
        e.preventDefault();
        runCompletion(e.key, textareaRef, setState, recalc);
        return;
      }

      // 其他键：不拦截，textarea 原生处理（Backspace、方向键、Enter 等）
    },
    [imeReady, copyText, copyAndClear, clear, recalc],
  );

  return {
    state,
    handleKeyDown,
    clear,
    copyText,
    copyAndClear,
  };
}
