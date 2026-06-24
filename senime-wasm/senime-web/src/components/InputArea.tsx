import { useEffect } from "react";
import type { ImeState } from "../hooks/useIme.ts";

interface Props {
  state: ImeState;
  imeReady: boolean;
  inputRef: React.RefObject<HTMLInputElement | null>;
  onKeyDown: (e: KeyboardEvent) => void;
  onInput: () => void;
  onSelectCandidate: (selectKey: string) => void;
}

export function InputArea({
  state, imeReady, inputRef, onKeyDown, onInput, onSelectCandidate,
}: Props) {

  // 自动聚焦隐藏 input（触发移动端键盘）
  useEffect(() => {
    if (imeReady) {
      inputRef.current?.focus();
    }
  }, [imeReady]);

  // 挂载事件到隐藏 input
  useEffect(() => {
    const el = inputRef.current;
    if (!el) return;
    const kd = (e: KeyboardEvent) => onKeyDown(e);
    const inp = () => onInput();
    el.addEventListener("keydown", kd);
    el.addEventListener("input", inp);
    return () => {
      el.removeEventListener("keydown", kd);
      el.removeEventListener("input", inp);
    };
  }, [onKeyDown, onInput]);

  // 点击展示区时聚焦隐藏 input
  const handleClick = () => {
    inputRef.current?.focus();
  };

  // 点击候选时也保持聚焦
  const handleCandidateClick = (selectKey: string) => {
    onSelectCandidate(selectKey);
    inputRef.current?.focus();
  };

  // cursor text 直接从 hidden input 读取（processInput 已同步）
  const cursorText = inputRef.current?.value ?? "";

  return (
    <section className="input-area">
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
            <span className="cursor-text">{cursorText}</span>
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
