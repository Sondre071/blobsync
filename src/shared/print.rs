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
                .replace("%n", $crate::shared::print::NAME_COLOR)
                .replace("%d", $crate::shared::print::DATA_COLOR)
                .replace("%t", $crate::shared::print::TEXT_COLOR)
                .replace("%m", $crate::shared::print::MESSAGE_COLOR)
                .replace("%w", $crate::shared::print::WARNING_COLOR);

                std::println!("{text}{}", $crate::shared::print::RESET_STYLE);
            }
        }}
}
