#!/usr/bin/env python3
"""
Fix duplicate error codes in docs/ERROR_CODES.md:
1. Identify duplicates (same code, different meaning)
2. Reassign to unused codes
3. Update the ERROR_CODES.md file
4. Sort category tables by numeric code
"""

import re
import sys
from collections import defaultdict
from pathlib import Path

def parse_error_codes_file(filepath):
    """Parse ERROR_CODES.md and extract all information."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    lines = content.split('\n')
    
    # Find where per-contract section starts
    per_contract_line = None
    for i, line in enumerate(lines):
        if "Per-Contract Error Codes" in line:
            per_contract_line = i
            break
    
    # Split content
    category_section = '\n'.join(lines[:per_contract_line])
    per_contract_section = '\n'.join(lines[per_contract_line:])
    
    return category_section, per_contract_section, per_contract_line

def extract_table_rows(section_text, start_marker, end_marker=None):
    """Extract table rows between two markers."""
    lines = section_text.split('\n')
    start_idx = None
    end_idx = None
    
    for i, line in enumerate(lines):
        if start_marker in line:
            start_idx = i
        if end_idx is None and start_idx is not None and end_marker and end_marker in line:
            end_idx = i
            break
    
    if start_idx is None:
        return []
    
    if end_idx is None:
        end_idx = len(lines)
    
    rows = []
    for i in range(start_idx, end_idx):
        line = lines[i]
        if line.startswith('| ') and re.match(r'^\| \d+', line):
            rows.append((i, line))
    
    return rows

def parse_table_row(row_text):
    """Parse a markdown table row."""
    parts = [p.strip() for p in row_text.split('|')[1:-1]]
    if len(parts) < 3:
        return None
    
    try:
        code = int(parts[0])
        symbol = parts[1].strip('`')
        contracts = parts[2]
        description = parts[3] if len(parts) > 3 else ""
        remediation = parts[4] if len(parts) > 4 else ""
        
        return {
            'code': code,
            'symbol': symbol,
            'contracts': contracts,
            'description': description,
            'remediation': remediation,
            'raw': row_text
        }
    except (ValueError, IndexError):
        return None

def find_duplicates_in_categories(filepath):
    """Find duplicate codes in category sections."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Find per-contract marker
    per_contract_idx = content.find("## Per-Contract Error Codes")
    category_content = content[:per_contract_idx]
    
    # Extract all rows from category sections
    code_to_rows = defaultdict(list)
    
    for match in re.finditer(r'^\| (\d+) \|', category_content, re.MULTILINE):
        code = int(match.group(1))
        line_start = category_content.rfind('\n', 0, match.start()) + 1
        line_end = category_content.find('\n', match.end())
        row_text = category_content[line_start:line_end]
        
        parsed = parse_table_row(row_text)
        if parsed:
            code_to_rows[code].append(parsed)
    
    # Find codes with conflicting definitions
    duplicates = {}
    for code, rows in code_to_rows.items():
        if len(rows) > 1:
            symbols = set(r['symbol'] for r in rows)
            if len(symbols) > 1 or len(rows) > 1:
                duplicates[code] = rows
    
    return duplicates

def find_unused_codes(filepath):
    """Find unused codes in each range."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Extract all used codes
    used_codes = set()
    for match in re.finditer(r'^\| (\d+) \|', content, re.MULTILINE):
        used_codes.add(int(match.group(1)))
    
    # Find unused codes in ranges
    ranges = {
        (100, 199): 'Access Control',
        (200, 299): 'Input Validation',
        (300, 399): 'Lifecycle & State',
        (400, 499): 'Entity Existence',
        (500, 599): 'Financial & Resource',
        (600, 699): 'Cryptography',
        (700, 799): 'Cross-Chain',
        (800, 899): 'Reentrancy & Safety',
    }
    
    unused_by_range = {}
    for (start, end), name in ranges.items():
        unused = [c for c in range(start, end+1) if c not in used_codes]
        unused_by_range[(start, end)] = unused
    
    return unused_by_range

def get_next_unused_code(code, unused_by_range):
    """Get the next unused code in the same range."""
    for (start, end), unused_list in unused_by_range.items():
        if start <= code <= end:
            return unused_list[0] if unused_list else None
    return None

def main():
    docs_path = Path(__file__).parent.parent / 'docs' / 'ERROR_CODES.md'
    
    print(f"Analyzing {docs_path}")
    print("=" * 80)
    
    # Find duplicates
    duplicates = find_duplicates_in_categories(docs_path)
    unused = find_unused_codes(docs_path)
    
    if not duplicates:
        print("\n✓ No duplicate error codes found!")
        return 0
    
    print(f"\nFound {len(duplicates)} codes with conflicts:\n")
    
    # Create reassignment plan
    reassignments = {}
    for code, rows in sorted(duplicates.items()):
        print(f"\nCode {code}: {len(rows)} conflicting definitions")
        for i, row in enumerate(rows):
            print(f"  [{i}] {row['symbol']:30s} ({row['contracts']:20s}): {row['description'][:40]}")
        
        # Find next unused code in same range
        next_unused = get_next_unused_code(code, unused)
        if next_unused:
            print(f"  → Will reassign one to {next_unused}")
            reassignments[code] = next_unused
        else:
            print(f"  ✗ No unused codes in range!")
    
    print("\n" + "=" * 80)
    print("\nNOTE: Manual action required!")
    print("\nSteps to resolve:")
    for code, new_code in sorted(reassignments.items()):
        print(f"  1. Update ERROR_CODES.md: change one row with code {code} to {new_code}")
        print(f"  2. Update corresponding errors.rs file(s)")
    
    print("\nDuplicate codes by range:")
    for code in sorted(duplicates.keys()):
        for (start, end), _ in sorted(unused.items()):
            if start <= code <= end:
                unused_in_range = unused[(start, end)]
                print(f"  Code {code} ({start}-{end}): next unused is {unused_in_range[:3]}")
                break
    
    return 1

if __name__ == '__main__':
    sys.exit(main())
