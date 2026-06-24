import { useRef } from "react";
import { useDictLoader } from "./hooks/useDictLoader";
import { useIme } from "./hooks/useIme";
import { DictLoader } from "./components/DictLoader";
import { InputArea } from "./components/InputArea";
import { ActionBar } from "./components/ActionBar";

export default function App() {
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const { status, imeReady, selectionKeys, setSelectionKeys, uploadDict } = useDictLoader();
  const {
    state, handleKeyDown, clear, copyText, copyAndClear,
  } = useIme(imeReady, textareaRef);

  const displayText = textareaRef.current?.value ?? "";

  return (
    <div className="app">
      <h1 className="app-title">senime-web</h1>
      <DictLoader
        status={status}
        selectionKeys={selectionKeys}
        onSelectionKeysChange={setSelectionKeys}
        onUpload={uploadDict}
      />
      <InputArea
        state={state}
        imeReady={imeReady}
        textareaRef={textareaRef}
        onKeyDown={handleKeyDown}
      />
      <ActionBar
        text={displayText}
        onClear={clear}
        onCopy={copyText}
        onCopyAndClear={copyAndClear}
      />
      <div className="help-text">
        <p>输入编码自动上屏 · <kbd>1</kbd>-<kbd>9</kbd> 选重 · <kbd>Enter</kbd> 提交原始编码 · <kbd>Ctrl+C</kbd> 复制 · <kbd>Ctrl+X</kbd> 清空 · <kbd>Ctrl+Shift+X</kbd> 复制并清空</p>
      </div>
    </div>
  );
}
