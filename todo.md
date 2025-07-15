# ✅ TODOs

## 🧩 Config File Handling

- [ ] Ensure the config path works both in dev and when installed
  - [ ] Default to `~/.config/mycli/aliases.toml`
  - [ ] Allow override via `--config` CLI flag or `MYCLI_CONFIG` env var
  - [ ] Fallback to `config/aliases.toml` in dev if no other found

## 🤔 Investigate Patterns

- [ ] Research how other Rust CLI apps handle config paths
  - [ ] Look into `ripgrep`, `bat`, `cargo`, etc.
  - [ ] Learn about common fallback/load patterns

## 🛠 Implementation Tasks

- [ ] Add `config.rs` module to encapsulate config logic
- [ ] Add graceful error if config file is missing
- [ ] Add example `aliases.toml` in `config/` folder for dev

## 🧪 Testing

- [ ] Test running CLI with and without a config file
- [ ] Test `--config` override
- [ ] Test with env var `MYCLI_CONFIG`
