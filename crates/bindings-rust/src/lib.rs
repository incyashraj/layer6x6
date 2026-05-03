//! Rust guest SDK for Layer36 Phase 2 components.
//!
//! This crate is intentionally thin while UAPI v0.1 is still moving. It wraps
//! the generated WIT bindings with stable module names that sample apps can use
//! today, then gives us one place to improve ergonomics later.

#[allow(warnings)]
#[doc(hidden)]
pub mod bindings;

pub use bindings::Guest;

/// Common imports for small Layer36 Rust components.
///
/// This keeps sample apps readable while the SDK is still thin:
///
/// ```no_run
/// use layer36::prelude::*;
/// ```
pub mod prelude {
    pub use crate::export;
    pub use crate::fs::{self, FileExt, OpenMode};
    pub use crate::io::{self, streams::OutputStreamExt, Guest};
    pub use crate::locale;
    pub use crate::net;
    pub use crate::time;
}

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
    pub use crate::bindings::layer36::io::types::IoError;
    pub use crate::Guest;

    pub mod args {
        /// Return raw app arguments passed after `--`.
        ///
        /// The current Phase 2 draft carries arguments as newline-separated
        /// text. `split_raw` and `first_raw` are the safer helpers for normal
        /// apps.
        #[inline]
        pub fn raw() -> String {
            crate::bindings::layer36::io::args::raw()
        }

        #[inline]
        pub fn split_raw(raw: &str) -> impl Iterator<Item = &str> {
            raw.split('\n').filter(|arg| !arg.is_empty())
        }

        #[inline]
        pub fn first_raw(raw: &str) -> Option<&str> {
            split_raw(raw).next()
        }
    }

    pub mod streams {
        pub use crate::bindings::layer36::io::streams::{InputStream, OutputStream};
        pub use crate::bindings::layer36::io::types::IoError;

        pub trait InputStreamExt {
            fn read_to_end(&self) -> Result<Vec<u8>, IoError>;
            fn read_text(&self) -> Result<String, IoError>;
        }

        impl InputStreamExt for InputStream {
            fn read_to_end(&self) -> Result<Vec<u8>, IoError> {
                let mut out = Vec::new();

                loop {
                    let chunk = self.read(8192)?;
                    if chunk.is_empty() {
                        break;
                    }
                    out.extend_from_slice(&chunk);
                }

                Ok(out)
            }

            fn read_text(&self) -> Result<String, IoError> {
                String::from_utf8(self.read_to_end()?).map_err(|_| IoError::InvalidUtf8)
            }
        }

        pub trait OutputStreamExt {
            fn write_bytes(&self, bytes: &[u8]) -> Result<(), IoError>;
            fn write_text(&self, value: &str) -> Result<(), IoError>;
            fn write_line(&self, value: &str) -> Result<(), IoError>;
        }

        impl OutputStreamExt for OutputStream {
            fn write_bytes(&self, bytes: &[u8]) -> Result<(), IoError> {
                self.write_all(bytes)
            }

            fn write_text(&self, value: &str) -> Result<(), IoError> {
                self.write_all(value.as_bytes())
            }

            fn write_line(&self, value: &str) -> Result<(), IoError> {
                self.write_all(value.as_bytes())?;
                self.write_all(b"\n")
            }
        }
    }

    pub mod stdio {
        pub use crate::bindings::layer36::io::stdio::{stderr, stdin, stdout};

        use super::streams::OutputStreamExt;
        use super::IoError;

        pub fn print(value: &str) -> Result<(), IoError> {
            stdout().write_text(value)
        }

        pub fn println(value: &str) -> Result<(), IoError> {
            stdout().write_line(value)
        }

        pub fn eprint(value: &str) -> Result<(), IoError> {
            stderr().write_text(value)
        }

        pub fn eprintln(value: &str) -> Result<(), IoError> {
            stderr().write_line(value)
        }
    }

    pub mod log {
        pub use crate::bindings::layer36::io::log::{emit, Field};
        pub use crate::bindings::layer36::io::types::LogLevel;
    }
}

pub mod fs {
    pub use crate::bindings::layer36::fs::files::File;
    pub use crate::bindings::layer36::fs::types::{FileStat, FsError, OpenMode};

    pub fn open(path: &str, mode: OpenMode) -> Result<File, FsError> {
        crate::bindings::layer36::fs::files::open(path, mode)
    }

    pub fn read(path: &str) -> Result<Vec<u8>, FsError> {
        open(path, OpenMode::Read)?.read_to_end()
    }

    pub fn read_to_string(path: &str) -> Result<String, FsError> {
        open(path, OpenMode::Read)?.read_text()
    }

    pub fn write(path: &str, bytes: &[u8]) -> Result<(), FsError> {
        open(path, OpenMode::Write)?.write_all(bytes)
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

    pub trait FileExt {
        fn read_to_end(&self) -> Result<Vec<u8>, FsError>;
        fn read_text(&self) -> Result<String, FsError>;
        fn write_all(&self, bytes: &[u8]) -> Result<(), FsError>;
        fn write_text(&self, value: &str) -> Result<(), FsError>;
    }

    impl FileExt for File {
        fn read_to_end(&self) -> Result<Vec<u8>, FsError> {
            let mut out = Vec::new();

            loop {
                let chunk = self.read(8192)?;
                if chunk.is_empty() {
                    break;
                }
                out.extend_from_slice(&chunk);
            }

            Ok(out)
        }

        fn read_text(&self) -> Result<String, FsError> {
            String::from_utf8(self.read_to_end()?)
                .map_err(|_| FsError::Io("file is not valid UTF-8".to_string()))
        }

        fn write_all(&self, bytes: &[u8]) -> Result<(), FsError> {
            let mut written = 0;
            while written < bytes.len() {
                let count = self.write(&bytes[written..])? as usize;
                if count == 0 {
                    return Err(FsError::Io("file write made no progress".to_string()));
                }
                written += count;
            }

            Ok(())
        }

        fn write_text(&self, value: &str) -> Result<(), FsError> {
            self.write_all(value.as_bytes())
        }
    }
}

pub mod net {
    pub use crate::bindings::layer36::net::types::{
        Header, HttpMethod, NetError, Request, Response,
    };

    pub fn get(url: &str) -> Result<Vec<u8>, NetError> {
        crate::bindings::layer36::net::http_client::get(url)
    }

    pub fn get_text(url: &str) -> Result<String, NetError> {
        String::from_utf8(get(url)?)
            .map_err(|_| NetError::Other("response body is not valid UTF-8".to_string()))
    }

    pub fn fetch(req: Request) -> Result<Response, NetError> {
        crate::bindings::layer36::net::http_client::fetch(&req)
    }
}

pub mod time {
    pub fn now_millis() -> u64 {
        clock::now_millis()
    }

    pub fn monotonic_nanos() -> u64 {
        clock::monotonic_nanos()
    }

    pub fn sleep_millis(millis: u32) {
        sleep::sleep_millis(millis)
    }

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

    pub fn current() -> LocaleId {
        info::current()
    }

    pub fn timezone() -> String {
        info::timezone()
    }

    pub fn format_date(millis: u64, tz: &str, style: DateStyle, loc: &LocaleId) -> String {
        format::format_date(millis, tz, style, loc)
    }

    pub fn format_number(value: f64, style: NumberStyle, loc: &LocaleId) -> String {
        format::format_number(value, style, loc)
    }

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

    #[test]
    fn sdk_splits_raw_phase2_args() {
        assert_eq!(
            io::args::split_raw("one\n\nthree\n").collect::<Vec<_>>(),
            vec!["one", "three"]
        );
    }
}
