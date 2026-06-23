const DB_NAME = "senime";
const DB_VERSION = 1;
const STORE_NAME = "files";

function openDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION);
    request.onupgradeneeded = (e) => {
      const db = (e.target as IDBRequest<IDBDatabase>).result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        db.createObjectStore(STORE_NAME, { keyPath: "id" });
      }
    };
    request.onsuccess = (e) => resolve((e.target as IDBRequest<IDBDatabase>).result);
    request.onerror = (e) => reject((e.target as IDBRequest<IDBDatabase>).error);
  });
}

export async function saveFile(id: string, data: Uint8Array | string): Promise<void> {
  const db = await openDB();
  const tx = db.transaction(STORE_NAME, "readwrite");
  tx.objectStore(STORE_NAME).put({ id, content: data });
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
}

export async function getFile(id: string): Promise<Uint8Array | string | null> {
  const db = await openDB();
  const tx = db.transaction(STORE_NAME, "readonly");
  const request = tx.objectStore(STORE_NAME).get(id);
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result ? request.result.content : null);
    request.onerror = () => reject(request.error);
  });
}
