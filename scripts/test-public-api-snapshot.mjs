#!/usr/bin/env node
/**
 * Snapshot test for docs/PUBLIC_API.md
 *
 * Regenerates the public API doc into a temp file and diffs it against the
 * committed snapshot (docs/PUBLIC_API.md).  Exits with code 1 if they differ,
 * which signals drift: a contract signature was changed without regenerating docs.
 *
 * Usage (CI / pre-commit):
 *   node scripts/test-public-api-snapshot.mjs
 *
 * To accept the current state as the new baseline:
 *   node scripts/generate-public-api.mjs   # update snapshot
 *   git add docs/PUBLIC_API.md
 */

import { execSync } from 'child_process';
import fs from 'fs';
import os from 'os';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const PROJECT_ROOT = path.resolve(__dirname, '..');
const SNAPSHOT = path.join(PROJECT_ROOT, 'docs', 'PUBLIC_API.md');
const GENERATOR = path.join(PROJECT_ROOT, 'scripts', 'generate-public-api.mjs');

if (!fs.existsSync(SNAPSHOT)) {
  console.error(`ERROR: Snapshot not found at ${SNAPSHOT}`);
  console.error('Run `node scripts/generate-public-api.mjs` to create it first.');
  process.exit(1);
}

// Generate fresh output to a temp file (strip the timestamp line before comparing
// so it doesn't create false positives on every run).
const tmpFile = path.join(os.tmpdir(), `public_api_fresh_${Date.now()}.md`);

try {
  execSync(`node ${GENERATOR} --output ${tmpFile}`, { stdio: 'pipe' });

  const normalize = content =>
    content
      .split('\n')
      // Strip the timestamp line — it changes on every run but is not a drift signal.
      .filter(l => !l.startsWith('- **Generated**:'))
      .join('\n');

  const fresh = normalize(fs.readFileSync(tmpFile, 'utf8'));
  const snapshot = normalize(fs.readFileSync(SNAPSHOT, 'utf8'));

  if (fresh === snapshot) {
    console.log('✅ PUBLIC_API.md snapshot matches — no drift detected.');
    process.exit(0);
  }

  // Print a simple line-diff to help developers locate the change.
  const freshLines = fresh.split('\n');
  const snapLines = snapshot.split('\n');
  const maxLen = Math.max(freshLines.length, snapLines.length);

  console.error('❌ DRIFT DETECTED: docs/PUBLIC_API.md is out of date.\n');
  console.error('Changed lines (snapshot → fresh):');
  let shown = 0;
  for (let i = 0; i < maxLen && shown < 20; i++) {
    const s = snapLines[i] ?? '(missing)';
    const f = freshLines[i] ?? '(missing)';
    if (s !== f) {
      console.error(`  line ${i + 1}:`);
      console.error(`    snapshot: ${s}`);
      console.error(`    fresh:    ${f}`);
      shown++;
    }
  }
  console.error('\nRun `make docs` (or `node scripts/generate-public-api.mjs`) to update the snapshot.');
  process.exit(1);
} finally {
  if (fs.existsSync(tmpFile)) fs.unlinkSync(tmpFile);
}
