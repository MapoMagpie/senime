import { useState, useCallback, useEffect, type RefObject } from "react";

export interface CursorPos {
  top: number;
  left: number;
  showAbove: boolean;
}

/**
 * 计算 textarea 光标的像素位置，用于定位浮动的 IME 弹窗。
 * 使用 hidden mirror div 镜像 textarea 内容至光标处来测量。
 */
export function useCursorPos(textareaRef: RefObject<HTMLTextAreaElement | null>) {
  const [pos, setPos] = useState<CursorPos>({ top: 0, left: 0, showAbove: false });

  const recalc = useCallback(() => {
    const ta = textareaRef.current;
    if (!ta) return;

    const sel = ta.selectionStart;

    // 创建或复用 mirror div
    let mirror = document.getElementById("cursor-mirror") as HTMLDivElement | null;
    if (!mirror) {
      mirror = document.createElement("div");
      mirror.id = "cursor-mirror";
      const style = mirror.style;
      style.position = "absolute";
      style.visibility = "hidden";
      style.whiteSpace = "pre-wrap";
      style.wordWrap = "break-word";
      style.overflow = "hidden";
      style.pointerEvents = "none";
      document.body.appendChild(mirror);
    }

    // 镜像 textarea 的关键样式
    const cs = window.getComputedStyle(ta);
    const props = [
      "fontFamily", "fontSize", "fontWeight", "fontStyle",
      "letterSpacing", "lineHeight", "textTransform",
      "paddingTop", "paddingRight", "paddingBottom", "paddingLeft",
      "borderTopWidth", "borderRightWidth", "borderBottomWidth", "borderLeftWidth",
      "boxSizing", "width",
    ] as const;
    for (const p of props) {
      (mirror.style as any)[p] = cs[p];
    }

    // 写入内容至光标位置
    const before = ta.value.substring(0, sel);
    const textNode = document.createTextNode(before);
    const marker = document.createElement("span");
    marker.textContent = "\u200b"; // 零宽空格标记
    mirror.replaceChildren(textNode, marker);

    const markerRect = marker.getBoundingClientRect();
    const mirrorRect = mirror.getBoundingClientRect();

    const rawTop = markerRect.top - mirrorRect.top + ta.offsetTop - ta.scrollTop;
    const rawLeft = markerRect.left - mirrorRect.left + ta.offsetLeft;
    const lineHeight = markerRect.height || parseFloat(cs.lineHeight) || 20;

    const maxTop = ta.offsetTop + ta.clientHeight - lineHeight;
    const clampedTop = Math.min(rawTop, maxTop);
    const clampedLeft = Math.min(Math.max(rawLeft, ta.offsetLeft), ta.offsetLeft + ta.clientWidth);

    // 光标在 textarea 底部附近时，弹窗显示在上方
    const showAbove = rawTop >= maxTop;

    setPos({ top: clampedTop, left: clampedLeft, showAbove });
  }, [textareaRef]);

  useEffect(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    const onScroll = () => recalc();
    const onInput = () => recalc();
    ta.addEventListener("scroll", onScroll);
    ta.addEventListener("input", onInput);
    window.addEventListener("resize", recalc);
    return () => {
      ta.removeEventListener("scroll", onScroll);
      ta.removeEventListener("input", onInput);
      window.removeEventListener("resize", recalc);
    };
  }, [textareaRef, recalc]);

  return { pos, recalc };
}
