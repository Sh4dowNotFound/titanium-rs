//! UI Utilities for creating rich embeds and messages.

/// A text-based progress bar generator.
#[derive(Debug, Clone)]
pub struct ProgressBar {
    /// Total number of steps.
    pub length: u32,
    /// Character for the filled portion.
    pub filled_char: char,
    /// Character for the empty portion.
    pub empty_char: char,
    /// Character for the current position (head).
    pub head_char: Option<char>,
    /// Opening bracket/character.
    pub start_char: Option<String>,
    /// Closing bracket/character.
    pub end_char: Option<String>,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            length: 10,
            filled_char: '▬',
            empty_char: '▬',
            head_char: Some('🔘'),
            start_char: None,
            end_char: None,
        }
    }
}

impl ProgressBar {
    /// Create a new default progress bar with specified length.
    #[must_use]
    pub fn new(length: u32) -> Self {
        Self {
            length,
            ..Default::default()
        }
    }

    /// Create a new "Pac-Man" style progress bar (Arch Linux style).
    /// e.g. [------C o o o]
    #[must_use]
    pub fn pacman(length: u32) -> Self {
        Self {
            length,
            filled_char: '-',     // Eaten path
            empty_char: 'o',      // Pellets
            head_char: Some('C'), // Pac-Man
            start_char: Some("[".to_string()),
            end_char: Some("]".to_string()),
        }
    }

    /// Generate the progress bar string.
    ///
    /// # Arguments
    /// * `percent` - A value between 0.0 and 1.0.
    #[must_use]
    #[inline]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub fn create(&self, percent: f32) -> String {
        let percent = percent.clamp(0.0, 1.0);
        let filled_count = (self.length as f32 * percent).round() as u32;
        let filled_count = filled_count.min(self.length); // clamp to max length

        // Calculate exact capacity to avoid re-allocation
        // Base length + overhead for start/end/head chars (approximate but sufficient)
        let capacity = (self.length as usize * 4) + 16;
        let mut result = String::with_capacity(capacity);

        if let Some(s) = &self.start_char {
            result.push_str(s);
        }

        for i in 0..self.length {
            if let Some(head) = self.head_char {
                if i == filled_count {
                    result.push(head);
                    continue;
                }
                // If we are at the very end and filled_count == length, we might want to show head?
                // Current logic: head replaces the character AT the split point.
                // If split == length (100%), i never equals split in 0..length loop?
                // Wait, if 100%, filled_count=10. i goes 0..9. i never equals 10.
                // So head is not shown at 100%? That seems wrong if head is a 'thumb'.
                // But for a progress bar, usually full = all filled.
            }

            if i < filled_count {
                result.push(self.filled_char);
            } else {
                result.push(self.empty_char);
            }
        }

        // Handle 100% case if head should be visible at the very end?
        // Or if we strictly follow "head is the current step".
        // If 100%, there is no "next step", so full bar is appropriate.

        if let Some(e) = &self.end_char {
            result.push_str(e);
        }

        result
    }
}

/// Discord Timestamp formatting styles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimestampStyle {
    /// Short Time (e.g. 16:20)
    ShortTime, // t
    /// Long Time (e.g. 16:20:30)
    LongTime, // T
    /// Short Date (e.g. 20/04/2021)
    ShortDate, // d
    /// Long Date (e.g. 20 April 2021)
    LongDate, // D
    /// Short Date/Time (e.g. 20 April 2021 16:20)
    ShortDateTime, // f (default)
    /// Long Date/Time (e.g. Tuesday, 20 April 2021 16:20)
    LongDateTime, // F
    /// Relative Time (e.g. 2 months ago)
    Relative, // R
}

impl TimestampStyle {
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::ShortTime => 't',
            Self::LongTime => 'T',
            Self::ShortDate => 'd',
            Self::LongDate => 'D',
            Self::ShortDateTime => 'f',
            Self::LongDateTime => 'F',
            Self::Relative => 'R',
        }
    }
}

/// Helper for generating Discord timestamp strings.
pub struct Timestamp;

impl Timestamp {
    /// Create a Discord timestamp tag from unix seconds.
    #[must_use]
    pub fn from_unix(seconds: i64, style: TimestampStyle) -> String {
        format!("<t:{}:{}>", seconds, style.as_char())
    }

    /// Create a Discord timestamp tag from time remaining (now + duration).
    /// Useful for "Ends in..."
    ///
    /// # Panics
    ///
    /// Panics if the system time is before the unix epoch.
    #[must_use]
    pub fn expires_in(duration_secs: u64) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let target = since_the_epoch + duration_secs;
        format!("<t:{target}:R>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        let bar = ProgressBar::new(10);
        // 50% -> 5 filled
        // 0 1 2 3 4 [5] 6 7 8 9
        // ▬ ▬ ▬ ▬ ▬ 🔘 ▬ ▬ ▬ ▬
        let output = bar.create(0.5);
        assert!(output.contains('🔘'));
        assert_eq!(output.chars().count(), 10);
    }

    #[test]
    fn test_pacman_bar() {
        let bar = ProgressBar::pacman(10);
        // 50%
        // [-----C o o o o]
        let output = bar.create(0.5);
        assert!(output.contains("C"));
        assert!(output.contains("-"));
        assert!(output.contains("o"));
        assert!(output.starts_with('['));
        assert!(output.ends_with(']'));
    }

    #[test]
    fn test_timestamp() {
        let ts = Timestamp::from_unix(1234567890, TimestampStyle::Relative);
        assert_eq!(ts, "<t:1234567890:R>");
    }
}
