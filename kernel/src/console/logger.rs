// helper macro
#[macro_export]
macro_rules! __log_print_helper {
    ($level:literal, $color:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        $crate::console::print(
            format_args!(
                "[kernel] [{}{}{}] {}\n",
                $color.to_ansi_code(),
                $level,
                $crate::console::styles::Color::Default.to_ansi_code(),
                format_args!($fmt $(, $($arg)+)?)
            )
        );
    };

    ($level:literal, $color:expr, $fmt:literal, no_newline $(, $($arg:tt)+)?) => {
        $crate::console::print(
            format_args!(
                "[kernel] [{}{}{}] {}",
                $color.to_ansi_code(),
                $level,
                $crate::console::styles::Color::Default.to_ansi_code(),
                format_args!($fmt $(, $($arg)+)?)
            )
        );
    };
}

// DEBUG Level
#[macro_export]
macro_rules! debug {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::__log_print_helper!("DEBUG", $crate::console::styles::Color::BrightBlack, $fmt, no_newline $(, $($arg)+)?);
    };
}

#[macro_export]
macro_rules! debugln {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::__log_print_helper!("DEBUG", $crate::console::styles::Color::BrightBlack, $fmt $(, $($arg)+)?);
    };
}

// INFO Level
#[macro_export]
macro_rules! info {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::__log_print_helper!("INFO ", $crate::console::styles::Color::Green, $fmt, no_newline $(, $($arg)+)?);
    };
}

#[macro_export]
macro_rules! infoln {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::__log_print_helper!("INFO ", $crate::console::styles::Color::Green, $fmt $(, $($arg)+)?);
    };
}

// WARN Level
#[macro_export]
macro_rules! warn {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::__log_print_helper!("WARN ", $crate::console::styles::Color::Yellow, $fmt, no_newline $(, $($arg)+)?);
    };
}

#[macro_export]
macro_rules! warnln {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::__log_print_helper!("WARN ", $crate::console::styles::Color::Yellow, $fmt $(, $($arg)+)?);
    };
}

// ERROR Level
#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::__log_print_helper!("ERROR", $crate::console::styles::Color::Red, $fmt, no_newline $(, $($arg)+)?);
    };
}

#[macro_export]
macro_rules! errorln {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::__log_print_helper!("ERROR", $crate::console::styles::Color::Red, $fmt $(, $($arg)+)?);
    };
}
