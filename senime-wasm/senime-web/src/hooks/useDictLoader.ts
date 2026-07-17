import { useState, useEffect, useCallback } from "react";
import init, { init_ime, load_bin } from "senime-wasm";
import { getFile, saveFile } from "../db";

const DICT_KEY = "dict_bin";
const CONFIG_KEY = "dict_config";

const DEFAULT_SELECTION_KEYS: string[] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const DEFAULT_PAGE_COUNT = 5;

export type DictStatus = {
  state: "none" | "ready" | "loading" | "file_downloading" | "file_selected" | "error",
  message: string,
};
export type DictConfig = {
  selection_keys: string[],
  page_count: number,
  file?: File | string,
  dict_name?: string,
}

const DEFAULT_CONFIG = {
  selection_keys: DEFAULT_SELECTION_KEYS,
  page_count: DEFAULT_PAGE_COUNT,
};

export function useDictLoader() {
  const [status, setStatus] = useState<DictStatus>({ state: "none", message: "请选择码表文件(.txt)" });
  const [config, setConfig] = useState<DictConfig>(DEFAULT_CONFIG);
  // 初始化 WASM 并尝试从缓存加载
  useEffect(() => {
    (async () => {
      try {
        await init();
        setStatus({ state: "loading", message: "尝试加载缓存数据中..." });
        const [cached, cachedConfig] = await Promise.all([getFile(DICT_KEY), getFile(CONFIG_KEY)]);
        if (cached instanceof Uint8Array && typeof cachedConfig === "string") {
          const cfg = JSON.parse(cachedConfig);
          setConfig(cfg as DictConfig);
          // 加载引擎
          load_bin(cached, cachedConfig);
          setStatus({ state: "ready", message: "已从缓存加载码表" });
        } else {
          setStatus({ state: "none", message: "请选择码表文件(.txt)" });
        }
      } catch (e) {
        setStatus({ state: "error", message: String(e) });
      }
    })();
  }, []);

  // 用户传入码表文本内容进行加载
  const onConfirm = useCallback(async () => {
    try {
      if (!config.file) throw new Error("请先上传码表文件或选择预设码表");
      setStatus({ state: "file_downloading", message: "下载码表中..." });
      const content = await download(config.file);
      setStatus({ state: "loading", message: "加载码表中..." });
      const cfg = JSON.stringify({ selection_keys: config.selection_keys, page_count: config.page_count, dict_name: config.dict_name });
      const bin = init_ime(content, cfg);
      await Promise.all([saveFile(DICT_KEY, bin), saveFile(CONFIG_KEY, cfg)]);
      setStatus({ state: "ready", message: "码表已加载并缓存" });
    } catch (e) {
      setStatus({ state: "error", message: `加载失败: ${e}` });
    }
  }, [config, setStatus]);

  return { status, setStatus, config, setConfig, onConfirm };
}

async function download(file: File | string): Promise<string> {
  if (typeof file === "string") {
    const text = await fetch(file).then(resp => resp.text()).catch(Error);
    if (text instanceof Error) {
      throw new Error(`下载码表文件失败，${text}`, { cause: text.cause })
    };
    return text;
  } else if (file instanceof File) {
    return await file.text();
  } else {
    throw new Error("非有效的文本文件或URL");
  }
}
