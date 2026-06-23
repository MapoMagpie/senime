interface Props {
  text: string;
  onClear: () => void;
  onCopy: () => void;
  onCopyAndClear: () => void;
}

export function ActionBar({ text, onClear, onCopy, onCopyAndClear }: Props) {
  return (
    <section className="action-bar">
      <button onClick={onClear}>清空 (Ctrl+X)</button>
      <button onClick={onCopy} disabled={!text}>
        复制 (Ctrl+C)
      </button>
      <button onClick={onCopyAndClear} disabled={!text}>
        复制并清空 (Ctrl+Shift+X)
      </button>
    </section>
  );
}
