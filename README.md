# rust-learning

Small Rust projects built while learning the language.

## Projects

### [number-guesser](./number-guesser)

A CLI number guessing game. Pick a max value (or press Enter for 100), then guess the secret number between 1 and that max. The game tells you to guess higher or lower until you get it right.

**Run**

```bash
cd number-guesser
cargo run
```

**Test**

```bash
cd number-guesser
cargo test
```

---

### [train-booking-api](https://github.com/adtsiko/train-booking-api)

REST API for booking train seats — moved to its own repo for further development.

---

### [leaderboard-api](./leaderboard-api)

A REST API for managing game scores. Add scores, list all, and get the top N.

**Run**

```bash
cd leaderboard-api
cargo run
```

Server listens on `http://127.0.0.1:3000`.

**Endpoints**

- `POST /scores` — add a score
- `GET /scores` — list all scores
- `GET /scores/top?limit=10` — top N scores

---

### [leaderboard-cli](./leaderboard-cli)

A CLI client for managing high scores locally with JSON persistence.

**Run**

```bash
cd leaderboard-cli
cargo run -- add Alice 42
cargo run -- list
cargo run -- top 10
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (Cargo included)
