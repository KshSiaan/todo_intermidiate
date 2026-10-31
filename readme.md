# 📋 Rust Todo CLI

A simple terminal to-do app written in Rust. Tasks are saved to a local JSON file, so they're still there the next time you run it.

## Features

- Add a TODO with a title
- List all TODOs with a checkbox, strikethrough for finished ones, and a progress bar
- Mark a TODO as done
- Data persists in `data.json` (created automatically on first run)

## Preview

```
📋 Your Todos
────────────────────────────────────────
  1. ✔ Learn ownership
  2. ☐ Add file saving
  3. ☐ Try clap
────────────────────────────────────────
  [██████░░░░░░░░░░░░░░] 1/3 done
```

## Built with

| Crate | Used for |
|-------|----------|
| [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) | Converting the `TODO` struct to and from JSON |
| [`chrono`](https://crates.io/crates/chrono) | `created_at` / `updated_at` timestamps |
| [`nanoid`](https://crates.io/crates/nanoid) | Generating a unique ID for each TODO |
| `std::fs`, `std::path`, `std::io` | Reading and writing the data file, taking user input |

Terminal colors are plain ANSI escape codes, so no extra crate is needed.

## Getting started

```bash
git clone <your-repo-url>
cd <your-repo-name>
cargo run
```

Dependencies in `Cargo.toml`:

```toml
[dependencies]
chrono = "0.4"
nanoid = "0.4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Usage

```
1. Add TODO
2. List TODOs
3. Mark TODO as done
4. Exit
```

Type a number and press Enter. Choosing **3** asks for the TODO's `id`, which you can find in `data.json`.

## What I learned

- **Structs and derive macros**: `#[derive(Serialize, Deserialize, Debug)]` gives a struct JSON support with almost no code.
- **Reading user input**: `stdin().read_line()` appends to a `String`, so I need `.trim()` to remove the newline.
- **File I/O**: reading and writing with `fs::read_to_string` and `fs::write`, and creating the file on first run if it doesn't exist.
- **Handling `Result`**: using `match` on `Ok` / `Err`, `.expect()` for errors that should crash, and `unwrap_or_default()` for graceful fallbacks.
- **Borrowing with slices**: `print_todos(todos: &[TODO])` borrows the list instead of taking ownership, so it can be used again afterward.
- **Iterators**: `.iter()`, `.enumerate()`, `.filter().count()`, and `.position()` to loop over, count, and search the list.
- **Constants and string formatting**: inline format args like `{done}/{total}`, width formatting like `{:>width$}`, and ANSI codes for color.
- **Shadowing**: re-declaring a variable with `let` inside a block hides the outer one (this caused a real bug, see below).

## Known limitations and next steps

- [ ] Marking a TODO as done should update the in-memory list immediately
- [ ] Select TODOs by list number instead of by `id`
- [ ] Handle an invalid `id` without panicking
- [ ] Update `updated_at` when a TODO changes
- [ ] Add delete and edit
- [ ] Switch to `clap` for command-line arguments
- [ ] Build a full TUI with [`ratatui`](https://ratatui.rs)

## License

MIT