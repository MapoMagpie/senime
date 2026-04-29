const ext = globalThis.browser ?? globalThis.chrome;

const OVERLAY_ID = "senibe-suggestion";
const STATUS_ID = "senibe-status";
const INPUT_TYPES = new Set([
  "text",
  "search",
  // "url",
  // "tel",
  // "email",
  // "password",
  "textarea",
  ""
]);

let activeInput = null;
let currentSuggestion = "";
let requestToken = 0;

function sendMessage(message) {
  return new Promise((resolve, reject) => {
    let settled = false;
    const finalizeResolve = (value) => {
      if (!settled) {
        settled = true;
        resolve(value);
      }
    };
    const finalizeReject = (error) => {
      if (!settled) {
        settled = true;
        reject(error);
      }
    };
    const maybePromise = ext.runtime.sendMessage(message, (response) => {
      const error = ext.runtime.lastError;
      if (error) {
        finalizeReject(new Error(error.message));
        return;
      }
      finalizeResolve(response);
    });
    if (maybePromise && typeof maybePromise.then === "function") {
      maybePromise.then(finalizeResolve, finalizeReject);
    }
  });
}

function isSupportedInput(element) {
  if (!(element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement)) {
    return false;
  }
  if (element.disabled || element.readOnly) {
    return false;
  }
  return INPUT_TYPES.has((element.type || "").toLowerCase());
}

function ensureOverlay() {
  let overlay = document.getElementById(OVERLAY_ID);
  if (!overlay) {
    overlay = document.createElement("div");
    overlay.id = OVERLAY_ID;
    overlay.style.position = "fixed";
    overlay.style.zIndex = "2147483647";
    overlay.style.maxWidth = "min(32rem, calc(100vw - 24px))";
    overlay.style.padding = "6px 10px";
    overlay.style.borderRadius = "10px";
    overlay.style.background = "rgba(18, 22, 28, 0.96)";
    overlay.style.color = "#f8fafc";
    overlay.style.boxShadow = "0 10px 30px rgba(15, 23, 42, 0.35)";
    overlay.style.border = "1px solid rgba(148, 163, 184, 0.32)";
    overlay.style.font = "13px/1.4 ui-monospace, SFMono-Regular, Menlo, monospace";
    overlay.style.pointerEvents = "none";
    overlay.style.whiteSpace = "pre-wrap";
    overlay.style.display = "none";
    document.documentElement.appendChild(overlay);
  }
  return overlay;
}

function ensureStatus() {
  let status = document.getElementById(STATUS_ID);
  if (!status) {
    status = document.createElement("div");
    status.id = STATUS_ID;
    status.style.position = "fixed";
    status.style.zIndex = "2147483647";
    status.style.right = "12px";
    status.style.bottom = "12px";
    status.style.padding = "8px 10px";
    status.style.borderRadius = "10px";
    status.style.background = "rgba(124, 45, 18, 0.96)";
    status.style.color = "#ffedd5";
    status.style.boxShadow = "0 8px 24px rgba(124, 45, 18, 0.35)";
    status.style.font = "12px/1.4 system-ui, sans-serif";
    status.style.display = "none";
    status.style.maxWidth = "20rem";
    document.documentElement.appendChild(status);
  }
  return status;
}

function showStatus(text) {
  const status = ensureStatus();
  status.textContent = text;
  status.style.display = "block";
}

function hideStatus() {
  const status = document.getElementById(STATUS_ID);
  if (status) {
    status.style.display = "none";
  }
}

function hideOverlay() {
  currentSuggestion = "";
  const overlay = document.getElementById(OVERLAY_ID);
  if (overlay) {
    overlay.style.display = "none";
    overlay.textContent = "";
  }
}

function positionOverlay(element) {
  const overlay = ensureOverlay();
  const rect = element.getBoundingClientRect();
  const fitsBelow = rect.bottom + 88 < window.innerHeight;
  const top = fitsBelow
    ? rect.bottom + 8
    : Math.max(12, rect.top - 56);
  const left = Math.min(window.innerWidth - 12, rect.left);
  overlay.style.top = `${top}px`;
  overlay.style.left = `${left}px`;
}

function showOverlay(element, text) {
  const overlay = ensureOverlay();
  overlay.textContent = text;
  positionOverlay(element);
  overlay.style.display = "block";
}

async function updateSuggestion(element) {
  const value = element.value ?? "";
  if (!value.trim()) {
    hideOverlay();
    hideStatus();
    return;
  }

  const token = ++requestToken;
  try {
    const response = await sendMessage({
      type: "senime-complete",
      input: value,
    });
    if (token !== requestToken || element !== activeInput) {
      return;
    }
    if (!response?.ok) {
      hideOverlay();
      if (response?.reason === "dict-not-ready") {
        showStatus("Senibe 未初始化码表，请在扩展设置页上传字典二进制。");
      }
      return;
    }
    hideStatus();
    const suggestion = typeof response.text === "string" ? response.text : "";
    if (!suggestion || suggestion === value) {
      hideOverlay();
      return;
    }
    currentSuggestion = suggestion;
    showOverlay(element, `${suggestion}\nTab 上屏`);
  } catch (error) {
    if (token !== requestToken) {
      return;
    }
    hideOverlay();
    showStatus(`Senibe 请求失败: ${String(error)}`);
  }
}

function commitSuggestion() {
  if (!activeInput || !currentSuggestion) {
    return false;
  }
  activeInput.value = currentSuggestion;
  activeInput.dispatchEvent(new Event("input", { bubbles: true }));
  activeInput.dispatchEvent(new Event("change", { bubbles: true }));
  hideOverlay();
  return true;
}

document.addEventListener("focusin", (event) => {
  const target = event.target;
  if (isSupportedInput(target)) {
    activeInput = target;
    updateSuggestion(target);
    return;
  }
  activeInput = null;
  hideOverlay();
});

document.addEventListener("focusout", (event) => {
  if (event.target === activeInput) {
    activeInput = null;
    hideOverlay();
  }
});

document.addEventListener("input", (event) => {
  if (event.target === activeInput && activeInput) {
    updateSuggestion(activeInput);
  }
}, true);

document.addEventListener("keydown", (event) => {
  if (event.key === "Tab" && event.target === activeInput && currentSuggestion) {
    event.preventDefault();
    event.stopPropagation();
    commitSuggestion();
    return;
  }
  if (event.key === "Escape") {
    hideOverlay();
  }
}, true);

window.addEventListener("scroll", () => {
  if (activeInput && currentSuggestion) {
    positionOverlay(activeInput);
  }
}, true);

window.addEventListener("resize", () => {
  if (activeInput && currentSuggestion) {
    positionOverlay(activeInput);
  }
});
