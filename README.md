# LogBook

**A simple, offline-first task manager for your terminal.**

LogBook is a lightweight CLI application built with Rust and SQLite that helps you manage your todos directly from the terminal. Your tasks are stored locally, so your data stays on your machine and is available across sessions.

## ✨ Features

- **Add tasks** — Quickly create todos from your terminal.
- **List tasks** — View all your tasks with their completion status.
- **Complete tasks** — Mark tasks as done using their IDs.
- **Delete tasks** — Remove tasks you no longer need.
- **Local persistence** — Store tasks in an SQLite database.
- **Offline-first** — No account, cloud service, or internet connection required for everyday use.
- **Lightweight** — Built with Rust for a fast, native command-line experience.

## 🚀 Usage

### Add a task

```bash
lg add "Learn Rust"
lg add "Build LogBook"
```

### List all tasks

```bash
lg list
```

Example output:

```text
1. [Pending] Learn Rust
2. [Done] Build LogBook
```

### Mark a task as completed

Use the task ID shown by `lg list`.

```bash
lg done 1
```

### Delete a task

```bash
lg delete 2
```

### View help

```bash
lg --help
```

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) — Systems programming language
- [Clap](https://docs.rs/clap/latest/clap/) — Command-line argument parsing
- [SQLite](https://www.sqlite.org/) — Local database
- [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) — Rust bindings for SQLite

## 💾 Data Storage

LogBook stores tasks in a local SQLite database. Your tasks persist after you close the terminal.

The database location is determined by the application's configured data path.

## 📦 Installation

Homebrew distribution for macOS and Linux is planned.

Once published to Homebrew, the intended installation command will be:

```bash
brew install logbook
```

Then start managing tasks with:

```bash
lg add "My first task"
lg list
```

*Homebrew installation is not available until the package is published.*

## 🗺️ Roadmap

- [x] Add tasks
- [x] List tasks
- [x] Mark tasks as completed
- [x] Delete tasks
- [x] SQLite persistence
- [ ] Edit existing tasks
- [ ] Filter pending and completed tasks
- [ ] Add task priorities and due dates
- [ ] Publish Homebrew package for macOS and Linux

## 🤝 Contributing

Contributions, ideas, and bug reports are welcome!

1. Fork the repository.
2. Create a feature branch.
3. Make your changes and test them.
4. Open a pull request.

## 📄 License

This project is intended to be released under the MIT License. The license will apply once the repository includes the corresponding `LICENSE` file.

---

Built with Rust, one task at a time. 🦀
