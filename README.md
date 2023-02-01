# bib-picker

A simple fuzzy finder of your biblatex references

## Motivation

I cannot remember the key of the references.
This small program lets me search my bib file and yields the key of the selected reference.

## Usage

```bash
nix run github:GuilloteauQ/bib-picker [REFS].bib
# or
nix shell github:GuilloteauQ/bib-picker --command bp [REFS].bib
```

The key of the reference is then in the clipboard!