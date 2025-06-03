# emojis-rs

A collection of emoji constants for Rust applications, particularly useful for CLI tools and terminal applications that want to add visual flair to their output.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
emojis-rs = "0.1.2"
```

## Usage

### Using Constants Directly
```rust
use emojis_rs::*;

fn main() {
    println!("   {} Color set successfully.", EMOJI_CHECK);
    println!("   {} Failed to load configuration.", EMOJI_CROSS);
    println!("   {} Processing...", EMOJI_HOURGLASS);
    println!("   {} Task completed!", EMOJI_SUCCESS);
}
```

### Or
```rust
use emojis_rs::*;

fn main() {
    println!("{EMOJI_CHECK} Color set successfully.");
    println!("{EMOJI_CROSS} Failed to load configuration.");
    println!("{EMOJI_HOURGLASS} Processing {} files...", 42);
    println!("{EMOJI_SUCCESS} Task completed!");
}
```

## Available Emojis

### Status & Feedback
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_CHECK` | ✅ | Success/completion |
| `EMOJI_CROSS` | ❌ | Error/failure |
| `EMOJI_WARNING` | ⚠️ | Warning/caution |
| `EMOJI_INFO` | ℹ️ | Information |
| `EMOJI_SUCCESS` | 🎉 | Celebration/success |
| `EMOJI_ERROR` | 💥 | Error/explosion |
| `EMOJI_QUESTION` | ❓ | Question/unknown |
| `EMOJI_EXCLAMATION` | ❗ | Important/attention |
| `EMOJI_PROHIBITED` | 🚫 | Not allowed/prohibited |
| `EMOJI_WRONG_WAY` | ⛔ | Wrong direction |
| `EMOJI_HEAVY_CHECK` | ✔️ | Heavy check mark |
| `EMOJI_BALLOT_X` | ✗ | Ballot X |
| `EMOJI_THUMBS_UP` | 👍 | Approval/like |
| `EMOJI_THUMBS_DOWN` | 👎 | Disapproval/dislike |
| `EMOJI_OK_HAND` | 👌 | OK/perfect |

### Actions & Process
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_RELOAD` | 🔄 | Reload/refresh |
| `EMOJI_REFRESH` | 🔃 | Refresh clockwise |
| `EMOJI_HOURGLASS` | ⏳ | Processing/waiting |
| `EMOJI_HOURGLASS_DONE` | ⌛ | Completed process |
| `EMOJI_STOPWATCH` | ⏱️ | Timing/benchmark |
| `EMOJI_TIMER` | ⏲️ | Timer clock |
| `EMOJI_CLOCK` | 🕐 | Time/schedule |
| `EMOJI_PLAY` | ▶️ | Start/play |
| `EMOJI_PAUSE` | ⏸️ | Pause/stop |
| `EMOJI_STOP` | ⏹️ | Stop/halt |
| `EMOJI_FAST_FORWARD` | ⏩ | Fast forward |
| `EMOJI_REWIND` | ⏪ | Rewind/go back |
| `EMOJI_REPEAT` | 🔁 | Repeat/loop |
| `EMOJI_REPEAT_ONCE` | 🔂 | Repeat once |

### Performance & Speed
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_BOLT` | ⚡ | Fast/lightning speed |
| `EMOJI_ZAP` | ⚡ | Electric/energy |
| `EMOJI_FIRE` | 🔥 | Hot/trending |
| `EMOJI_ROCKET` | 🚀 | Launch/fast |
| `EMOJI_RACING_CAR` | 🏎️ | Speed/performance |
| `EMOJI_AIRPLANE` | ✈️ | Travel/deployment |
| `EMOJI_DASH` | 💨 | Speed/quick |
| `EMOJI_COMET` | ☄️ | Fast moving |

### Navigation & Discovery
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_TARGET` | 🎯 | Goal/target |
| `EMOJI_RADAR` | 📡 | Scanning/detection |
| `EMOJI_COMPASS` | 🧭 | Direction/navigation |
| `EMOJI_MAP` | 🗺️ | Map/location |
| `EMOJI_LOCATION` | 📍 | Pin/location |
| `EMOJI_PIN` | 📌 | Pin/mark |
| `EMOJI_EYE` | 👁️ | Watch/observe |
| `EMOJI_EYES` | 👀 | Looking/monitoring |
| `EMOJI_MAGNIFYING_GLASS` | 🔍 | Search/examine |
| `EMOJI_MAGNIFYING_GLASS_RIGHT` | 🔎 | Search right |
| `EMOJI_TELESCOPE` | 🔭 | Long range view |
| `EMOJI_MICROSCOPE` | 🔬 | Detailed analysis |

### Status Indicators & Colors
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_RED_CIRCLE` | 🔴 | Error/stop |
| `EMOJI_GREEN_CIRCLE` | 🟢 | Success/go |
| `EMOJI_YELLOW_CIRCLE` | 🟡 | Warning/caution |
| `EMOJI_BLUE_CIRCLE` | 🔵 | Information/neutral |
| `EMOJI_PURPLE_CIRCLE` | 🟣 | Special/custom |
| `EMOJI_BROWN_CIRCLE` | 🟤 | Neutral/earth |
| `EMOJI_BLACK_CIRCLE` | ⚫ | Off/disabled |
| `EMOJI_WHITE_CIRCLE` | ⚪ | Empty/available |
| `EMOJI_ORANGE_CIRCLE` | 🟠 | Alert/attention |
| `EMOJI_LARGE_RED_SQUARE` | 🟥 | Error block |
| `EMOJI_LARGE_GREEN_SQUARE` | 🟩 | Success block |
| `EMOJI_LARGE_YELLOW_SQUARE` | 🟨 | Warning block |
| `EMOJI_LARGE_BLUE_SQUARE` | 🟦 | Info block |
| `EMOJI_BLACK_SQUARE` | ⬛ | Solid block |
| `EMOJI_WHITE_SQUARE` | ⬜ | Empty block |

### Audio & Notifications
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_BELL` | 🔔 | Notification/alert |
| `EMOJI_BELL_SLASH` | 🔕 | Muted/no notifications |
| `EMOJI_MUTE` | 🔇 | Muted/silent |
| `EMOJI_SPEAKER_LOW` | 🔈 | Low volume |
| `EMOJI_SPEAKER_MEDIUM` | 🔉 | Medium volume |
| `EMOJI_SPEAKER_HIGH` | 🔊 | High volume |
| `EMOJI_MEGAPHONE` | 📢 | Announcement |
| `EMOJI_LOUDSPEAKER` | 📣 | Public announcement |
| `EMOJI_ALARM_CLOCK` | ⏰ | Alarm/reminder |

### System, AI & Technology
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_BRAIN` | 🧠 | AI/intelligence |
| `EMOJI_ROBOT` | 🤖 | Robot/automation |
| `EMOJI_GEAR` | ⚙️ | Settings/configuration |
| `EMOJI_WRENCH` | 🔧 | Tools/maintenance |
| `EMOJI_HAMMER` | 🔨 | Build/construction |
| `EMOJI_SCREWDRIVER` | 🪛 | Assembly/repair |
| `EMOJI_NUT_AND_BOLT` | 🔩 | Hardware/assembly |
| `EMOJI_LINK` | 🔗 | Connection/link |
| `EMOJI_CHAINS` | ⛓️ | Blockchain/connection |
| `EMOJI_ELECTRIC_PLUG` | 🔌 | Power/connection |
| `EMOJI_BATTERY` | 🔋 | Power/energy |
| `EMOJI_COMPUTER` | 💻 | Computing/laptop |
| `EMOJI_DESKTOP` | 🖥️ | Desktop computer |
| `EMOJI_KEYBOARD` | ⌨️ | Input/typing |
| `EMOJI_MOUSE` | 🖱️ | Mouse/cursor |
| `EMOJI_GAMEPAD` | 🎮 | Gaming/controls |

### Files, Data & Storage
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_FOLDER` | 📁 | Directory/folder |
| `EMOJI_FOLDER_OPEN` | 📂 | Open folder |
| `EMOJI_FILE` | 📄 | Document/file |
| `EMOJI_PAGE` | 📃 | Page/document |
| `EMOJI_DOCUMENT` | 📋 | Clipboard/document |
| `EMOJI_CLIPBOARD` | 📋 | Copy/paste |
| `EMOJI_CARD_INDEX` | 📇 | Index/catalog |
| `EMOJI_CARD_BOX` | 🗃️ | File box/storage |
| `EMOJI_FILE_CABINET` | 🗄️ | File cabinet |
| `EMOJI_WASTEBASKET` | 🗑️ | Delete/trash |
| `EMOJI_DATABASE` | 🗃️ | Database/storage |
| `EMOJI_FLOPPY_DISK` | 💾 | Save/storage |
| `EMOJI_HARD_DISK` | 💿 | Hard drive |
| `EMOJI_DVD` | 📀 | Optical disc |

### Communication & Social
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_CHAT` | 💬 | Chat/message |
| `EMOJI_SPEECH` | 💭 | Thought/idea |
| `EMOJI_MAIL` | ✉️ | Email/mail |
| `EMOJI_EMAIL` | 📧 | Email/electronic mail |
| `EMOJI_INBOX` | 📥 | Incoming/inbox |
| `EMOJI_OUTBOX` | 📤 | Outgoing/outbox |
| `EMOJI_PACKAGE` | 📦 | Package/delivery |
| `EMOJI_MAILBOX` | 📪 | Mailbox/mail |
| `EMOJI_TELEPHONE` | ☎️ | Phone/call |
| `EMOJI_MOBILE_PHONE` | 📱 | Mobile/smartphone |
| `EMOJI_SATELLITE` | 📡 | Satellite/communication |

### Security & Privacy
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_LOCK` | 🔒 | Locked/secure |
| `EMOJI_UNLOCK` | 🔓 | Unlocked/open |
| `EMOJI_LOCK_WITH_KEY` | 🔐 | Secure with key |
| `EMOJI_KEY` | 🔑 | Key/access |
| `EMOJI_OLD_KEY` | 🗝️ | Old key/legacy |
| `EMOJI_SHIELD` | 🛡️ | Protection/security |
| `EMOJI_DETECTIVE` | 🕵️ | Investigation/security |

### Network & Web
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_GLOBE` | 🌐 | Global/internet |
| `EMOJI_EARTH_AMERICAS` | 🌎 | Earth Americas |
| `EMOJI_EARTH_EUROPE` | 🌍 | Earth Europe/Africa |
| `EMOJI_EARTH_ASIA` | 🌏 | Earth Asia/Australia |
| `EMOJI_SIGNAL_STRENGTH` | 📶 | Signal/connectivity |

### Arrows & Directions
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_ARROW_UP` | ⬆️ | Up/increase |
| `EMOJI_ARROW_DOWN` | ⬇️ | Down/decrease |
| `EMOJI_ARROW_LEFT` | ⬅️ | Left/back |
| `EMOJI_ARROW_RIGHT` | ➡️ | Right/forward |
| `EMOJI_ARROW_UPPER_LEFT` | ↖️ | Upper left diagonal |
| `EMOJI_ARROW_UPPER_RIGHT` | ↗️ | Upper right diagonal |
| `EMOJI_ARROW_LOWER_LEFT` | ↙️ | Lower left diagonal |
| `EMOJI_ARROW_LOWER_RIGHT` | ↘️ | Lower right diagonal |
| `EMOJI_ARROW_UP_DOWN` | ↕️ | Vertical/bidirectional |
| `EMOJI_ARROW_LEFT_RIGHT` | ↔️ | Horizontal/bidirectional |

### Charts & Analytics
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_CHART_INCREASING` | 📈 | Growth/trending up |
| `EMOJI_CHART_DECREASING` | 📉 | Decline/trending down |
| `EMOJI_BAR_CHART` | 📊 | Statistics/data |
| `EMOJI_ABACUS` | 🧮 | Calculation/counting |
| `EMOJI_STRAIGHT_RULER` | 📏 | Measurement/length |
| `EMOJI_TRIANGULAR_RULER` | 📐 | Geometry/angles |

### Creative & Design
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_PAINT` | 🎨 | Art/design |
| `EMOJI_PAINTBRUSH` | 🖌️ | Painting/creativity |
| `EMOJI_CRAYON` | 🖍️ | Coloring/drawing |
| `EMOJI_PENCIL` | ✏️ | Writing/editing |
| `EMOJI_PEN` | 🖊️ | Writing/documentation |
| `EMOJI_FOUNTAIN_PEN` | 🖋️ | Formal writing |
| `EMOJI_MEMO` | 📝 | Notes/documentation |
| `EMOJI_BOOKMARK` | 🔖 | Bookmark/save |

### Ideas & Innovation
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_LAMP` | 💡 | Idea/inspiration |
| `EMOJI_CANDLE` | 🕯️ | Light/atmosphere |
| `EMOJI_FLASHLIGHT` | 🔦 | Illumination/search |
| `EMOJI_SPARKLES` | ✨ | Magic/special |
| `EMOJI_GLOWING_STAR` | 🌟 | Excellence/featured |
| `EMOJI_DIZZY` | 💫 | Dizzy/confused |
| `EMOJI_COLLISION` | 💥 | Impact/explosion |

### Achievement & Success
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_TROPHY` | 🏆 | Victory/achievement |
| `EMOJI_MEDAL` | 🏅 | Award/recognition |
| `EMOJI_FIRST_PLACE` | 🥇 | First place/gold |
| `EMOJI_SECOND_PLACE` | 🥈 | Second place/silver |
| `EMOJI_THIRD_PLACE` | 🥉 | Third place/bronze |
| `EMOJI_CROWN` | 👑 | Royalty/premium |
| `EMOJI_GEM` | 💎 | Valuable/precious |

### Nature & Weather
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_RAINBOW` | 🌈 | Diversity/colorful |
| `EMOJI_SUN` | ☀️ | Sunny/bright |
| `EMOJI_CLOUD` | ☁️ | Cloud/weather |
| `EMOJI_RAIN` | 🌧️ | Rain/wet |
| `EMOJI_SNOW` | ❄️ | Snow/cold |
| `EMOJI_LIGHTNING` | ⚡ | Lightning/power |
| `EMOJI_WAVE` | 🌊 | Wave/fluid |
| `EMOJI_DROPLET` | 💧 | Water/liquid |
| `EMOJI_MOUNTAIN` | ⛰️ | Mountain/challenge |
| `EMOJI_VOLCANO` | 🌋 | Volcano/eruption |

### Programming & Languages
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_RUST` | 🦀 | Rust programming |
| `EMOJI_SNAKE` | 🐍 | Python programming |
| `EMOJI_COFFEE` | ☕ | Java programming |
| `EMOJI_DIAMOND` | 💎 | Ruby programming |
| `EMOJI_ELEPHANT` | 🐘 | PHP programming |
| `EMOJI_BUG` | 🐛 | Bug/debugging |
| `EMOJI_ATOM` | ⚛️ | React/atomic |

### Numbers & Math
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_ZERO` | 0️⃣ | Number zero |
| `EMOJI_ONE` | 1️⃣ | Number one |
| `EMOJI_TWO` | 2️⃣ | Number two |
| `EMOJI_THREE` | 3️⃣ | Number three |
| `EMOJI_FOUR` | 4️⃣ | Number four |
| `EMOJI_FIVE` | 5️⃣ | Number five |
| `EMOJI_SIX` | 6️⃣ | Number six |
| `EMOJI_SEVEN` | 7️⃣ | Number seven |
| `EMOJI_EIGHT` | 8️⃣ | Number eight |
| `EMOJI_NINE` | 9️⃣ | Number nine |
| `EMOJI_TEN` | 🔟 | Number ten |
| `EMOJI_HASH` | #️⃣ | Hash/number sign |
| `EMOJI_PLUS` | ➕ | Addition/plus |
| `EMOJI_MINUS` | ➖ | Subtraction/minus |
| `EMOJI_MULTIPLY` | ✖️ | Multiplication |

### Misc & Fun
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_PARTY` | 🎉 | Celebration/party |
| `EMOJI_CONFETTI` | 🎊 | Confetti/celebration |
| `EMOJI_BALLOON` | 🎈 | Balloon/party |
| `EMOJI_GIFT` | 🎁 | Gift/present |
| `EMOJI_CLAP` | 👏 | Applause/appreciation |
| `EMOJI_MUSCLE` | 💪 | Strength/power |
| `EMOJI_PEACE` | ✌️ | Peace/victory |
| `EMOJI_HAND_WAVE` | 👋 | Hello/goodbye |
| `EMOJI_HANDSHAKE` | 🤝 | Agreement/partnership |
| `EMOJI_PRAY` | 🙏 | Thank you/please |
| `EMOJI_THINKING` | 🤔 | Thinking/pondering |
| `EMOJI_SHRUG` | 🤷 | Don't know/unsure |

### Special Symbols
| Constant | Emoji | Description |
|----------|--------|-------------|
| `EMOJI_RECYCLE` | ♻️ | Recycle/reuse |
| `EMOJI_TRIDENT` | 🔱 | Trident/power |
| `EMOJI_BEGINNER` | 🔰 | Beginner/new |
| `EMOJI_BALLOT_BOX_CHECK` | ☑️ | Checked/completed |
| `EMOJI_RADIO_BUTTON` | 🔘 | Radio button/option |
| `EMOJI_SMALL_BLUE_DIAMOND` | 🔹 | Small blue diamond |
| `EMOJI_SMALL_ORANGE_DIAMOND` | 🔸 | Small orange diamond |
| `EMOJI_LARGE_BLUE_DIAMOND` | 🔷 | Large blue diamond |
| `EMOJI_LARGE_ORANGE_DIAMOND` | 🔶 | Large orange diamond |

## Features

- **200+ emoji constants** - Comprehensive collection for CLI applications
- **Organized categories** - Emojis grouped by function and context
- **Zero dependencies** - No external dependencies required
- **Well documented** - Clear documentation with examples

## Terminal Compatibility

**Note**: Emoji rendering depends on your terminal and font support. Test in your target environment. *Works on my machine.*

## Examples

### CLI Progress Indicator
```rust
use emojis_rs::*;

fn main() {
    println!("{EMOJI_HOURGLASS} Starting backup process...");
    println!("{EMOJI_FOLDER} Scanning {} files", 1000);
    println!("{EMOJI_ARROW_RIGHT} Copying to backup location");
    println!("{EMOJI_CHECK} Backup completed successfully!");
    println!("{EMOJI_PARTY} All done! {EMOJI_THUMBS_UP}");
}
```
```
⏳ Starting backup process...
📁 Scanning 1000 files
➡️ Copying to backup location
✅ Backup completed successfully!
🎉 All done! 👍
```

### Status Dashboard
```rust
use emojis_rs::*;

fn show_status() {
    println!("{EMOJI_COMPUTER} System Status:");
    println!("  CPU: {EMOJI_GREEN_CIRCLE} 23%");
    println!("  Memory: {EMOJI_YELLOW_CIRCLE} 67%");
    println!("  Disk: {EMOJI_GREEN_CIRCLE} 45%");
    println!("  Network: {EMOJI_RED_CIRCLE} Offline");
}
```

```
💻 System Status:
  CPU: 🟢 23%
  Memory: 🟡 67%
  Disk: 🟢 45%
  Network: 🔴 Offline
```

### Build Output
```rust
use emojis_rs::*;

fn build_status() {
    println!("{EMOJI_RUST} Compiling Rust project...");
    println!("{EMOJI_GEAR} Running optimizations...");
    println!("{EMOJI_BOLT} Build completed in 2.3s");
    println!("{EMOJI_TROPHY} No warnings or errors!");
}
```

```
🦀 Compiling Rust project...
⚙️ Running optimizations...
⚡ Build completed in 2.3s
🏆 No warnings or errors!
```

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Adding New Emojis
1. Add the constant to `src/lib.rs`
3. Update this README with the new emoji
4. Add tests if needed

### Testing
```bash
cargo test
cargo test display_all_emojis -- --nocapture  # See all emojis in terminal
```
