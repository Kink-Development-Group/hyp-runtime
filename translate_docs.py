#!/usr/bin/env python3
"""
Comprehensive German to English translation for HypnoScript documentation.
This script performs sentence-by-sentence translation while preserving code blocks,
formatting, links, and technical terms.
"""

import os
import re
from pathlib import Path
from typing import List, Tuple

# Comprehensive translation dictionary
TRANSLATIONS = {
    # Common full sentences
    "Die folgenden Funktionen sind in der": "The following functions are available in the",
    "verfügbar": "available",
    "Bibliothek verfügbar": "library",
    
    # Headers and common titles
    "Übersicht": "Overview",
    "Verfügbare Funktionen": "Available Functions",
    "Grundlegende": "Basic",
    "Erweiterte": "Advanced",
    "Beispiele": "Examples",
    "Verwendung": "Usage",
    "Beschreibung": "Description",
    "Parameter": "Parameters",
    "Rückgabewert": "Return Value",
    "Rückgabe": "Returns",
   
    # Technical terms
    "Berechnet": "Calculates",
    "Gibt": "Returns",
    "zurück": "",
    "Prüft": "Checks",
    "Konvertiert": "Converts",
    "Erstellt": "Creates",
    "Liest": "Reads",
    "Schreibt": "Writes",
    "Findet": "Finds",
    "Sucht": "Searches",
    "Extrahiert": "Extracts",
    "Verkettet": "Concatenates",
    "Macht": "Makes",
    "Zeigt": "Shows",
    "Führt": "Executes",
    "Tokenisiert": "Tokenizes",
    "Generiert": "Generates",
    "Listet": "Lists",
    "Lade": "Load",
    "Entpacke": "Extract",
    "Füge": "Add",
    "hinzu": "",
    "Nutze": "Use",
    "Kopieren": "Copy",
    "Ergänze": "Add",
    
    # Common phrases
    "aus den": "from the",
    "das passende Archiv": "the appropriate archive",
    "den Binärpfad deiner": "the binary path to your",
    "Umgebungsvariable": "environment variable",
    "die Installation mit": "the installation with",
    "Die kompilierten Binaries findest du unter": "You'll find the compiled binaries under",
    "Alle Subcommands sind bewusst schlank gehalten": "All subcommands are intentionally kept lean",
    "Für einen tieferen Blick sieh dir die folgenden Abschnitte an": "For a deeper look, check out the following sections",
    "Weitere Details liefert die Seite": "Further details are provided on the page",
    "ohne Ausführung": "without execution",
    "Bei Fehlern": "If there are errors",
    "aktivieren": "enable",
    "verschafft dir einen schnellen Überblick über": "gives you a quick overview of",
    "die Standardbibliothek": "the standard library",
    
    # Table headers
    "Befehl": "Command",
    "Kurzbeschreibung": "Brief Description",
    "Kategorie": "Category",
    "Funktion": "Function",
    "Funktionen": "Functions",
    
    # File/system terms
    "Datei": "file",
    "Dateien": "files",
    "Verzeichnis": "directory",
    "Verzeichnisse": "directories",
    "Ordner": "folder",
    
    # Common code comments
    "Hilfe anzeigen": "Show help",
    "Versionshinweis": "Version information",
    "Programm ausführen": "Run a program",
    "Optional installieren": "Optionally install",
    "Type Checking": "Type checking",
    "AST prüfen": "Check AST",
    "Debug-Ausgabe": "Debug output",
    "WASM generieren": "Generate WASM",
    
    # Specific phrases in examples
    "Willkommen bei HypnoScript": "Welcome to HypnoScript",
    "Hallo Welt": "Hello World",
    "Hallo": "Hello",
    "Entwickler": "Developer",
    "Summe": "Sum",
    "Die Erinnerung wird jetzt intensiver": "The memory is now becoming more intense",
}

# German patterns that need more context-aware translation
SENTENCE_PATTERNS = [
    (r"Die ([\w\s]+) ist", r"The \1 is"),
    (r"Das ([\w\s]+) ist", r"The \1 is"),
    (r"Der ([\w\s]+) ist", r"The \1 is"),
    (r"Ein ([\w\s]+) ist", r"A \1 is"),
    (r"Eine ([\w\s]+) ist", r"A \1 is"),
]


def is_code_line(line: str) -> bool:
    """Check if a line is part of a code block marker."""
    return line.strip().startswith('```')


def translate_line(line: str, in_code_block: bool) -> str:
    """Translate a single line while preserving structure."""
    if in_code_block or is_code_line(line):
        return line
    
    # Don't translate if line is just a header marker, link, or special syntax
    if line.strip().startswith('#') and '(' not in line:
        # Translate header text
        for de, en in TRANSLATIONS.items():
            line = line.replace(de, en)
        return line
    
    # Don't translate URLs, code snippets, or technical markers
    if any(marker in line for marker in ['http://', 'https://', '`', 'github.com', '.hyp', '.md']):
        # Only translate parts outside of links and code
        parts = re.split(r'(\[.*?\]\(.*?\)|`.*?`|https?://\S+)', line)
        translated_parts = []
        for part in parts:
            if part.startswith('[') or part.startswith('`') or part.startswith('http'):
                translated_parts.append(part)
            else:
                translated_part = part
                for de, en in TRANSLATIONS.items():
                    if de in translated_part:
                        translated_part = translated_part.replace(de, en)
                translated_parts.append(translated_part)
        return ''.join(translated_parts)
    
    # Regular translation
    translated = line
    for de, en in TRANSLATIONS.items():
        if de in translated:
            translated = translated.replace(de, en)
    
    # Apply pattern-based translations
    for pattern, replacement in SENTENCE_PATTERNS:
        translated = re.sub(pattern, replacement, translated)
    
    return translated


def translate_markdown_file(filepath: Path) -> str:
    """Translate a markdown file from German to English."""
    print(f"Translating: {filepath.name}...", end=" ")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"ERROR reading: {e}")
        return None
    
    translated_lines = []
    in_code_block = False
    
    for line in lines:
        # Track code blocks
        if line.strip().startswith('```'):
            in_code_block = not in_code_block
            translated_lines.append(line)
            continue
        
        # Translate line
        translated_line = translate_line(line, in_code_block)
        translated_lines.append(translated_line)
    
    print("✓")
    return ''.join(translated_lines)


def process_directory(directory: Path, dry_run: bool = False):
    """Process all markdown files in a directory."""
    if not directory.exists():
        print(f"Directory not found: {directory}")
        return 0
    
    md_files = sorted(directory.glob('*.md'))
    if not md_files:
        return 0
    
    print(f"\n{'[DRY RUN] ' if dry_run else ''}Processing {len(md_files)} files in {directory.name}/")
    
    success_count = 0
    for md_file in md_files:
        translated_content = translate_markdown_file(md_file)
        
        if translated_content and not dry_run:
            try:
                with open(md_file, 'w', encoding='utf-8') as f:
                    f.write(translated_content)
                success_count += 1
            except Exception as e:
                print(f"  ERROR writing {md_file.name}: {e}")
        elif translated_content:
            success_count += 1
    
    return success_count


def main():
    """Main translation function."""
    import argparse
    
    parser = argparse.ArgumentParser(description='Translate HypnoScript docs from German to English')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be translated without writing')
    args = parser.parse_args()
    
    base_dir = Path(__file__).parent / "hypnoscript-docs" / "docs"
    
    if not base_dir.exists():
        print(f"ERROR: Documentation directory not found: {base_dir}")
        return 1
    
    # List of directories to translate
    directories = [
        base_dir / "builtins",
        base_dir / "cli",
        base_dir / "debugging",
        base_dir / "development",
        base_dir / "error-handling",
        base_dir / "examples",
        base_dir / "reference",
        base_dir / "tutorial-basics",
        base_dir / "tutorial-extras",
    ]
    
    # Root files
    root_files = [
        base_dir / "index.md",
        base_dir / "intro.md",
    ]
    
    print("=" * 70)
    print("HypnoScript Documentation Translation (German → English)")
    print("=" * 70)
    
    if args.dry_run:
        print("\n⚠️  DRY RUN MODE - No files will be modified\n")
    
    total_files = 0
    
    # Process root files
    print("\nProcessing root files...")
    for file in root_files:
        if file.exists():
            content = translate_markdown_file(file)
            if content and not args.dry_run:
                try:
                    with open(file, 'w', encoding='utf-8') as f:
                        f.write(content)
                    total_files += 1
                except Exception as e:
                    print(f"  ERROR writing {file.name}: {e}")
            elif content:
                total_files += 1
    
    # Process directories
    for directory in directories:
        count = process_directory(directory, args.dry_run)
        total_files += count
    
    print("\n" + "=" * 70)
    if args.dry_run:
        print(f"Would translate {total_files} files")
    else:
        print(f"Successfully translated {total_files} files")
    print("=" * 70)
    
    return 0


if __name__ == "__main__":
    exit(main())
