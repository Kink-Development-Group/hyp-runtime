#!/usr/bin/env node
import { copyFileSync, chmodSync, statSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const repoRoot = resolve(__dirname, '..', '..');
const source = resolve(repoRoot, 'install.sh');
const target = resolve(__dirname, '..', 'static', 'install.sh');

try {
  statSync(source);
} catch (error) {
  console.error(`[sync-installer] install.sh not found at ${source}`);
  process.exit(1);
}

copyFileSync(source, target);
try {
  chmodSync(target, 0o755);
} catch (error) {
  // On Windows chmod may fail; ignore silently
}

console.log(`[sync-installer] Copied installer to ${target}`);
