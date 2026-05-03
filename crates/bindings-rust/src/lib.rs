//! Rust guest SDK for Layer36 Phase 2 components.
//!
//! This crate is intentionally thin while UAPI v0.1 is still moving. It wraps
//! the generated WIT bindings with stable module names that sample apps can use
//! today, then gives us one place to improve ergonomics later.

#[allow(warnings)]
#[doc(hidden)]
pub mod bindings;

pub use bindings::Guest;

#[macro_export]
macro_rules! export {
    ($ty:ident) => {
        const _: () = {
            #[unsafe(export_name = "run")]
            unsafe extern "C" fn export_run() -> i32 {
                unsafe { $crate::bindings::_export_run_cabi::<$ty>() }
            }
        };
    };
}

pub mod io {
    pub mod args {
        pub fn raw() -> String {
            crate::bindings::layer36::io::args::raw()
        }
    }

    pub mod streams {
        pub use crate::bindings::layer36::io::streams::{InputStream, IoError, OutputStream};
    }

    pub mod stdio {
        pub use crate::bindings::layer36::io::stdio::{stderr, stdin, stdout};
    }

    pub mod log {
        pub use crate::bindings::layer36::io::log::{emit, Field};
        pub use crate::bindings::layer36::io::types::LogLevel;
    }

    pub use crate::bindings::layer36::io::types::IoError;
}

pub mod fs {
    pub use crate::bindings::layer36::fs::files::File;
    pub use crate::bindings::layer36::fs::types::{FileStat, FsError, OpenMode};

    pub fn open(path: &str, mode: OpenMode) -> Result<File, FsError> {
        crate::bindings::layer36::fs::files::open(path, mode)
    }

    pub fn stat(path: &str) -> Result<FileStat, FsError> {
        crate::bindings::layer36::fs::files::stat(path)
    }

    pub fn list(path: &str) -> Result<Vec<String>, FsError> {
        crate::bindings::layer36::fs::files::list(path)
    }

    pub fn remove_file(path: &str) -> Result<(), FsError> {
        crate::bindings::layer36::fs::files::remove_file(path)
    }

    pub fn remove_dir(path: &str) -> Result<(), FsError> {
        crate::bindings::layer36::fs::files::remove_dir(path)
    }

    pub fn mkdir(path: &str) -> Result<(), FsError> {
        crate::bindings::layer36::fs::files::mkdir(path)
    }

    pub fn rename(from: &str, to: &str) -> Result<(), FsError> {
        crate::bindings::layer36::fs::files::rename(from, to)
    }
}

pub mod net {
    pub use crate::bindings::layer36::net::types::{
        Header, HttpMethod, NetError, Request, Response,
    };

    pub fn get(url: &str) -> Result<Vec<u8>, NetError> {
        crate::bindings::layer36::net::http_client::get(url)
    }

    pub fn fetch(req: Request) -> Result<Response, NetError> {
        crate::bindings::layer36::net::http_client::fetch(&req)
    }
}

pub mod time {
    pub mod clock {
        pub fn now_millis() -> u64 {
            crate::bindings::layer36::time::clock::now_millis()
        }

        pub fn monotonic_nanos() -> u64 {
            crate::bindings::layer36::time::clock::monotonic_nanos()
        }
    }

    pub mod sleep {
        pub fn sleep_millis(millis: u32) {
            crate::bindings::layer36::time::sleep::sleep_millis(millis)
        }
    }
}

pub mod locale {
    pub use crate::bindings::layer36::locale::types::{DateStyle, LocaleId, NumberStyle};

    pub mod info {
        pub use crate::bindings::layer36::locale::info::{current, timezone};
    }

    pub mod format {
        pub use crate::bindings::layer36::locale::format::{format_date, format_number};
        pub use crate::bindings::layer36::locale::types::{DateStyle, LocaleId, NumberStyle};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdk_reexports_core_phase2_types() {
        let mode = fs::OpenMode::Read;
        let method = net::HttpMethod::Get;
        let date_style = locale::DateStyle::Short;

        assert!(matches!(mode, fs::OpenMode::Read));
        assert!(matches!(method, net::HttpMethod::Get));
        assert!(matches!(date_style, locale::DateStyle::Short));
    }
}
