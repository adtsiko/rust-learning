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

### [train-booking-system](./train-booking-system)

A REST API for booking train seats. A train has 4 wagons with 50 seats each. Seat availability is persisted to a JSON file.

**Architecture**

```
handlers → service → repository → models
```

**Run**

```bash
cd train-booking-system
cargo run
```

Server listens on `http://127.0.0.1:3001`.

**Book a seat**

```bash
curl -X POST http://127.0.0.1:3001/book \
  -H "Content-Type: application/json" \
  -d '{
    "username": "Noa",
    "email": "noa@example.com",
    "requested_seats": [{"wagon_number": 1, "seat_number": 2}]
  }'
```

**Error responses**

| Status | Meaning |
|--------|---------|
| 409 | Seat already booked |
| 400 | Invalid seat or wagon number |
| 500 | Internal / I/O error |

**Test**

```bash
cd train-booking-system
cargo test
```

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
