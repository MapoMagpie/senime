import { useState, useCallback, useRef } from "react";
import { completion } from "senime-wasm";

export interface ImeState {
  completedText: string;
  pendingText: string;
  candidates: { text: string; order: number; selectKey: string }[];
}

const EMPTY_STATE: ImeState = {
  completedText: "",
  pendingText: "",
  candidates: [],
};

/**
 * 读取 hidden input 的 value，调用 wasm completion，更新 state 并回写 input。
 * 回写 input.value 不会触发 input 事件，因此不存在循环依赖。
 */
function processInput(
  inputRef: React.RefObject<HTMLInputElement | null>,
  setState: React.Dispatch<React.SetStateAction<ImeState>>,
) {
  const raw = inputRef.current?.value ?? "";

  if (!raw) {
    setState((s) => {
      if (s.pendingText || s.candidates.length > 0) {
        return { ...s, pendingText: "", candidates: [] };
      }
      return s;
    });
    return;
  }

  const result = completion(raw);
  const segCount = result.segment_count;

  const segments: { text: string; origin: string }[] = [];
  for (let i = 0; i < segCount; i++) {
    const seg = result.segment(i);
    segments.push({ text: seg.text, origin: seg.origin });
    seg.free();
  }

  const cands: { text: string; order: number; selectKey: string }[] = [];
  if (result.has_candidates) {
    for (let i = 0; i < result.candidate_count; i++) {
      const c = result.candidate(i);
      cands.push({ text: c.text, order: c.order, selectKey: c.select_key });
      c.free();
    }
  }
  // console.log("request completion, input: [", raw, "]\nresult segments: ", segments, "\nresult candidates: ", cands);

  // 中间段的文本自动提交
  let autoCommit = "";
  for (let i = 0; i < segments.length - 1; i++) {
    autoCommit += segments[i].text;
  }

  const lastSeg = segments[segments.length - 1];

  // 无分段：整段提交
  if (!lastSeg) {
    setState((s) => ({
      ...s,
      completedText: s.completedText + raw,
      pendingText: "",
      candidates: [],
    }));
    if (inputRef.current) inputRef.current.value = "";
    return;
  }

  // 无候选：全部提交（含最后一段）
  if (cands.length === 0) {
    setState((s) => ({
      ...s,
      completedText: s.completedText + autoCommit + lastSeg.text,
      pendingText: "",
      candidates: [],
    }));
    if (inputRef.current) inputRef.current.value = "";
    return;
  }

  // 有候选：中间段提交，最后一段悬停
  setState((s) => ({
    ...s,
    completedText: s.completedText + autoCommit,
    pendingText: lastSeg.text,
    candidates: cands,
  }));
  if (inputRef.current) {
    inputRef.current.value = lastSeg.origin;
    inputRef.current.setSelectionRange(lastSeg.origin.length, lastSeg.origin.length);
  }
}

export function useIme(imeReady: boolean, inputRef: React.RefObject<HTMLInputElement | null>) {
  const [state, setState] = useState<ImeState>(EMPTY_STATE);
  const stateRef = useRef(state);
  stateRef.current = state;

  /** 读取 inputRef.value → completion → 更新 state → 回写 input */
  const handleInput = useCallback(() => {
    if (!imeReady) return;
    processInput(inputRef, setState);
  }, [imeReady]);

  /** 选重：将选重键追加到 hidden input，再走 processInput */
  const selectCandidate = useCallback((selectKey: string) => {
    const el = inputRef.current;
    if (!el) return;
    el.value = el.value + selectKey;
    processInput(inputRef, setState);
  }, []);

  /** 提交：将 pendingText + input.value 合入 completedText，清空 input */
  const commitPending = useCallback(() => {
    const raw = inputRef.current?.value ?? "";
    setState((s) => {
      if (!s.pendingText && !raw) return s;
      return {
        ...s,
        completedText: s.completedText + s.pendingText + raw,
        pendingText: "",
        candidates: [],
      };
    });
    if (inputRef.current) inputRef.current.value = "";
  }, []);

  const clear = useCallback(() => {
    setState(EMPTY_STATE);
    if (inputRef.current) inputRef.current.value = "";
  }, []);

  /** 复制全部文本到剪贴板 */
  const copyText = useCallback(() => {
    const s = stateRef.current;
    const raw = inputRef.current?.value ?? "";
    const text = s.completedText + s.pendingText + raw;
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

  const copyAndClear = useCallback(() => {
    const s = stateRef.current;
    const raw = inputRef.current?.value ?? "";
    const text = s.completedText + s.pendingText + raw;
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
    if (inputRef.current) inputRef.current.value = "";
  }, []);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (!imeReady) return;

      if (e.key === "X" && e.ctrlKey && e.shiftKey) {
        e.preventDefault();
        copyAndClear();
        return;
      }

      if (e.key === "c" && e.ctrlKey) {
        e.preventDefault();
        copyText();
        return;
      }

      if (e.key === "x" && e.ctrlKey) {
        e.preventDefault();
        clear();
        return;
      }

      if (e.ctrlKey || e.metaKey) return;

      if (e.key === "Enter") {
        e.preventDefault();
        commitPending();
        return;
      }

      // Backspace: input 为空且 completedText 非空时，删最后一个字符
      if (e.key === "Backspace") {
        const raw = inputRef.current?.value ?? "";
        if (!raw) {
          e.preventDefault();
          setState((prev) => {
            if (!prev.completedText) return prev;
            return {
              ...prev,
              completedText: prev.completedText.slice(0, -1),
              pendingText: "",
              candidates: [],
            };
          });
        }
        return;
      }
    },
    [imeReady, commitPending, copyText, copyAndClear, clear],
  );

  return {
    state,
    handleKeyDown,
    handleInput,
    clear,
    copyText,
    copyAndClear,
    selectCandidate,
    commitPending,
  };
}
