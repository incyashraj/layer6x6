#[allow(warnings)]
mod bindings;

use bindings::layer36::{
    fs::{
        files,
        types::{FsError, OpenMode},
    },
    io::{args, stdio},
};
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn run() -> i32 {
        let app_args = args::raw();
        let stderr = stdio::stderr();

        if app_args.is_empty() {
            let _ = write_line(&stderr, "usage: layer36-cat <file> [file...]");
            let _ = stderr.flush();
            return 2;
        }

        let stdout = stdio::stdout();
        for path in app_args.split('\n') {
            if path.is_empty() {
                continue;
            }

            let file = match files::open(path, OpenMode::Read) {
                Ok(file) => file,
                Err(FsError::PermissionDenied) => {
                    let _ = write_error(&stderr, "permission denied", path);
                    let _ = stderr.flush();
                    return 25;
                }
                Err(FsError::NotFound) => {
                    let _ = write_error(&stderr, "not found", path);
                    let _ = stderr.flush();
                    return 20;
                }
                Err(_) => {
                    let _ = write_error(&stderr, "could not open", path);
                    let _ = stderr.flush();
                    return 21;
                }
            };

            loop {
                let bytes = match file.read(8192) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        let _ = write_error(&stderr, "could not read", path);
                        let _ = stderr.flush();
                        return 22;
                    }
                };

                if bytes.is_empty() {
                    break;
                }

                if stdout.write_all(&bytes).is_err() {
                    return 23;
                }
            }
        }

        if stdout.flush().is_err() {
            return 24;
        }

        0
    }
}

fn write_line(stream: &bindings::layer36::io::streams::OutputStream, value: &str) -> bool {
    stream.write_all(value.as_bytes()).is_ok() && stream.write_all(b"\n").is_ok()
}

fn write_error(
    stream: &bindings::layer36::io::streams::OutputStream,
    message: &str,
    path: &str,
) -> bool {
    stream.write_all(b"layer36-cat: ").is_ok()
        && stream.write_all(message.as_bytes()).is_ok()
        && stream.write_all(b": ").is_ok()
        && write_line(stream, path)
}

bindings::export!(Component with_types_in bindings);
