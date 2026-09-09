
# Contributing to Sprout

Thank you for taking the time to contribute. Every bug report, suggestion, and pull request makes Sprout better, and it is genuinely appreciated.

I started this project as a way to learn Rust, so there are probably rough edges and bugs I haven't caught yet. If you run into something, please feel free to open an issue.

---

## Getting started

```
git clone https://github.com/kb019/habit_tracker_tui
cd habit-tracker-tui
just setup
```

Run the TUI:

```
cargo run
```

Run with sample data:

```
cargo run -- sample
```

---

## Submitting changes

- Fork the repo and create a branch from `main`
- Keep changes focused -- one fix or feature per pull request
- Commit messages must follow [Conventional Commits](https://www.conventionalcommits.org/) format, e.g. `fix: correct heatmap legend for binary habits`
- Run `just lint` before opening a PR
- Open the pull request against `main`

---

## Commit types

| Type       | When to use                        |
|------------|------------------------------------|
| `feat`     | New feature                        |
| `fix`      | Bug fix                            |
| `docs`     | Documentation only                 |
| `refactor` | Code change with no behavior change|
| `chore`    | Tooling, dependencies, config      |
| `style`    | Formatting only                    |
| `perf`     | Performance improvement            |
| `test`     | Adding or updating tests           |

---

## Reporting bugs

Open an issue with steps to reproduce, what you expected, and what actually happened. Including your OS and terminal emulator helps a lot.

---

## License

By contributing, you agree that your changes will be licensed under the [MIT License](LICENSE).
