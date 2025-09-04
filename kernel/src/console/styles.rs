/// Text Colors
#[allow(unused)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Default,
}

/// Background Colors
#[allow(unused)]
pub enum BackgroundColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Default,
}

/// Terminal Font Styles
#[allow(unused)]
pub enum FontStyle {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Inverse,
    Hidden,
    Strikethrough,
    NormalIntensity,
    NotUnderlined,
    PositiveImage,
}

impl Color {
    #[allow(unused)]
    pub fn to_ansi_code(self) -> &'static str {
        match self {
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::BrightBlack => "\x1b[90m",
            Color::BrightRed => "\x1b[91m",
            Color::BrightGreen => "\x1b[92m",
            Color::BrightYellow => "\x1b[93m",
            Color::BrightBlue => "\x1b[94m",
            Color::BrightMagenta => "\x1b[95m",
            Color::BrightCyan => "\x1b[96m",
            Color::BrightWhite => "\x1b[97m",
            Color::Default => "\x1b[39m",
        }
    }
}

impl BackgroundColor {
    #[allow(unused)]
    pub fn to_ansi_code(self) -> &'static str {
        match self {
            BackgroundColor::Black => "\x1b[40m",
            BackgroundColor::Red => "\x1b[41m",
            BackgroundColor::Green => "\x1b[42m",
            BackgroundColor::Yellow => "\x1b[43m",
            BackgroundColor::Blue => "\x1b[44m",
            BackgroundColor::Magenta => "\x1b[45m",
            BackgroundColor::Cyan => "\x1b[46m",
            BackgroundColor::White => "\x1b[47m",
            BackgroundColor::BrightBlack => "\x1b[100m",
            BackgroundColor::BrightRed => "\x1b[101m",
            BackgroundColor::BrightGreen => "\x1b[102m",
            BackgroundColor::BrightYellow => "\x1b[103m",
            BackgroundColor::BrightBlue => "\x1b[104m",
            BackgroundColor::BrightMagenta => "\x1b[105m",
            BackgroundColor::BrightCyan => "\x1b[106m",
            BackgroundColor::BrightWhite => "\x1b[107m",
            BackgroundColor::Default => "\x1b[49m",
        }
    }
}

impl FontStyle {
    #[allow(unused)]
    pub fn to_ansi_code(self) -> &'static str {
        match self {
            FontStyle::Reset => "\x1b[0m",
            FontStyle::Bold => "\x1b[1m",
            FontStyle::Dim => "\x1b[2m",
            FontStyle::Italic => "\x1b[3m",
            FontStyle::Underline => "\x1b[4m",
            FontStyle::Blink => "\x1b[5m",
            FontStyle::Inverse => "\x1b[7m",
            FontStyle::Hidden => "\x1b[8m",
            FontStyle::Strikethrough => "\x1b[9m",
            FontStyle::NormalIntensity => "\x1b[22m",
            FontStyle::NotUnderlined => "\x1b[24m",
            FontStyle::PositiveImage => "\x1b[27m",
        }
    }
}
