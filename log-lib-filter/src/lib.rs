pub use log;

#[macro_export]
macro_rules! if_not_any_feature {
    ($code:expr, $($feat:literal),+$(,)?) => {
        #[cfg(not(any($(feature = $feat),+)))]
        $code
    }
}

#[macro_export]
macro_rules! if_log_trace {
    ($code:expr) => {
        $crate::if_not_any_feature!($code,
            "max_level_off",
            "max_level_error",
            "max_level_warn",
            "max_level_info",
            "max_level_debug",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_debug {
    ($code:expr) => {
        $crate::if_not_any_feature!($code,
            "max_level_off",
            "max_level_error",
            "max_level_warn",
            "max_level_info",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_info {
    ($code:expr) => {
        $crate::if_not_any_feature!($code,
            "max_level_off",
            "max_level_error",
            "max_level_warn",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_warn {
    ($code:expr) => {
        $crate::if_not_any_feature!($code,
            "max_level_off",
            "max_level_error",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_error {
    ($code:expr) => {
        $crate::if_not_any_feature!($code,
            "max_level_off",
        )
    }
}
 
#[macro_export]
macro_rules! if_log_off {
    ($code:expr) => {
        #[cfg(feature = "max_level_off")]
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
