import { useState, useEffect, useCallback } from "react";

type ThemeMode = "auto" | "light" | "dark";

const STORAGE_KEY = "senime-theme";

/** 读取 localStorage 中保存的主题偏好 */
function readStored(): ThemeMode {
  try {
    const v = localStorage.getItem(STORAGE_KEY);
    if (v === "light" || v === "dark" || v === "auto") return v;
  } catch { /* 无权限访问 localStorage 时忽略 */ }
  return "auto";
}

function writeStored(mode: ThemeMode) {
  try { localStorage.setItem(STORAGE_KEY, mode); } catch { /* 忽略 */ }
}

/** 根据 mode 和系统偏好决定实际应用的 data-theme 值 */
function resolveTheme(mode: ThemeMode): "light" | "dark" {
  if (mode === "auto") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  }
  return mode;
}

export function useTheme() {
  const [mode, setMode] = useState<ThemeMode>(readStored);

  // 监听系统主题变化（仅在 auto 模式下需要响应）
  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = () => {
      if (mode === "auto") {
        document.documentElement.setAttribute(
          "data-theme",
          mq.matches ? "dark" : "light",
        );
      }
    };
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, [mode]);

  // mode 变化时同步 data-theme 与 localStorage
  useEffect(() => {
    document.documentElement.setAttribute("data-theme", resolveTheme(mode));
    writeStored(mode);
  }, [mode]);

  // 三态循环：auto → light → dark → auto
  const cycle = useCallback(() => {
    setMode((prev) => {
      if (prev === "auto") return "light";
      if (prev === "light") return "dark";
      return "auto";
    });
  }, []);

  return { mode, cycle } as const;
}
