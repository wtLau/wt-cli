# Brian's CLI

## How to Release and Run the CLI Locally

1. Build a release binary

```bash
cargo build --release
```

2.  Run the binary locally

```bash
./target/release/wt list
```

3. (Optional) Install globally via Cargo

```bash
cargo install --path .
```

Installs the CLI into ~/.cargo/bin for running wt anywhere.

4. (Optional) Move binary to system path manually

```bash
sudo cp target/release/wt /usr/local/bin/
```
