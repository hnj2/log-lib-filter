#![no_std]
pub use log;

#[macro_export]
macro_rules! if_features {
    ($code:expr, not_any $($not_any:literal,)* any $($any:literal),*$(,)?) => {
        #[cfg(all(
            not(any(
                $(feature = $not_any),*
            )),
            any(
                $(feature = $any),*
            )
        ))]
        $code
    }
}

#[macro_export]
macro_rules! if_log_trace {
    ($code:expr) => {
        $crate::if_features!(
            $code,
                not_any
                    "max_level_off",
                    "max_level_error",
                    "max_level_warn",
                    "max_level_info",
                    "max_level_debug",
                any
                    "log_level_trace",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_debug {
    ($code:expr) => {
        $crate::if_features!(
            $code,
                not_any
                    "max_level_off",
                    "max_level_error",
                    "max_level_warn",
                    "max_level_info",
                any
                    "log_level_debug",
                    "log_level_trace",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_info {
    ($code:expr) => {
        $crate::if_features!(
            $code,
                not_any
                    "max_level_off",
                    "max_level_error",
                    "max_level_warn",
                any
                    "log_level_info",
                    "log_level_debug",
                    "log_level_trace",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_warn {
    ($code:expr) => {
        $crate::if_features!(
            $code,
                not_any
                    "max_level_off",
                    "max_level_error",
                any
                    "log_level_warn",
                    "log_level_info",
                    "log_level_debug",
                    "log_level_trace",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_error {
    ($code:expr) => {
        $crate::if_features!(
            $code,
                not_any
                    "max_level_off",
                any
                    "log_level_error",
                    "log_level_warn",
                    "log_level_info",
                    "log_level_debug",
                    "log_level_trace",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_off {
    ($code:expr) => {
        #[cfg(any(
            feature = "max_level_off",
            not(any(
                "log_level_error",
                "log_level_warn",
                "log_level_info",
                "log_level_debug",
                "log_level_trace",
            ))
        ))]
        $code
    }
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => { $crate::if_log_trace!($crate::log::trace!($target, $($arg)+)) };
    ($($arg:tt)+) => {                       $crate::if_log_trace!($crate::log::trace!($($arg)+)) };
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => { $crate::if_log_debug!($crate::log::debug!($target, $($arg)+)) };
    ($($arg:tt)+) => {                       $crate::if_log_debug!($crate::log::debug!($($arg)+)) };
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => { $crate::if_log_info!($crate::log::info!($target, $($arg)+)) };
    ($($arg:tt)+) => {                       $crate::if_log_info!($crate::log::info!($($arg)+)) };
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => { $crate::if_log_warn!($crate::log::warn!($target, $($arg)+)) };
    ($($arg:tt)+) => {                       $crate::if_log_warn!($crate::log::warn!($($arg)+)) };
}

#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => { $crate::if_log_error!($crate::log::error!($target, $($arg)+)) };
    ($($arg:tt)+) => {                       $crate::if_log_error!($crate::log::error!($($arg)+)) };
}

pub fn run() {
    trace!("liba trace");
    debug!("liba debug");
    info!("liba info");
    warn!("liba warning");
    error!("liba error");
}
