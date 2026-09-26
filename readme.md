
# 🌱 Sprout - TUI Habit Tracker

A habit tracker that lives in your terminal.

![GitHub License](https://img.shields.io/github/license/kb019/sprout?style=for-the-badge)
[![Crates.io Version](https://img.shields.io/crates/v/sprout-tui?style=for-the-badge)](https://crates.io/crates/sprout-tui)
[![Crates.io Downloads](https://img.shields.io/crates/d/sprout-tui?style=for-the-badge)](https://crates.io/crates/sprout-tui)

---

![demo](https://vhs.charm.sh/vhs-6qM6LED0L8C9Z9ymD2JRYb.gif)

---

## Why Sprout?

Most habit apps live on your phone — easy to ignore and full of distractions. The terminal is already open when you're working, so logging a habit takes seconds. All your data stays local, no accounts, no subscriptions.

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
