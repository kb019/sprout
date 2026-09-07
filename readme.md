
<p align="center">
  <img src="assets/sprout_logo_plant.png" width="40%" alt="Description of image">
</p>

<p align="center">
  <a href="https://ratatui.rs/">
    <img
      src="https://img.shields.io/badge/Built%20With-Ratatui-6E56CF?style=flat"
      alt="Built With Ratatui"
    >
  </a>
</p>

# sprout

A habit tracker that lives in your terminal.

Track daily habits, visualise streaks, and view GitHub-style activity heatmaps inside a full-screen TUI. A CLI is also included for quick logging and scripting without opening the interface. No background daemon, no cloud sync. A single binary and a SQLite file.

---

## Features

- Full-screen TUI with dashboard, heatmap, stats, and settings screens
- GitHub-style activity heatmap with five intensity levels
- Current and best streak tracking
- Daily, weekly, monthly, and yearly goal tracking
- Three accent themes: Sprout, Amber, Mono
- CLI for logging and querying without opening the TUI

---

## Installation

Requires Rust 1.80 or later.

```
git clone https://github.com/<your-repo>/sprout
cd sprout
cargo build --release
```

Copy the binary to your PATH:

```
# Linux / macOS
cp target/release/sprout ~/.local/bin/

# Windows (PowerShell)
Copy-Item target\release\sprout.exe $env:USERPROFILE\bin\
```

The database file `habit.db` is created automatically next to the binary on first run.

---

## Usage

```
sprout                                               # open the TUI
sprout list                                          # list all habits with progress
sprout status                                        # today's completion summary
sprout streak [name]                                 # current and best streaks
sprout heatmap <name> [--year YYYY]                  # activity heatmap
sprout add --name <name> [--daily-goal <n>]          # create a habit
sprout log <name> [value]                            # record progress
sprout edit <name> [--rename <n>] [--daily-goal <n>] # update a habit
sprout delete <name> [--yes]                         # delete a habit and its history
sprout sample                                        # open TUI with demo data
```

All name arguments are case-insensitive. Run `sprout --help` or `sprout <command> --help` for the full flag reference.

---

## Documentation

See [`docs/index.html`](docs/index.html) for the full documentation including keybind reference, settings, and data storage details.

---

## Acknowledgements

- [bottom](https://github.com/ClementTsang/bottom) — referred approach for checking whether the output is a TTY before launching the TUI
- [openai/codex](https://github.com/openai/codex/blob/1fb5158b3496a05abb89fb992d45737a02511d47/codex-rs/tui/src/chatwidget/tokens/chart.rs) — reference for the heatmap cli implementation

---

## License

MIT License. See [LICENSE](LICENSE).
