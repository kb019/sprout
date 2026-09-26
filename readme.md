
<p align="center">
  A habit tracker that lives in your terminal.
</p>

<p align="center">
  <a href="https://ratatui.rs/">
    <img src="https://img.shields.io/badge/Built%20With-Ratatui-6E56CF?style=flat" alt="Built With Ratatui">
  </a>
  &nbsp;&nbsp;
  <img src="https://img.shields.io/badge/License-MIT-blue?style=flat" alt="License MIT">
  &nbsp;&nbsp;
  <img src="https://img.shields.io/badge/Rust-1.97.0-orange?style=flat" alt="Rust 1.97.0">
  &nbsp;&nbsp;
  <a href="https://crates.io/crates/sprout-tui">
    <img src="https://img.shields.io/crates/v/sprout-tui?style=flat" alt="crates.io">
  </a>
  &nbsp;&nbsp;
  <a href="https://github.com/kb019/sprout/actions/workflows/pages/pages-build-deployment">
    <img src="https://github.com/kb019/sprout/actions/workflows/pages/pages-build-deployment/badge.svg" alt="Docs">
  </a>
</p>

<h4 align="center">
  <a href="https://kb019.github.io/sprout/">Documentation</a>
</h4>

---

![demo](https://vhs.charm.sh/vhs-6qM6LED0L8C9Z9ymD2JRYb.gif)

---

## Features

- Full-screen TUI with dashboard, heatmap, stats, and settings screens
- GitHub-style activity heatmap with five intensity levels
- Current and best streak tracking
- Daily, weekly, monthly, and yearly goal tracking
- Five themes: Sprout, Amber, Mono, Ocean, Paper (light)
- CLI for logging and querying without opening the TUI
- Fully keyboard driven, no mouse required

---

## Install

```sh
cargo install sprout-tui
```

Or check the [full installation guide](https://kb019.github.io/sprout/#installation) for Homebrew, Scoop, and prebuilt binaries.

---

## Documentation

- [Installation](https://kb019.github.io/sprout/#installation)
- [CLI overview](https://kb019.github.io/sprout/#cli)
- [Launching the TUI](https://kb019.github.io/sprout/#launching)
- [Navigation](https://kb019.github.io/sprout/#navigation)
- [Keybind reference](https://kb019.github.io/sprout/#keybinds)
- [Settings reference](https://kb019.github.io/sprout/#settings-ref)
- [Data storage](https://kb019.github.io/sprout/#storage)

---

## Similar projects

- [dijo](https://github.com/oppiliappan/dijo) — scriptable, curses-based habit tracker written in Rust
- [habito](https://github.com/codito/habito) — simple CLI habit tracker written in Python
- [habu](https://github.com/schmee/habu) — TUI habit tracker

---

## Acknowledgements

- [bottom](https://github.com/ClementTsang/bottom/blob/6323bef6f0e304316b19845da6fcb3a764a5f5d7/src/lib.rs#L93) — referred approach for checking whether the output is a TTY before launching the TUI

---

## License

MIT License. See [LICENSE](LICENSE).
