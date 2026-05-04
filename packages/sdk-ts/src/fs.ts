import {
  list,
  mkdir,
  open,
  removeDir,
  removeFile,
  rename,
  stat,
  type File,
  type FileStat,
  type OpenMode,
} from "layer36:fs/files";

const decoder = new TextDecoder();
const encoder = new TextEncoder();

export { list, mkdir, open, removeDir, removeFile, rename, stat };
export type { File, FileStat, OpenMode };

export const OpenMode = {
  Read: "read",
  Write: "write",
  ReadWrite: "read-write",
  Append: "append",
} as const satisfies Record<string, OpenMode>;

export function read(path: string): Uint8Array {
  return open(path, OpenMode.Read).read(4 * 1024 * 1024);
}

export function readText(path: string): string {
  return decoder.decode(read(path));
}

export function writeText(path: string, value: string): number {
  return open(path, OpenMode.Write).write(encoder.encode(value));
}
