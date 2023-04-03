# Stand Up!

A sedentary reminder written in Rust.

## Usage

```bash
# Make sure you have terminal-notifier installed on your system, as it's required for sending notifications on macOS. 
# You can install it with Homebrew:
brew install terminal-notifier

# To compile the package.
cargo build --release

# Running in the background.
nohup ./target/release/stand_up > /dev/null 2>&1 &
```
