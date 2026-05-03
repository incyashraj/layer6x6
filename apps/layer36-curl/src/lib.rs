use layer36::{
    io::{args, stdio, streams::OutputStream},
    net::{self, NetError},
    Guest,
};

struct Component;

impl Guest for Component {
    fn run() -> i32 {
        let url = args::raw();
        let stderr = stdio::stderr();
        if url.is_empty() {
            let _ = write_line(&stderr, "usage: layer36-curl <url>");
            let _ = stderr.flush();
            return 2;
        }

        let body = match net::get(&url) {
            Ok(body) => body,
            Err(NetError::PermissionDenied) => {
                let _ = write_line(&stderr, "layer36-curl: permission denied");
                let _ = stderr.flush();
                return 25;
            }
            Err(NetError::InvalidUrl) => {
                let _ = write_line(&stderr, "layer36-curl: invalid url");
                let _ = stderr.flush();
                return 20;
            }
            Err(_) => {
                let _ = write_line(&stderr, "layer36-curl: fetch failed");
                let _ = stderr.flush();
                return 21;
            }
        };

        let stdout = stdio::stdout();
        if stdout.write_all(&body).is_err() || stdout.flush().is_err() {
            return 23;
        }

        0
    }
}

fn write_line(stream: &OutputStream, value: &str) -> bool {
    stream.write_all(value.as_bytes()).is_ok() && stream.write_all(b"\n").is_ok()
}

layer36::export!(Component);
