import './style.css';
import typescriptLogo from './typescript.svg';
import viteLogo from '/vite.svg';
import init, { completion, init_ime } from 'senime-wasm';


document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <a href="https://vite.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <h1>Vite + TypeScript</h1>
    <div class="card">
      <input id="input" type="text" />
      <button id="button" type="button">Submit</button>
      <input type="file" id="file" name="dict_file" accept="application/octet-stream" />
    </div>
    <p id="show" class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`;
init().then(() => {
  const input = document.querySelector<HTMLInputElement>("#input");
  const button = document.querySelector<HTMLButtonElement>("#button");
  const show = document.querySelector<HTMLParagraphElement>("#show");
  const fileInput = document.querySelector<HTMLInputElement>("#file");
  if (!input || !button || !show || !fileInput) {
    throw new Error("elements are not ready!");
  }

  fileInput.addEventListener("change", () => {
    if (fileInput.files && fileInput.files.length > 0) {
      const file = fileInput.files[0];
      file.arrayBuffer().then(buf => {
        const bin = new Uint8Array(buf);
        saveFile("dict_aaa", bin);
        init_ime(bin);
      }).then(() => {
        show.textContent = "init input method engine finished!";
      }).catch(console.error);
    }
  });

  input.addEventListener("input", () => {
    if (input.value.trim()) {
      const str = completion(input.value);
      show.textContent = str;
    }
  });

  getFile("dict_aaa").then(bin => {
    if (bin) {
      init_ime(bin);
      show.textContent = "init input method engine finished!";
    }
  });
  // button.addEventListener("click", () => {
  //   if (input.value.trim()) {
  //     const str = completion(input.value);
  //     show.textContent = str;
  //   }
  // })
});

const initDB = () => {
  return new Promise<IDBDatabase>((resolve, reject) => {
    const request = indexedDB.open("dict-bin", 1);

    request.onupgradeneeded = (e) => {
      const db = (e.target as IDBRequest<IDBDatabase>).result;
      // 创建一个名为 'files' 的存储对象，以 'id' 作为主键
      if (!db.objectStoreNames.contains("files")) {
        db.createObjectStore("files", { keyPath: "id" });
      }
    };

    request.onsuccess = (e) => resolve((e.target as IDBRequest<IDBDatabase>).result);
    request.onerror = (e) => reject((e.target as IDBRequest<IDBDatabase>).error);
  });
};

// 2. 存储数据 (支持 Uint8Array 或 Blob)
const saveFile = async (id: string, data: Uint8Array) => {
  const db = await initDB();
  const tx = db.transaction("files", "readwrite");
  const store = tx.objectStore("files");

  // 直接放入对象，不需要转换
  store.put({ id, content: data });

  return new Promise<void>((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
};

// 3. 取出数据
const getFile = async (id: string) => {
  const db = await initDB();
  const tx = db.transaction("files", "readonly");
  const store = tx.objectStore("files");
  const request = store.get(id);

  return new Promise<Uint8Array | null>((resolve, reject) => {
    request.onsuccess = () => resolve(request.result ? request.result.content : null);
    request.onerror = () => reject(request.error);
  });
};
