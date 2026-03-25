# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
cargo build                             # Build the project
cargo test --no-fail-fast               # Run all tests
cargo +nightly fmt --all -- --check     # Check formatting
cargo clippy -- -Dwarnings              # Lint (CI treats warnings as errors)
```

Tests use temporary SQLite databases and `#[tokio::test]` for async support.

## Environment Variables

- `TELOXIDE_TOKEN` — Telegram bot API token
- `DATABASE_URL` — SQLite connection string (e.g. `sqlite:///path/to/db.sqlite`)

## Architecture

This is a Telegram bot that sends Italian "buongiornissimo" greeting images on command and on a cron schedule.

**Core flow:** `main.rs` → `Buongiornissimo::init()` (loads config from env, starts cron scheduler, connects to
SQLite) → `app.run()` (starts teloxide polling loop).

### Key modules under `src/bot/`

- **`bot.rs`** — Main `Buongiornissimo` struct with `init()`, `run()`, and `answer()` (command dispatcher). Holds a
  global `AUTOMATIZER` static via `OnceCell`.
- **`commands.rs`** — Teloxide `BotCommands` enum defining all slash commands.
- **`answer.rs`** — `AnswerBuilder` pattern for composing text + image responses and sending them via teloxide.
- **`providers.rs`** — Abstraction over multiple greeting image scrapers (from `buongiornissimo-rs` crate). Providers
  are tried in random order with fallback.
- **`automatize.rs`** — Cron-scheduled jobs (via `tokio-cron-scheduler`) that send greetings to all subscribed chats at
  fixed times throughout the day.
- **`config.rs`** — Config loaded from environment variables via `envy`.
- **`repository.rs`** — High-level database interface wrapping Chat and Birthday operations.

### Data layer (`src/repository/`)

- **`mod.rs`** — `SqliteDb`: connection pool, auto-creates `chat` and `birthday` tables on startup.
- **`chat.rs`** — Chat subscription entity (insert/delete/get_all).
- **`birthday.rs`** — Birthday tracking entity (insert/delete_by_chat/get_all).

## Code Style

- `rustfmt.toml` enforces `group_imports = "StdExternalCrate"` and `imports_granularity = "Module"`.
- CI runs clippy with `-Dwarnings` — all warnings must be fixed.
