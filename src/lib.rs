//! # emoji-rs
//!
//! A collection of emoji constants for Rust applications, particularly useful for CLI tools
//! and terminal applications that want to add visual flair to their output.
//!
//! ## Usage
//!
//! ```rust
//! use emojis_rs::*;
//!
//! println!("   {EMOJI_CHECK} Color set successfully.");
//! println!("   {EMOJI_CROSS} Failed to load configuration.");
//! println!("   {EMOJI_HOURGLASS} Processing...");
//! ```

// ✅ Status
pub const EMOJI_CHECK: &str = "✅";
pub const EMOJI_CROSS: &str = "❌";
pub const EMOJI_WARNING: &str = "⚠️";
pub const EMOJI_INFO: &str = "ℹ️";
pub const EMOJI_SUCCESS: &str = "🎉";
pub const EMOJI_WRONG_WAY: &str = "⛔";

// 🔄 Actions
pub const EMOJI_RELOAD: &str = "🔄";
pub const EMOJI_HOURGLASS: &str = "⏳";
pub const EMOJI_STOPWATCH: &str = "⏱️";
pub const EMOJI_SPARKLES: &str = "✨";
pub const EMOJI_WRENCH: &str = "🔧";
pub const EMOJI_BOLT: &str = "⚡";

// 🎯 Indicators
pub const EMOJI_TARGET: &str = "🎯";
pub const EMOJI_RADAR: &str = "📡";
pub const EMOJI_EYE: &str = "👁️";
pub const EMOJI_MAGNIFYING_GLASS: &str = "🔍";
pub const EMOJI_PAINT: &str = "🎨";
pub const EMOJI_LAMP: &str = "💡";

// 🔴 Color indicators
pub const EMOJI_RED_CIRCLE: &str = "🔴";
pub const EMOJI_GREEN_CIRCLE: &str = "🟢";
pub const EMOJI_YELLOW_CIRCLE: &str = "🟡";
pub const EMOJI_BLUE_CIRCLE: &str = "🔵";

// 🔊 Sound / Notification
pub const EMOJI_BELL: &str = "🔔";
pub const EMOJI_MUTE: &str = "🔇";

// 🧠 System / AI
pub const EMOJI_BRAIN: &str = "🧠";
pub const EMOJI_ROBOT: &str = "🤖";

// 🗂️ File / Data
pub const EMOJI_FOLDER: &str = "📁";
pub const EMOJI_FILE: &str = "📄";
pub const EMOJI_DATABASE: &str = "🗃️";

// 💬 Communication
pub const EMOJI_CHAT: &str = "💬";
pub const EMOJI_MAIL: &str = "✉️";

// 🔒 Security
pub const EMOJI_LOCK: &str = "🔒";
pub const EMOJI_UNLOCK: &str = "🔓";

// 🌈 Misc
pub const EMOJI_RAINBOW: &str = "🌈";
pub const EMOJI_WAVE: &str = "🌊";
pub const EMOJI_GAMEPAD: &str = "🎮";
pub const EMOJI_PUFF: &str = "💨";
pub const EMOJI_STARS: &str = "✨";
pub const EMOJI_RUST: &str = "🦀";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emojis_constants() {
        assert_eq!(EMOJI_CHECK, "✅");
        assert_eq!(EMOJI_CROSS, "❌");
        assert_eq!(EMOJI_RUST, "🦀");
    }

    #[test]
    fn test_emoji_display() {
        let message = format!("{EMOJI_CHECK} Test passed!");
        assert!(message.contains("✅"));
    }
}
