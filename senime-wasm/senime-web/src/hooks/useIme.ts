import { useState, useCallback, useEffect, useRef } from "react";
import { completion } from "senime-wasm";

export interface ImeState {
  /** 已确认上屏的中文文本 */
  completedText: string;
  /** 当前正在输入的原始 ascii */
  userInput: string;
  /** 最后一段有待选候选时的待定文本（带下划线展示） */
  pendingText: string;
  /** 候选列表 */
  candidates: { text: string; order: number; selectKey: string }[];
}

const EMPTY_STATE: ImeState = {
  completedText: "",
  userInput: "",
  pendingText: "",
  candidates: [],
};

export function useIme(imeReady: boolean) {
  const [state, setState] = useState<ImeState>(EMPTY_STATE);
  const stateRef = useRef(state);
  stateRef.current = state;

  // 每次 userInput 变化时调用 completion，自动提交已解析段
  useEffect(() => {
    if (!imeReady || !state.userInput) {
      // userInput 为空时清空 pending 和候选
      setState((s) => {
        if (s.pendingText || s.candidates.length > 0) {
          return { ...s, pendingText: "", candidates: [] };
        }
        return s;
      });
      return;
    }

    console.log("request completion: ", state.userInput);
    const result = completion(state.userInput);
    const segCount = result.segment_count;

    // 收集 segments
    const segments: { text: string; origin: string; tag_name: string }[] = [];
    for (let i = 0; i < segCount; i++) {
      const seg = result.segment(i);
      segments.push({ text: seg.text, origin: seg.origin, tag_name: seg.tag_name });
      seg.free();
    }

    // 收集候选（仅最后一段可能有）
    const cands: { text: string; order: number; selectKey: string }[] = [];
    if (result.has_candidates) {
      for (let i = 0; i < result.candidate_count; i++) {
        const c = result.candidate(i);
        cands.push({ text: c.text, order: c.order, selectKey: c.select_key });
        c.free();
      }
    }

    // 前 N-1 段自动提交到 completedText
    let autoCommit = "";
    for (let i = 0; i < segments.length - 1; i++) {
      autoCommit += segments[i].text;
    }

    // 最后一段：无候选则也自动提交，有候选则作为 pendingText
    const lastSeg = segments[segments.length - 1];
    if (!lastSeg) {
      // segments 为空（不应发生），直接提交 userInput
      setState((s) => ({
        ...s,
        completedText: s.completedText + s.userInput,
        userInput: "",
        pendingText: "",
        candidates: [],
      }));
      return;
    }

    if (cands.length === 0) {
      // 无候选，最后一段自动提交到 completedText
      setState((s) => ({
        ...s,
        completedText: s.completedText + autoCommit + lastSeg.text,
        userInput: "",
        pendingText: "",
        candidates: [],
      }));
    } else {
      // 有候选：completedText 追加前 N-1 段，pendingText = lastSeg.text
      // userInput 保留最后一段的 origin（去掉已提交的前缀）
      setState((s) => ({
        ...s,
        completedText: s.completedText + autoCommit,
        userInput: lastSeg.origin,
        pendingText: lastSeg.text,
        candidates: cands,
      }));
    }
  }, [state.userInput, imeReady]);

  // 选择候选：将 selectKey 追加到 userInput，由 senime-lib 处理选重
  const selectCandidate = useCallback((selectKey: string) => {
    setState((s) => ({
      ...s,
      userInput: s.userInput + selectKey,
    }));
  }, []);

  // 提交 pendingText 到 completedText（Enter）
  const commitPending = useCallback(() => {
    setState((s) => {
      if (!s.pendingText && !s.userInput) return s;
      return {
        ...s,
        completedText: s.completedText + s.pendingText + s.userInput,
        userInput: "",
        pendingText: "",
        candidates: [],
      };
    });
  }, []);

  // 清空
  const clear = useCallback(() => {
    setState(EMPTY_STATE);
  }, []);

  // 复制输入框内容到剪贴板
  const copyText = useCallback(() => {
    const s = stateRef.current;
    const text = s.completedText + s.pendingText + s.userInput;
    if (!text) return;
    navigator.clipboard.writeText(text).catch(() => {
      const ta = document.createElement("textarea");
      ta.value = text;
      document.body.appendChild(ta);
      ta.select();
      document.execCommand("copy");
      document.body.removeChild(ta);
    });
  }, []);

  // 复制并清空
  const copyAndClear = useCallback(() => {
    const s = stateRef.current;
    const text = s.completedText + s.pendingText + s.userInput;
    if (!text) return;
    navigator.clipboard.writeText(text).catch(() => {
      const ta = document.createElement("textarea");
      ta.value = text;
      document.body.appendChild(ta);
      ta.select();
      document.execCommand("copy");
      document.body.removeChild(ta);
    });
    setState(EMPTY_STATE);
  }, []);

  // 键盘事件处理 — 通过 ref 读取最新 state
  const commitPendingRef = useRef(commitPending);
  commitPendingRef.current = commitPending;
  const copyTextRef = useRef(copyText);
  copyTextRef.current = copyText;
  const copyAndClearRef = useRef(copyAndClear);
  copyAndClearRef.current = copyAndClear;
  const clearRef = useRef(clear);
  clearRef.current = clear;

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (!imeReady) return;
      const s = stateRef.current;

      // Ctrl+Shift+X: 复制并清空
      if (e.key === "X" && e.ctrlKey && e.shiftKey) {
        e.preventDefault();
        copyAndClearRef.current();
        return;
      }

      // Ctrl+C: 复制输入框内容
      if (e.key === "c" && e.ctrlKey) {
        e.preventDefault();
        copyTextRef.current();
        return;
      }

      // Ctrl+X: 清空输入框
      if (e.key === "x" && e.ctrlKey) {
        e.preventDefault();
        clearRef.current();
        return;
      }

      // Escape: 无处理
      if (e.key === "Escape") {
        return;
      }

      // Backspace: 有 userInput 则 pop 最后一个字符；无 userInput 则 pop completedText
      if (e.key === "Backspace") {
        e.preventDefault();
        if (s.userInput) {
          setState((prev) => ({
            ...prev,
            userInput: prev.userInput.slice(0, -1),
            pendingText: "",
            candidates: [],
          }));
        } else if (s.completedText) {
          setState((prev) => ({
            ...prev,
            completedText: prev.completedText.slice(0, -1),
            pendingText: "",
            candidates: [],
          }));
        }
        return;
      }

      // Enter: 提交 pendingText + userInput
      if (e.key === "Enter") {
        e.preventDefault();
        commitPendingRef.current();
        return;
      }

      // 字母键
      if (/^[a-z]$/.test(e.key)) {
        e.preventDefault();
        setState((prev) => ({ ...prev, userInput: prev.userInput + e.key }));
        return;
      }

      // 空格和其他可打印字符
      if (e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey) {
        e.preventDefault();
        setState((prev) => ({ ...prev, userInput: prev.userInput + e.key }));
      }
    },
    [imeReady]
  );

  return { state, handleKeyDown, clear, copyText, copyAndClear, selectCandidate, commitPending };
}
