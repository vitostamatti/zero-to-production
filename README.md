## Setup Steps

- linker


- cargo-watch: 

```bash
cargo watch -x check -x test -x run
```

- cargo-tarpaulin: 

```bash
cargo tarpaulin --ignore-tests
```

- linting: 

```bash
rusup components add clippy
```

```bash
cargo clippy
```

```bash
cargo clippy -- -D warnings
```

- formatting:
```bash
rustup component add rustfmt
```

```bash
cargo fmt
```

```bash
cargo fmt --check
```

- vulnerabilities:

```bash
cargo install cargo-audit
```

```bash
cargo audit
```


# Scripts

```bash
chmod +x scripts/init_db.sh
```


# Migrations

```bash
sqlx migrate add create_subscriptions_table
```
