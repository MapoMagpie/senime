import { useState, useEffect, useCallback } from "react";
import init, { init_ime, load_bin } from "senime-wasm";
import { getFile, saveFile } from "../db";

const DICT_KEY = "dict_bin";
const CONFIG_KEY = "dict_config";

const DEFAULT_SELECTION_KEYS: string[] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

export type DictStatus =
  | { state: "loading" }
  | { state: "wasm_init" }
  | { state: "cached"; message: string }
  | { state: "uploaded"; message: string }
  | { state: "error"; message: string }
  | { state: "idle" };

export function useDictLoader() {
  const [status, setStatus] = useState<DictStatus>({ state: "wasm_init" });
  const [imeReady, setImeReady] = useState(false);
  const [selectionKeys, setSelectionKeys] = useState<string[]>(DEFAULT_SELECTION_KEYS);

  // 初始化 WASM 并尝试从缓存加载
  useEffect(() => {
    (async () => {
      try {
        await init();
        setStatus({ state: "loading" });
        const [cached, cachedConfig] = await Promise.all([getFile(DICT_KEY), getFile(CONFIG_KEY)]);
        if (cached instanceof Uint8Array && typeof cachedConfig === "string") {
          const cfg = JSON.parse(cachedConfig);
          if (cfg.selection_keys) {
            setSelectionKeys(cfg.selection_keys);
          }
          load_bin(cached, cachedConfig);
          setImeReady(true);
          setStatus({ state: "cached", message: "已从缓存加载码表" });
        } else {
          setStatus({ state: "idle" });
        }
      } catch (e) {
        setStatus({ state: "error", message: String(e) });
      }
    })();
  }, []);

  // 用户上传 txt 码表文件
  const uploadDict = useCallback(async (file: File, keys: string[]) => {
    try {
      setStatus({ state: "loading" });
      const text = await file.text();
      const config = JSON.stringify({ selection_keys: keys });
      const bin = init_ime(text, config);
      await Promise.all([saveFile(DICT_KEY, bin), saveFile(CONFIG_KEY, config)]);
      setImeReady(true);
      setStatus({ state: "uploaded", message: "码表已加载并缓存" });
    } catch (e) {
      setStatus({ state: "error", message: `加载失败: ${e}` });
    }
  }, []);

  return { status, imeReady, selectionKeys, setSelectionKeys, uploadDict };
}
