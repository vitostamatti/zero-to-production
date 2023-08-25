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
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

```bash
sqlx migrate add create_subscriptions_table
```


# API Request

```bash
curl -i -X POST -d \
'email=thomas_mann@hotmail.com&name=Tom' \
http://127.0.0.1:8000/subscriptions    
```

```bash
docker build --tag zero2prod --file Dockerfile .
```