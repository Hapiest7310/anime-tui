#[cfg(feature = "tui")]
use ratatui::style::{Color, Modifier, Style};

#[cfg(feature = "tui")]
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary: Color,   // Headers, titles
    pub highlight: Color, // Selected items
    pub success: Color,   // Success messages
    pub subtle: Color,    // Help text, hints
    #[allow(dead_code)]
    pub text: Color, // Default text (reserved for future use)
}

#[cfg(feature = "tui")]
impl Theme {
    /// Detect theme based on terminal's light/dark preference
    pub fn detect() -> Self {
        // Try to detect if terminal is dark or light
        let is_dark = terminal_colorsaurus::theme_mode(Default::default())
            .map(|mode| matches!(mode, terminal_colorsaurus::ThemeMode::Dark))
            .unwrap_or(true); // Default to dark theme

        if is_dark {
            Self::dark()
        } else {
            Self::light()
        }
    }

    /// Dark theme - suitable for dark terminal backgrounds
    pub fn dark() -> Self {
        Self {
            primary: Color::Cyan,
            highlight: Color::Yellow,
            success: Color::Green,
            subtle: Color::DarkGray,
            text: Color::White,
        }
    }

    /// Light theme - suitable for light terminal backgrounds
    pub fn light() -> Self {
        Self {
            primary: Color::Blue,
            highlight: Color::Magenta,
            success: Color::Green,
            subtle: Color::Gray,
            text: Color::Black,
        }
    }

    // Style builders for common UI elements
    pub fn header(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected(&self) -> Style {
        Style::default()
            .fg(self.highlight)
            .add_modifier(Modifier::REVERSED)
    }

    pub fn success_message(&self) -> Style {
        Style::default().fg(self.success)
    }

    pub fn hint_text(&self) -> Style {
        Style::default().fg(self.subtle)
    }

    #[allow(dead_code)]
    pub fn normal_text(&self) -> Style {
        Style::default().fg(self.text)
    }

    pub fn emphasized(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }
}

#[cfg(feature = "tui")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme_creation() {
        let theme = Theme::dark();
        assert_eq!(theme.primary, Color::Cyan);
        assert_eq!(theme.highlight, Color::Yellow);
    }

    #[test]
    fn test_light_theme_creation() {
        let theme = Theme::light();
        assert_eq!(theme.primary, Color::Blue);
        assert_eq!(theme.highlight, Color::Magenta);
    }
}
