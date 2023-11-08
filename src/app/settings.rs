pub(crate) enum Settings {
    WindowWidth,
    WindowHeight,
    WindowMaximized,
}

impl Settings {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::WindowWidth => "window-width",
            Self::WindowHeight => "window-height",
            Self::WindowMaximized => "window-maximized",
        }
    }
}
