dev:
  cargo watch -x "run"

test:
  cargo test

coverage:
  cargo llvm-cov
