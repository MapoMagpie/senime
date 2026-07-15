import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { copyFileSync, mkdirSync, readdirSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

function listTables(dir: string): string[] {
  return readdirSync(dir)
    .filter((f) => f.endsWith(".txt"))
    .sort();
}

/** 生成 index.txt：每行 "名称|地址"，名称取自去后缀的文件名 */
function buildIndex(tables: string[]): string {
  return tables
    .map((f) => `${f.replace(/\.txt$/, "")}|${f}`)
    .join("\n") + "\n";
}

function copyTablesPlugin() {
  const srcDir = join(__dirname, "assets", "tables");
  let tables: string[] = [];

  return {
    name: "copy-tables",
    configureServer(server) {
      // 开发模式：动态生成 index.txt
      tables = listTables(srcDir);
      const content = buildIndex(tables);
      server.middlewares.use("/assets/tables/index.txt", (_req, res) => {
        res.setHeader("Content-Type", "text/plain; charset=utf-8");
        res.end(content);
      });
    },
    closeBundle() {
      const outDir = join(__dirname, "dist", "tables");
      mkdirSync(outDir, { recursive: true });
      tables = listTables(srcDir);
      for (const f of tables) {
        copyFileSync(join(srcDir, f), join(outDir, f));
      }
      // 生成 index.txt：名称|文件名，部署后可手动追加远程条目
      writeFileSync(join(outDir, "index.txt"), buildIndex(tables));
    },
  };
}

export default defineConfig({
  plugins: [react(), copyTablesPlugin()],
  server: {
    fs: {
      allow: [".."]
    }
  }
});
