const ext = globalThis.browser ?? globalThis.chrome;
const storage = ext.storage.local;

const STORAGE_KEY = "senime_dict_bytes";

function setStorage(values) {
  return new Promise((resolve, reject) => {
    let settled = false;
    const finalizeResolve = () => {
      if (!settled) {
        settled = true;
        resolve();
      }
    };
    const finalizeReject = (error) => {
      if (!settled) {
        settled = true;
        reject(error);
      }
    };
    const maybePromise = storage.set(values, () => {
      const error = ext.runtime.lastError;
      if (error) {
        finalizeReject(new Error(error.message));
        return;
      }
      finalizeResolve();
    });
    if (maybePromise && typeof maybePromise.then === "function") {
      maybePromise.then(finalizeResolve, finalizeReject);
    }
  });
}

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

function setStatus(text) {
  const el = document.getElementById("status");
  el.textContent = text;
}

async function refreshStatus() {
  const response = await sendMessage({ type: "senime-status" });
  if (response?.ready) {
    setStatus("字典已加载，Senibe 已可用于页面输入框。");
  } else {
    setStatus("尚未检测到已加载的字典，请上传 `.bin` 文件。");
  }
}

document.getElementById("dict-file").addEventListener("change", async (event) => {
  const input = event.target;
  const file = input.files?.[0];
  if (!file) {
    return;
  }
  setStatus(`正在读取 ${file.name}...`);
  try {
    const buffer = await file.arrayBuffer();
    const bytes = Array.from(new Uint8Array(buffer));
    await setStorage({ [STORAGE_KEY]: bytes });
    await sendMessage({ type: "senime-dict-updated" });
    setStatus(`字典已保存并重新加载: ${file.name}`);
  } catch (error) {
    setStatus(`字典加载失败: ${String(error)}`);
  } finally {
    input.value = "";
  }
});

document.getElementById("reload").addEventListener("click", async () => {
  setStatus("正在重新加载字典...");
  try {
    const response = await sendMessage({ type: "senime-dict-updated" });
    if (response?.ok) {
      setStatus("字典重新加载完成。");
    } else {
      setStatus("未找到已保存的字典，请先上传文件。");
    }
  } catch (error) {
    setStatus(`重新加载失败: ${String(error)}`);
  }
});

refreshStatus().catch((error) => {
  setStatus(`状态读取失败: ${String(error)}`);
});
