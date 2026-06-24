import type { ImeState } from "../hooks/useIme.ts";

interface Props {
  state: ImeState;
  imeReady: boolean;
  textareaRef: React.RefObject<HTMLTextAreaElement | null>;
  onKeyDown: (e: React.KeyboardEvent<HTMLTextAreaElement>) => void;
}

export function InputArea({ state, imeReady, textareaRef, onKeyDown }: Props) {

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
        placeholder={imeReady ? "在此输入编码..." : "请先加载码表..."}
      />
      {state.preedit && (
        <div className="preedit-bar">
          <span className="preedit-text">{state.preedit}</span>
        </div>
      )}
      {state.candidates.length > 0 && (
        <div className="candidates">
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
      )}
    </section>
  );
}
