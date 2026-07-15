import { useRef } from "react";
import { useDictLoader } from "./hooks/useDictLoader";
import { useIme } from "./hooks/useIme";
import { useCursorPos } from "./hooks/useCursorPos";
import { DictLoader } from "./components/DictLoader";
import { InputArea } from "./components/InputArea";
import { ActionBar } from "./components/ActionBar";

export default function App() {
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const { pos, recalc } = useCursorPos(textareaRef);
  const { status, imeReady, selectionKeys, setSelectionKeys, pageCount, setPageCount, uploadDict } = useDictLoader();
  const {
    state, handleKeyDown, clear, copyText, copyAndClear,
  } = useIme(imeReady, textareaRef, recalc);

  const displayText = textareaRef.current?.value ?? "";

  return (
    <div className="app">
      <header className="app-header">
        <div className="app-title-row">
          <h1 className="app-title">Senime</h1>
          <a
            className="app-github"
            href="https://github.com/MapoMagpie/senime"
            target="_blank"
            rel="noopener noreferrer"
          >
            <svg className="app-github-icon" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
            </svg>
            GitHub
          </a>
        </div>
        <p className="app-subtitle">一个在线码表输入法，高性能、易使用。</p>
      </header>
      <DictLoader
        status={status}
        imeReady={imeReady}
        selectionKeys={selectionKeys}
        pageCount={pageCount}
        onSelectionKeysChange={setSelectionKeys}
        onPageCountChange={setPageCount}
        onUpload={uploadDict}
      />
      <InputArea
        state={state}
        imeReady={imeReady}
        textareaRef={textareaRef}
        onKeyDown={handleKeyDown}
        cursorPos={pos}
      />
      <ActionBar
        text={displayText}
        onClear={clear}
        onCopy={copyText}
        onCopyAndClear={copyAndClear}
      />
    </div>
  );
}
