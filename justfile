# just manual: https://github.com/casey/just/#readme

_default:
  @just --list

# Runs clippy on the project
check:
  cargo clippy --locked -- -D warnings

# Runs project in watch mode
dev:
  cargo watch -x "run"

# Runs project tests
test:
  cargo test --locked

# Runs coverage report
coverage:
  cargo llvm-cov
