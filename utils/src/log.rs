// log

#[cfg(feature = "dev")]
#[macro_export(local_inner_macros)]
macro_rules! info {
    ($($arg:tt)*) => {
        ::std::println!("{} \x1b[32mINFO\x1b[0m {}:{}  {:?}", ::utils::time::current_time(60 * 60 * 8), std::file!(), std::line!(), ::std::format_args!($($arg)*))
    }
}

#[cfg(feature = "dev")]
#[macro_export(local_inner_macros)]
macro_rules! debug {
    ($($arg:tt)*) => {
        ::std::println!("{} \x1b[34mDEBUG\x1b[0m {}:{}  {:?}", ::utils::time::current_time(60 * 60 * 8), std::file!(), std::line!(), ::std::format_args!($($arg)*))
    }
}

#[cfg(feature = "dev")]
#[macro_export(local_inner_macros)]
macro_rules! error {
    ($($arg:tt)*) => {
        ::std::println!("{} \x1b[31mERROR\x1b[0m {}:{}  {:?}", ::utils::time::current_time(60 * 60 * 8), std::file!(), std::line!(), ::std::format_args!($($arg)*))
    }
}

#[cfg(feature = "dev")]
#[macro_export(local_inner_macros)]
macro_rules! warn {
    ($($arg:tt)*) => {
        ::std::println!("{} \x1b[93mWARN\x1b[0m {}:{}  {:?}", ::utils::time::current_time(60 * 60 * 8), std::file!(), std::line!(), ::std::format_args!($($arg)*));
    }
}

#[cfg(feature = "clean")]
#[macro_export(local_inner_macros)]
macro_rules! info {
    ($($arg:tt)*) => { ::std::format_args!($($arg)*) };
}

#[cfg(feature = "clean")]
#[macro_export(local_inner_macros)]
macro_rules! debug {
    ($($arg:tt)*) => { ::std::format_args!($($arg)*) };
}

#[cfg(feature = "clean")]
#[macro_export(local_inner_macros)]
macro_rules! error {
    ($($arg:tt)*) => { ::std::format_args!($($arg)*) };
}

#[cfg(feature = "clean")]
#[macro_export(local_inner_macros)]
macro_rules! warn {
    ($($arg:tt)*) => { ::std::format_args!($($arg)*) };
}
