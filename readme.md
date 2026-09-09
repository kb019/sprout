
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

Sprout is a habit tracker for the terminal. It has a full-screen TUI for daily use and a CLI for quick logging without opening the interface. Just a binary and a SQLite file.

---

## Features

- Fully keyboard driven, no mouse required
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

See [`docs`](https://kb019.github.io/sprout/) for the full documentation including keybind reference, settings, and data storage details.

---

## Acknowledgements

- [bottom](https://github.com/ClementTsang/bottom/blob/6323bef6f0e304316b19845da6fcb3a764a5f5d7/src/lib.rs#L93) — referred approach for checking whether the output is a TTY before launching the TUI

---

## License

MIT License. See [LICENSE](LICENSE).
