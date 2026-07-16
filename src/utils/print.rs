pub const TEXT_COLOR: &str = "\x1b[0;37m";
pub const NAME_COLOR: &str = "\x1b[1;33m";
pub const DATA_COLOR: &str = "\x1b[1;37m";
pub const MESSAGE_COLOR: &str = "\x1b[1;36m";
pub const WARNING_COLOR: &str = "\x1b[1;33m";

pub const RESET_STYLE: &str = "\x1b[0m";

#[macro_export]
macro_rules! println {
        ($($arg:tt)*) => {{
            #[cfg(debug_assertions)]
            {
                let text = format!($($arg)*)
                .replace("%n", $crate::utils::print::NAME_COLOR)
                .replace("%d", $crate::utils::print::DATA_COLOR)
                .replace("%t", $crate::utils::print::TEXT_COLOR)
                .replace("%m", $crate::utils::print::MESSAGE_COLOR)
                .replace("%w", $crate::utils::print::WARNING_COLOR);

                std::println!("{text}{}", $crate::utils::print::RESET_STYLE);
            }
        }}
}
