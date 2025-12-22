import path from 'path';
import fs from 'fs';

const getDirnames = (dir: string) => {
  const msgs = fs.readdirSync(dir, { withFileTypes: true });
  // 过滤隐藏目录和 node_modules
  return msgs
    .filter(
      (msg) => msg.isDirectory() &&
        !msg.name.startsWith('.') &&
        msg.name !== 'node_modules'
    )
    .map((msg) => msg.name);
}

const setAliases = (dir: string) => {
  const aliases: Record<string, string> = {
    '@': dir,
  };

  const dirnames = getDirnames(dir);
  dirnames.forEach((dirname) => {
    aliases[`@${dirname}`] = path.join(dir, dirname);
  });
  return aliases;
}

export const ViteAliases = ({ dir = 'src', root = process.cwd() } = {}) => {
  return {
    name: 'vite-aliases',
    config() {
      const aliases = setAliases(path.resolve(root, dir));
      return {
        resolve: {
          alias: aliases,
        },
      }
    }
  }
}