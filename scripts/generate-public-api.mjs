#!/usr/bin/env node
/**
 * Public API Generator for VitaStellar Contracts
 *
 * Walks each contract workspace member and concatenates /// doc comments +
 * pub fn signatures into docs/PUBLIC_API.md.
 *
 * Usage:
 *   node scripts/generate-public-api.mjs [--output path/to/PUBLIC_API.md]
 *
 * Canonical entry-point list for each contract version; wire into `make docs`.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const PROJECT_ROOT = path.resolve(__dirname, '..');

const CONTRACTS_DIR = path.join(PROJECT_ROOT, 'contracts');

const DEFAULT_OUTPUT = path.join(PROJECT_ROOT, 'docs', 'PUBLIC_API.md');
const outputIdx = process.argv.indexOf('--output');
const OUTPUT_PATH = outputIdx !== -1 ? process.argv[outputIdx + 1] : DEFAULT_OUTPUT;

// Contracts that MUST appear in the output (acceptance criteria).
const REQUIRED_CONTRACTS = [
  'healthcare_reputation',
  'identity_registry',
  'credential_registry',
  'payment_router',
];

/**
 * Parse all public entry-points from a contract's lib.rs.
 * Returns { name, functions: [{ signature, docs }] }
 */
function parseContract(contractDir) {
  const libPath = path.join(contractDir, 'src', 'lib.rs');
  if (!fs.existsSync(libPath)) return null;

  const name = path.basename(contractDir);
  const lines = fs.readFileSync(libPath, 'utf8').split('\n');

  const functions = [];
  let pendingDocs = [];

  for (let i = 0; i < lines.length; i++) {
    const raw = lines[i];
    const trimmed = raw.trim();

    if (trimmed.startsWith('///')) {
      pendingDocs.push(trimmed.replace(/^\/\/\/\s?/, ''));
      continue;
    }

    // Match a pub fn that is part of a #[contractimpl] block.
    // Accept multi-line signatures by collecting until we see `{` or `->`.
    if (/^\s*pub fn \w+/.test(raw)) {
      // Gather the full signature (may span several lines before the opening `{`)
      let sigLines = [trimmed];
      let j = i + 1;
      while (j < lines.length && !lines[j - 1].includes('{')) {
        const next = lines[j].trim();
        sigLines.push(next);
        if (next.includes('{')) break;
        j++;
      }

      // Strip body: keep everything up to (but not including) the opening `{`
      let fullSig = sigLines.join(' ').replace(/\s+/g, ' ');
      const braceIdx = fullSig.indexOf('{');
      if (braceIdx !== -1) fullSig = fullSig.slice(0, braceIdx).trim();

      functions.push({ signature: fullSig, docs: pendingDocs });
      pendingDocs = [];
      continue;
    }

    // Any non-blank, non-attribute line resets the pending doc buffer.
    if (trimmed && !trimmed.startsWith('#') && !trimmed.startsWith('//')) {
      pendingDocs = [];
    }
  }

  return { name, functions };
}

/**
 * Render a single contract section in Markdown.
 */
function renderContract({ name, functions }) {
  const anchor = name.replace(/_/g, '-');
  let md = `## ${name}\n\n`;

  if (functions.length === 0) {
    md += '_No public entry-points found._\n\n';
    return md;
  }

  for (const fn of functions) {
    if (fn.docs.length > 0) {
      md += fn.docs.map(l => `> ${l}`).join('\n') + '\n\n';
    }
    md += `\`\`\`rust\n${fn.signature}\n\`\`\`\n\n`;
  }

  return md;
}

function main() {
  // Collect all contract directories.
  const allDirs = fs.readdirSync(CONTRACTS_DIR)
    .map(n => path.join(CONTRACTS_DIR, n))
    .filter(p => fs.statSync(p).isDirectory());

  // Parse each contract, keep only those with at least one pub fn.
  const allParsed = allDirs
    .map(parseContract)
    .filter(c => c !== null);

  // Check required contracts are present.
  const found = new Set(allParsed.map(c => c.name));
  const missing = REQUIRED_CONTRACTS.filter(r => !found.has(r));
  if (missing.length > 0) {
    console.error(`ERROR: Required contracts not found: ${missing.join(', ')}`);
    process.exit(1);
  }

  // Sort: required contracts first (in declared order), then rest alphabetically.
  const requiredSet = new Set(REQUIRED_CONTRACTS);
  const requiredContracts = REQUIRED_CONTRACTS
    .map(r => allParsed.find(c => c.name === r))
    .filter(Boolean);
  const otherContracts = allParsed
    .filter(c => !requiredSet.has(c.name))
    .sort((a, b) => a.name.localeCompare(b.name));

  const ordered = [...requiredContracts, ...otherContracts];

  const timestamp = new Date().toISOString();
  const tocLines = ordered.map(c => `- [${c.name}](#${c.name.replace(/_/g, '-')})`);

  let md = `# VitaStellar — Public Contract API\n\n`;
  md += `> **Single source of truth.** Generated from \`/// \` doc comments and \`pub fn\` signatures in each contract's \`src/lib.rs\`.\n`;
  md += `> Do not edit manually — run \`make docs\` to regenerate.\n\n`;
  md += `- **Generated**: \`${timestamp}\`\n`;
  md += `- **Contracts**: ${ordered.length}\n\n`;
  md += `## Table of Contents\n\n${tocLines.join('\n')}\n\n---\n\n`;

  for (const contract of ordered) {
    md += renderContract(contract);
    md += `---\n\n`;
  }

  fs.mkdirSync(path.dirname(OUTPUT_PATH), { recursive: true });
  fs.writeFileSync(OUTPUT_PATH, md, 'utf8');
  console.log(`✅ PUBLIC_API.md written to ${OUTPUT_PATH}`);
  console.log(`   Contracts documented: ${ordered.length}`);
  console.log(`   Required contracts covered: ${REQUIRED_CONTRACTS.join(', ')}`);
}

main();
