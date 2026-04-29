import initWasm, {
  completion as wasmCompletion,
  init_ime as initIme,
} from "./vendor/senime-wasm/senime_wasm.js";

const ext = globalThis.browser ?? globalThis.chrome;
const storage = ext.storage.local;

const STORAGE_KEY = "senime_dict_bytes";

let wasmReady = false;
let imeReady = false;
let initPromise = null;

function getStorage(keys) {
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
    const maybePromise = storage.get(keys, (result) => {
      const error = ext.runtime.lastError;
      if (error) {
        finalizeReject(new Error(error.message));
        return;
      }
      finalizeResolve(result);
    });
    if (maybePromise && typeof maybePromise.then === "function") {
      maybePromise.then(finalizeResolve, finalizeReject);
    }
  });
}

async function ensureWasm() {
  if (wasmReady) {
    return;
  }
  if (!initPromise) {
    initPromise = initWasm().then(() => {
      wasmReady = true;
    });
  }
  await initPromise;
}

function normalizeBytes(raw) {
  if (!raw) {
    return null;
  }
  if (raw instanceof Uint8Array) {
    return raw;
  }
  if (Array.isArray(raw)) {
    return Uint8Array.from(raw);
  }
  if (raw.buffer instanceof ArrayBuffer) {
    return new Uint8Array(raw.buffer);
  }
  return null;
}

async function loadDictionaryFromStorage() {
  await ensureWasm();
  const stored = await getStorage(STORAGE_KEY);
  const bytes = normalizeBytes(stored?.[STORAGE_KEY]);
  if (!bytes || bytes.length === 0) {
    imeReady = false;
    return false;
  }
  initIme(bytes);
  imeReady = true;
  return true;
}

async function ensureIme() {
  if (imeReady) {
    return true;
  }
  return loadDictionaryFromStorage();
}

async function handleMessage(message) {
  switch (message?.type) {
    case "senime-complete": {
      const ready = await ensureIme();
      if (!ready) {
        return { ok: false, reason: "dict-not-ready" };
      }
      const input = typeof message.input === "string" ? message.input : "";
      const text = wasmCompletion(input);
      return { ok: true, text };
    }
    case "senime-dict-updated": {
      const ready = await loadDictionaryFromStorage();
      return { ok: ready };
    }
    case "senime-status": {
      const ready = await ensureIme();
      return { ok: true, ready };
    }
    default:
      return { ok: false, reason: "unknown-message" };
  }
}

ext.runtime.onMessage.addListener((message, _sender, sendResponse) => {
  handleMessage(message)
    .then((result) => sendResponse(result))
    .catch((error) => {
      console.error("senibe background error", error);
      sendResponse({
        ok: false,
        reason: "internal-error",
        detail: String(error),
      });
    });
  return true;
});

ensureIme().catch((error) => {
  console.error("senibe initialization failed", error);
});
