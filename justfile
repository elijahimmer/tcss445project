run:
  cargo fmt
  cargo run -F debug,dev

build:
  cargo fmt
  cargo build -F debug,dev

release:
  cargo build --release

clean:
  cargo clean
