#[allow(dead_code)]
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&format!($($arg)*).into())
    };
}

#[allow(dead_code)]
#[macro_export]
macro_rules! console_warn {
    ($($arg:tt)*) => {
        web_sys::console::warn_1(&format!($($arg)*).into());
    };
}

#[allow(dead_code)]
#[macro_export]
macro_rules! console_error {
    ($($arg:tt)*) => {
        web_sys::console::error_1(&format!($($arg)*).into());
    };
}

#[allow(dead_code)]
#[macro_export]
macro_rules! console_debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            web_sys::console::log_1(&format!("[DEBUG] {}", format!($($arg)*)).into());
        }
    };
}

#[allow(dead_code)]
#[macro_export]
macro_rules! console_log_with_context {
    ($context:expr, $($arg:tt)*) => {
        web_sys::console::log_1(&format!("[{}] {}", $context, format!($($arg)*)).into());
    };
}

#[allow(dead_code)]
#[macro_export]
macro_rules! console_warn_with_context {
    ($context:expr, $($arg:tt)*) => {
        web_sys::console::warn_1(&format!("[{}] WARNING: {}", $context, format!($($arg)*)).into());
    };
}

#[macro_export]
macro_rules! console_error_with_context {
    ($context:expr, $($arg:tt)*) => {
        web_sys::console::error_1(&format!("[{}] ERROR: {}", $context, format!($($arg)*)).into())
    };
}

#[allow(dead_code)]
#[macro_export]
macro_rules! console_log_function {
    ($function_name:expr) => {
        if cfg!(debug_assertions) {
            web_sys::console::log_1(&format!("[FUNCTION] Entering: {}", $function_name).into());
        }
    };
}

#[allow(dead_code)]
#[macro_export]
macro_rules! console_log_function_exit {
    ($function_name:expr) => {
        if cfg!(debug_assertions) {
            web_sys::console::log_1(&format!("[FUNCTION] Exiting: {}", $function_name).into());
        }
    };
}

#[allow(dead_code)]
#[macro_export]
macro_rules! console_log_state_change {
    ($from:expr, $to:expr) => {
        web_sys::console::log_1(&format!("[STATE] {} -> {}", $from, $to).into());
    };
}

#[macro_export]
macro_rules! console_log_user_action {
    ($action:expr) => {
        web_sys::console::log_1(&format!("[USER ACTION] {}", $action).into());
    };
    ($($arg:tt)*) => {
        web_sys::console::log_1(&format!("[USER ACTION] {}", format!($($arg)*)).into());
    };
}

#[allow(unused_imports)]
pub mod console {
    pub use crate::{
        console_debug as debug, console_error as error,
        console_error_with_context as error_with_context, console_log as log,
        console_log_function as log_function, console_log_function_exit as log_function_exit,
        console_log_state_change as log_state_change, console_log_user_action as log_user_action,
        console_log_with_context as log_with_context, console_warn as warn,
        console_warn_with_context as warn_with_context,
    };
}
