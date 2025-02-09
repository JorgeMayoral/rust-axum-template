# just manual: https://github.com/casey/just/#readme

export PORT := "8000"

_default:
  @just --list

# Runs clippy on the project
check:
  cargo clippy --locked -- -D warnings

# Runs the project
run:
  cargo run

# Runs project in watch mode
dev:
  cargo watch -x "run"

# Runs project tests
test:
  cargo test --locked

# Runs coverage report
coverage:
  cargo llvm-cov
