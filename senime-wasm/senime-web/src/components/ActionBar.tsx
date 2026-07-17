interface Props {
  isEmpty: boolean;
  onClear: () => void;
  onCopy: () => void;
  onCopyAndClear: () => void;
}

export function ActionBar({ isEmpty, onClear, onCopy, onCopyAndClear }: Props) {
  return (
    <section className="action-bar">
      <button onClick={onClear}>清空 (Ctrl+X)</button>
      <button onClick={onCopy} disabled={isEmpty}>
        复制 (Ctrl+C)
      </button>
      <button onClick={onCopyAndClear} disabled={isEmpty}>
        复制并清空 (Ctrl+Shift+X)
      </button>
    </section>
  );
}
