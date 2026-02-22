<div align="center">

<h1>
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="logo_dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="logo_light.svg">
  <img alt="Helix" height="128" src="logo_light.svg">
</picture>
</h1>

[![Build status](https://github.com/helix-editor/helix/actions/workflows/build.yml/badge.svg)](https://github.com/helix-editor/helix/actions)
[![GitHub Release](https://img.shields.io/github/v/release/helix-editor/helix)](https://github.com/helix-editor/helix/releases/latest)
[![Documentation](https://shields.io/badge/-documentation-452859)](https://docs.helix-editor.com/)
[![GitHub contributors](https://img.shields.io/github/contributors/helix-editor/helix)](https://github.com/helix-editor/helix/graphs/contributors)
[![Matrix Space](https://img.shields.io/matrix/helix-community:matrix.org)](https://matrix.to/#/#helix-community:matrix.org)

</div>

![Screenshot](./screenshot.png)

A [Kakoune](https://github.com/mawww/kakoune) / [Neovim](https://github.com/neovim/neovim) inspired editor, written in Rust.

The editing model is very heavily based on Kakoune; during development I found
myself agreeing with most of Kakoune's design decisions.

For more information, see the [website](https://helix-editor.com) or
[documentation](https://docs.helix-editor.com/).

All shortcuts/keymaps can be found [in the documentation on the website](https://docs.helix-editor.com/keymap.html).

[Troubleshooting](https://github.com/helix-editor/helix/wiki/Troubleshooting)

# ⚠️ Personal Fork — Experimental Features

This branch (`personal`) is a personal integration branch with experimental features tailored to my own needs. These changes were made through vibe coding — no major architectural concerns were considered — the main goal was simply to solve problems I was running into in my day-to-day workflow. Use at your own risk.

## Added Features

### Inline Git Blame

Displays git blame information inline next to each line of code, similar to how VS Code and other editors show inline blame annotations. Supports both single-line (current cursor line) and all-lines modes, configurable display format with commit hash, author, date and message, and a blame picker for browsing the full blame output of a file.

### Tmux Integration

Adds seamless navigation between Helix splits and tmux panes. When you hit the edge of a Helix split, movement commands automatically pass through to tmux, allowing you to navigate between Helix and terminal panes without switching keybindings.

### Inactive View Styling

Adds a `ui.background.inactive` theme scope that lets you style unfocused splits differently from the active one. Useful for visually distinguishing which split you're currently working in.

### Buffer Navigation by ID *(WIP — not working as intended)*

Adds a `:buffer` (`:b`) typable command and a `space+b` submenu for quickly jumping to open buffers by their ID number. Makes it easy to switch between buffers without reaching for the buffer picker. **Note:** This feature still needs more work and is not functioning as intended.

### LSP Inline Completion (Ghost Text)

Adds support for the `textDocument/inlineCompletion` LSP method, rendering completions as ghost text (dimmed text ahead of the cursor, similar to Copilot-style suggestions). Supports multiple completion items with cycling, manual and auto-trigger modes, multi-line ghost text rendering, and proper handling of tabs and non-ASCII characters.

# Features

- Vim-like modal editing
- Multiple selections
- Built-in language server support
- Smart, incremental syntax highlighting and code editing via tree-sitter

Although it's primarily a terminal-based editor, I am interested in exploring
a custom renderer (similar to Emacs) using wgpu.

Note: Only certain languages have indentation definitions at the moment. Check
`runtime/queries/<lang>/` for `indents.scm`.

# Installation

[Installation documentation](https://docs.helix-editor.com/install.html).

[![Packaging status](https://repology.org/badge/vertical-allrepos/helix-editor.svg?exclude_unsupported=1)](https://repology.org/project/helix-editor/versions)

# Contributing

Contributing guidelines can be found [here](./docs/CONTRIBUTING.md).

# Getting help

Your question might already be answered on the [FAQ](https://github.com/helix-editor/helix/wiki/FAQ).

Discuss the project on the community [Matrix Space](https://matrix.to/#/#helix-community:matrix.org) (make sure to join `#helix-editor:matrix.org` if you're on a client that doesn't support Matrix Spaces yet).

# Credits

Thanks to [@jakenvac](https://github.com/jakenvac) for designing the logo!
