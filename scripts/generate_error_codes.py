#!/usr/bin/env python3
"""
Generate docs/ERROR_CODES.md by scanning contracts/*/src/errors.rs and other files
that define `pub enum Error { ... }`.

Usage:
  scripts/generate_error_codes.py [--write]

If --write is provided, overwrite docs/ERROR_CODES.md; otherwise print to stdout.
"""
import argparse
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
CONTRACTS_DIR = ROOT / 'contracts'
DOCS_PATH = ROOT / 'docs' / 'ERROR_CODES.md'

ENUM_START_RE = re.compile(r'^\s*pub\s+enum\s+Error')
VARIANT_RE = re.compile(r'^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([0-9]+)\s*,?')
DOC_COMMENT_RE = re.compile(r'^\s*///\s?(.*)')


def parse_errors_rs(path: Path):
    """Return list of (code:int, symbol:str, doc:str) parsed from file."""
    codes = []
    if not path.exists():
        return codes
    inside = False
    last_doc_lines = []
    for line in path.read_text(encoding='utf-8').splitlines():
        if not inside:
            if ENUM_START_RE.search(line):
                inside = True
            continue
        # end of enum
        if line.strip().startswith('}'):
            break
        mdoc = DOC_COMMENT_RE.match(line)
        if mdoc:
            last_doc_lines.append(mdoc.group(1).strip())
            continue
        mvar = VARIANT_RE.match(line)
        if mvar:
            symbol = mvar.group(1)
            code = int(mvar.group(2))
            doc = ' '.join(last_doc_lines).strip() if last_doc_lines else ''
            codes.append((code, symbol, doc))
            last_doc_lines = []
            continue
        # reset on unrelated lines
        if line.strip().startswith('//'):
            continue
        last_doc_lines = []
    return codes


def find_contracts():
    contracts = []
    for p in CONTRACTS_DIR.iterdir():
        if p.is_dir():
            contracts.append(p.name)
    return sorted(contracts)


def collect_all():
    all_records = {}
    for contract in find_contracts():
        path = CONTRACTS_DIR / contract / 'src'
        # common location
        candidates = [path / 'errors.rs']
        # some projects put enum in lib.rs
        candidates.append(path / 'lib.rs')
        candidates.append(path / 'types.rs')
        candidates = [c for c in candidates if c.exists()]
        records = []
        for c in candidates:
            records.extend(parse_errors_rs(c))
        if records:
            # sort by code
            records.sort(key=lambda x: (x[0], x[1]))
            all_records[contract] = records
    return all_records


def render_markdown(all_records):
    lines = []
    lines.append('# Error Codes Reference')
    lines.append('')
    lines.append('> Comprehensive reference of all contract error codes across the VitaStellar Contracts ecosystem.')
    lines.append('> Auto-generated from contract source. Do not edit manually.')
    lines.append('')
    lines.append('## Per-Contract Error Codes')
    lines.append('')

    for contract in sorted(all_records.keys()):
        lines.append(f'### {contract}')
        lines.append('')
        lines.append('| Code | Symbol | Description |')
        lines.append('|------|--------|-------------|')
        for code, symbol, doc in all_records[contract]:
            desc = doc if doc else 'Generated from contract source'
            # escape pipes
            desc = desc.replace('|', '\\|')
            lines.append(f'| {code} | {symbol} | {desc} |')
        lines.append('')
    if not all_records:
        lines.append('_No contract error enums found._')
        lines.append('')
    return '\n'.join(lines) + '\n'


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--write', action='store_true', help='Write to docs/ERROR_CODES.md')
    args = parser.parse_args()

    all_records = collect_all()
    out = render_markdown(all_records)

    if args.write:
        DOCS_PATH.parent.mkdir(parents=True, exist_ok=True)
        DOCS_PATH.write_text(out, encoding='utf-8')
        print(f'Wrote {DOCS_PATH} ({sum(len(v) for v in all_records.values())} entries)')
        return 0
    else:
        print(out)
        return 0


if __name__ == '__main__':
    sys.exit(main())
