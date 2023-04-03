# Stand Up!

A sedentary reminder written in Rust.

## Usage

```bash
# Make sure you have terminal-notifier installed on your system as well, as it's required for sending notifications on macOS. You can install it with Homebrew:
brew install terminal-notifier

# To compile binary file 
cargo build --release

# Run it in background.
nohup ./target/release/stand_up > /dev/null 2>&1 &
```
