import { useEffect, useRef } from "react";
import type { ImeState } from "../hooks/useIme.ts";

interface Props {
  state: ImeState;
  imeReady: boolean;
  onKeyDown: (e: KeyboardEvent) => void;
  onSelectCandidate: (selectKey: string) => void;
}

export function InputArea({ state, imeReady, onKeyDown, onSelectCandidate }: Props) {
  const inputRef = useRef<HTMLInputElement>(null);
  const displayRef = useRef<HTMLDivElement>(null);

  // 自动聚焦隐藏 input（触发移动端键盘）
  useEffect(() => {
    if (imeReady) {
      inputRef.current?.focus();
    }
  }, [imeReady]);

  // 挂载 keydown 到隐藏 input
  useEffect(() => {
    const el = inputRef.current;
    if (!el) return;
    const handler = (e: KeyboardEvent) => onKeyDown(e);
    el.addEventListener("keydown", handler);
    return () => el.removeEventListener("keydown", handler);
  }, [onKeyDown]);

  // 点击展示区时聚焦隐藏 input
  const handleClick = () => {
    inputRef.current?.focus();
  };

  // 点击候选时也保持聚焦
  const handleCandidateClick = (selectKey: string) => {
    onSelectCandidate(selectKey);
    inputRef.current?.focus();
  };

  return (
    <section className="input-area">
      {/* 隐藏的真实 input，用于触发移动端键盘 */}
      <input
        ref={inputRef}
        className="hidden-input"
        type="text"
        autoComplete="off"
        autoCorrect="off"
        autoCapitalize="off"
        spellCheck={false}
      />
      <div
        ref={displayRef}
        className="input-display"
        onClick={handleClick}
      >
        {!imeReady ? (
          <span className="placeholder">请先加载码表...</span>
        ) : (
          <>
            <span className="confirmed-text">{state.completedText}</span>
            {state.pendingText && (
              <span className="pending-text">{state.pendingText}</span>
            )}
            <span className="cursor-text">{state.userInput}</span>
            <span className="cursor">|</span>
          </>
        )}
      </div>
      {state.candidates.length > 0 && (
        <div className="candidates">
          {state.candidates.map((c, i) => (
            <span
              key={i}
              className={`candidate ${i === 0 ? "candidate-primary" : ""}`}
              onClick={() => handleCandidateClick(c.selectKey)}
            >
              <span className="candidate-key">{c.selectKey}</span>
              <span className="candidate-text">{c.text}</span>
            </span>
          ))}
        </div>
      )}
    </section>
  );
}
