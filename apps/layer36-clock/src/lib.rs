use layer36::{
    io::{stdio, streams::OutputStream},
    locale::{format, info, DateStyle},
    time::clock,
    Guest,
};

struct Component;

impl Guest for Component {
    fn run() -> i32 {
        let millis = clock::now_millis();
        let locale = info::current();
        let timezone = info::timezone();
        let date = format::format_date(millis, &timezone, DateStyle::Medium, &locale);

        let stdout = stdio::stdout();
        if !write_pair(&stdout, "app", "layer36-clock")
            || !write_pair(&stdout, "timezone", &timezone)
            || !write_pair(&stdout, "locale", &locale.bcp47)
            || !write_pair(&stdout, "date", &date)
            || stdout.flush().is_err()
        {
            return 20;
        }

        0
    }
}

fn write_line(stream: &OutputStream, value: &str) -> bool {
    stream.write_all(value.as_bytes()).is_ok() && stream.write_all(b"\n").is_ok()
}

fn write_pair(stream: &OutputStream, key: &str, value: &str) -> bool {
    stream.write_all(key.as_bytes()).is_ok()
        && stream.write_all(b"=").is_ok()
        && write_line(stream, value)
}

layer36::export!(Component);
