# emojis-rs

A collection of emoji constants for Rust applications, particularly useful for CLI tools and terminal applications that want to add visual flair to their output.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
emoji-rs = "0.1.0"
```

## Usage

```rust
use emoji_rs::*;

fn main() {
    println!("   {EMOJI_CHECK} Color set successfully.");
    println!("   {EMOJI_CROSS} Failed to load configuration.");
    println!("   {EMOJI_HOURGLASS} Processing...");
    println!("   {EMOJI_SUCCESS} Task completed!");
}
```

## Available Emojis

### Status
- `EMOJI_CHECK` ✅
- `EMOJI_CROSS` ❌
- `EMOJI_WARNING` ⚠️
- `EMOJI_INFO` ℹ️
- `EMOJI_SUCCESS` 🎉
- `EMOJI_WRONG_WAY` ⛔

### Actions
- `EMOJI_RELOAD` 🔄
- `EMOJI_HOURGLASS` ⏳
- `EMOJI_STOPWATCH` ⏱️
- `EMOJI_SPARKLES` ✨
- `EMOJI_WRENCH` 🔧
- `EMOJI_BOLT` ⚡

### Indicators
- `EMOJI_TARGET` 🎯
- `EMOJI_RADAR` 📡
- `EMOJI_EYE` 👁️
- `EMOJI_MAGNIFYING_GLASS` 🔍
- `EMOJI_PAINT` 🎨
- `EMOJI_LAMP` 💡

### Color Indicators
- `EMOJI_RED_CIRCLE` 🔴
- `EMOJI_GREEN_CIRCLE` 🟢
- `EMOJI_YELLOW_CIRCLE` 🟡
- `EMOJI_BLUE_CIRCLE` 🔵

### Sound / Notification
- `EMOJI_BELL` 🔔
- `EMOJI_MUTE` 🔇

### System / AI
- `EMOJI_BRAIN` 🧠
- `EMOJI_ROBOT` 🤖

### File / Data
- `EMOJI_FOLDER` 📁
- `EMOJI_FILE` 📄
- `EMOJI_DATABASE` 🗃️

### Communication
- `EMOJI_CHAT` 💬
- `EMOJI_MAIL` ✉️

### Security
- `EMOJI_LOCK` 🔒
- `EMOJI_UNLOCK` 🔓

### Miscellaneous
- `EMOJI_RAINBOW` 🌈
- `EMOJI_WAVE` 🌊
- `EMOJI_GAMEPAD` 🎮
- `EMOJI_PUFF` 💨
- `EMOJI_STARS` ✨
- `EMOJI_RUST` 🦀

## License

Licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
