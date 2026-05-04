# UAPI Reference

> Generated from `wit/layer36/phase2`. Do not edit this page by hand.

Layer36 Phase 2 exposes the `cli` world from `layer36:app@0.1.0`.

The current world imports these interfaces:

- `layer36:io/types@0.1.0`
- `layer36:io/streams@0.1.0`
- `layer36:io/stdio@0.1.0`
- `layer36:io/args@0.1.0`
- `layer36:io/log@0.1.0`
- `layer36:fs/types@0.1.0`
- `layer36:fs/files@0.1.0`
- `layer36:net/types@0.1.0`
- `layer36:net/http-client@0.1.0`
- `layer36:time/clock@0.1.0`
- `layer36:time/sleep@0.1.0`
- `layer36:locale/types@0.1.0`
- `layer36:locale/info@0.1.0`
- `layer36:locale/format@0.1.0`

The app exports:

- `run() -> s32`

## `layer36:fs/files@0.1.0`

Filesystem entry points. All host file access should pass through these functions and resource methods.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `fs.read:<path-glob>` - manifest or session grant
- `fs.write:<path-glob>` - manifest or session grant
- `fs.list:<path-glob>` - manifest or session grant
- `fs.remove:<path-glob>` - manifest or session grant
- `fs.mkdir:<path-glob>` - manifest or session grant

- `open`, `stat`, and `list` require a matching `fs.read:PATH` grant for read-style access.
- Write, mkdir, remove, and rename operations are part of the Phase 2 shape, but the first runtime slice focuses on read grants.

### Rust SDK Example

```rust
let text = layer36::fs::read_to_string("notes.txt")?;
layer36::io::stdio::println(&text)?;
```

### Functions

- `open(path: string, mode: open-mode) -> result<own<file>, fs-error>`
  - Opens a host file through Layer36 and returns a `file` handle.
  - `read` needs `fs.read:PATH`; `write`, `append`, and `read-write` also need the matching write grant.
- `stat(path: string) -> result<file-stat, fs-error>`
  - Reads file metadata without opening the file body.
  - Requires `fs.read:PATH` for the path being inspected.
- `list(path: string) -> result<list<string>, fs-error>`
  - Returns directory entry names for a granted directory.
  - Requires `fs.list:PATH` before the adapter reads the directory.
- `remove-file(path: string) -> result<_, fs-error>`
  - Deletes one file.
  - Requires `fs.remove:PATH`; missing grants fail before host deletion is attempted.
- `remove-dir(path: string) -> result<_, fs-error>`
  - Deletes one directory.
  - Requires `fs.remove:PATH`; hosts can still reject non-empty directories.
- `mkdir(path: string) -> result<_, fs-error>`
  - Creates one directory.
  - Requires `fs.mkdir:PATH` for the directory being created.
- `rename(from: string, to: string) -> result<_, fs-error>`
  - Moves or renames a file or directory.
  - Requires grants for both sides: remove/write style access to the source and write style access to the destination.

### Types

#### `file` resource

#### `file` methods

- `read(n: u32) -> result<list<u8>, fs-error>`
  - Reads up to `n` bytes from an opened file handle.
  - The runtime rechecks the handle path before each adapter read.
- `write(bytes: list<u8>) -> result<u32, fs-error>`
  - Writes bytes to an opened file handle and returns the number written.
  - The runtime rechecks write permission before each adapter write.
- `seek-set(pos: u64) -> result<u64, fs-error>`
  - Moves the file cursor to an absolute byte position.
  - The handle must still be valid and backed by a granted file.
- `seek-end() -> result<u64, fs-error>`
  - Moves the file cursor to the end and returns the new position.
  - Useful before append-style writes or size checks.
- `stat() -> result<file-stat, fs-error>`
  - Reads metadata for the opened file handle.
  - The runtime rechecks the handle path before the adapter stat call.


## `layer36:fs/types@0.1.0`

Shared filesystem records, modes, and error shapes.

### Types

#### `file-stat` record

- `size`: `u64`
- `modified-millis`: `u64`
- `is-dir`: `bool`

#### `open-mode` variant

- `read`
- `write`
- `read-write`
- `append`

#### `fs-error` variant

- `not-found`
- `permission-denied`
- `already-exists`
- `invalid-path`
- `not-a-directory`
- `is-a-directory`
- `io`: `string`


## `layer36:io/args@0.1.0`

Raw Layer36 app arguments. These are the arguments passed after `--` in `layer36 run`.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `io.stdin` - default grant
- `io.stdout` - default grant
- `io.stderr` - default grant
- `io.args` - default grant
- `io.log` - default grant

- `io.args` is granted by default for CLI apps.
- The current draft encodes args as newline-separated text.

### Rust SDK Example

```rust
let raw = layer36::io::args::raw();
let first = layer36::io::args::first_raw(&raw);
```

### Functions

- `raw() -> string`
  - Returns the app arguments passed after `--` in `layer36 run`.
  - Current encoding is newline-separated text, so SDK helpers should parse it for app code.


## `layer36:io/log@0.1.0`

Structured app logs. Hosts can route these to native logs, developer consoles, or test captures.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `io.stdin` - default grant
- `io.stdout` - default grant
- `io.stderr` - default grant
- `io.args` - default grant
- `io.log` - default grant

- `io.log` is a low-risk default grant.

### Functions

- `emit(level: log-level, message: string, fields: list<field>)`
  - Sends one structured log event to the host.
  - Fields are plain key/value strings so native hosts can map them to their own log systems.

### Types

#### `field` record

- `key`: `string`
- `value`: `string`


## `layer36:io/stdio@0.1.0`

Standard input, output, and error streams for CLI-style apps.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `io.stdin` - default grant
- `io.stdout` - default grant
- `io.stderr` - default grant
- `io.args` - default grant
- `io.log` - default grant

- `io.stdin`, `io.stdout`, and `io.stderr` are low-risk default grants for CLI apps.

### Rust SDK Example

```rust
layer36::io::stdio::println("Hello from Layer36")?;
layer36::io::stdio::eprintln("debug line")?;
```

### Functions

- `stdin() -> own<input-stream>`
  - Returns an input stream connected to the host standard input.
  - Granted by default for CLI apps.
- `stdout() -> own<output-stream>`
  - Returns an output stream connected to host standard output.
  - Use this for normal command output that other tools may read.
- `stderr() -> own<output-stream>`
  - Returns an output stream connected to host standard error.
  - Use this for diagnostics and permission errors.


## `layer36:io/streams@0.1.0`

Byte streams used by stdio and other UAPI modules.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `io.stdin` - default grant
- `io.stdout` - default grant
- `io.stderr` - default grant
- `io.args` - default grant
- `io.log` - default grant

- `io.stdin`, `io.stdout`, and `io.stderr` are low-risk default grants for CLI apps.

### Rust SDK Example

```rust
use layer36::io::streams::OutputStreamExt;

let out = layer36::io::stdio::stdout();
out.write_line("ok")?;
out.flush()?;
```

### Types

#### `input-stream` resource

#### `output-stream` resource

#### `input-stream` methods

- `read(n: u32) -> result<list<u8>, io-error>`
  - Reads up to `n` bytes from an input stream.
  - A short read is valid; an empty read means the stream has no more bytes right now or is closed.
- `read-to-string() -> result<string, io-error>`
  - Reads the stream as UTF-8 text.
  - Invalid UTF-8 returns `io-error.invalid-utf8` instead of lossy text.

#### `output-stream` methods

- `write(bytes: list<u8>) -> result<u32, io-error>`
  - Writes bytes to an output stream and returns the number accepted.
  - Apps that need all bytes written should use `write-all` or an SDK helper.
- `write-all(bytes: list<u8>) -> result<_, io-error>`
  - Writes the full byte buffer or returns an IO error.
  - This is the right primitive for line-oriented CLI output.
- `flush() -> result<_, io-error>`
  - Asks the host to push buffered output through.
  - Use it before exiting after important diagnostics or prompts.


## `layer36:io/types@0.1.0`

Shared IO log and error types.

### Types

#### `log-level` enum

- `trace`
- `debug`
- `info`
- `warn`
- `error`

#### `io-error` variant

- `closed`
- `interrupted`
- `unexpected-eof`
- `invalid-utf8`
- `other`: `string`


## `layer36:locale/format@0.1.0`

Host-backed date and number formatting.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `locale.info` - default grant
- `locale.format` - default grant

- Locale reads and formatting are default grants for CLI apps.

### Rust SDK Example

```rust
let locale = layer36::locale::current();
let text = layer36::locale::format_number(42.0, layer36::locale::NumberStyle::Decimal, &locale);
```

### Functions

- `format-date(millis: u64, tz: string, style: date-style, loc: locale-id) -> string`
  - Formats a timestamp using a requested timezone, date style, and locale.
  - The host owns the native formatting behavior so output can match the platform.
- `format-number(value: f64, style: number-style, loc: locale-id) -> string`
  - Formats a number using a requested style and locale.
  - Currency style is present in the shape, but richer currency-code handling remains future work.


## `layer36:locale/info@0.1.0`

The host user's current locale and timezone.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `locale.info` - default grant
- `locale.format` - default grant

- Locale reads and formatting are default grants for CLI apps.

### Rust SDK Example

```rust
let locale = layer36::locale::current();
let timezone = layer36::locale::timezone();
```

### Functions

> The user's preferred locale as reported by the host.

- `current() -> locale-id`
  - Returns the host user's preferred locale as a BCP 47 string.
  - Good for display choices, not for security decisions.
> IANA timezone name, for example "Asia/Singapore".

- `timezone() -> string`
  - Returns the host timezone name.
  - Expected form is an IANA name such as `Asia/Singapore` when the host can provide one.


## `layer36:locale/types@0.1.0`

Locale and formatting type definitions.

### Types

#### `locale-id` record

- `bcp47`: `string`

#### `date-style` enum

- `short`
- `medium`
- `long`
- `full`

#### `number-style` enum

- `decimal`
- `percent`
- `currency`


## `layer36:net/http-client@0.1.0`

HTTP client calls. Phase 2 starts with simple request and response bodies.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `net.connect:<host>:<port>` - manifest or session grant

- `get` and `fetch` require a matching `net.connect:HOST:PORT` grant before the adapter opens a socket.
- The current host adapter supports plain HTTP request framing first, with a 1 MiB full-response cap; HTTPS, redirects, streaming, and richer network behavior are still Phase 2 work.

### Rust SDK Example

```rust
let body = layer36::net::get_text("http://127.0.0.1:8080/data.txt")?;
layer36::io::stdio::println(&body)?;
```

### Functions

- `get(url: string) -> result<list<u8>, net-error>`
  - Performs a simple HTTP GET and returns only the response body.
  - Requires `net.connect:HOST:PORT`; Phase 2 currently supports the plain HTTP adapter path.
- `fetch(req: request) -> result<response, net-error>`
  - Performs a lower-level HTTP request and returns status, headers, and body.
  - The plain HTTP adapter now forwards the method, app headers, and buffered body while keeping `Host`, `Connection`, and `Content-Length` under host control.
  - Timeouts, oversized bodies, malformed responses, and missing grants are typed as `net-error` cases.


## `layer36:net/types@0.1.0`

Shared network request, response, and error types.

### Types

#### `http-method` enum

- `get`
- `post`
- `put`
- `delete`
- `patch`
- `head`
- `options`

#### `header` record

- `name`: `string`
- `value`: `string`

#### `request` record

- `method`: `http-method`
- `url`: `string`
- `headers`: `list<header>`
- `body`: `list<u8>`
- `timeout-millis`: `option<u32>`

#### `response` record

- `status`: `u16`
- `headers`: `list<header>`
- `body`: `list<u8>`

#### `net-error` variant

- `invalid-url`
- `dns-failure`: `string`
- `connect-failure`: `string`
- `tls-failure`: `string`
- `timeout`
- `body-too-large`
- `permission-denied`
- `protocol`: `string`
- `other`: `string`


## `layer36:time/clock@0.1.0`

Wall-clock and monotonic clock reads.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `time.clock` - default grant
- `time.monotonic` - default grant
- `time.sleep` - default grant

- `time.clock` and `time.monotonic` are default grants.

### Rust SDK Example

```rust
let now = layer36::time::now_millis();
let tick = layer36::time::monotonic_nanos();
```

### Functions

> Milliseconds since Unix epoch. Wall-clock; can jump.

- `now-millis() -> u64`
  - Reads host wall-clock time in milliseconds since Unix epoch.
  - This value can move backward or forward if the host clock changes.
> Monotonic nanoseconds since an arbitrary origin.
> Guaranteed non-decreasing; suitable for measuring intervals.

- `monotonic-nanos() -> u64`
  - Reads a non-decreasing timer in nanoseconds.
  - Use this for durations instead of wall-clock time.


## `layer36:time/sleep@0.1.0`

Blocking sleep for CLI-style components.

### Capability Notes

Accepted capability strings for this module, generated from the runtime manifest table:

- `time.clock` - default grant
- `time.monotonic` - default grant
- `time.sleep` - default grant

- `sleep-millis` requires `time.sleep`.

### Rust SDK Example

```rust
layer36::time::sleep_millis(100);
```

### Functions

> Block the calling task for at least `millis` milliseconds.

- `sleep-millis(millis: u32)`
  - Blocks the calling component task for at least the requested milliseconds.
  - Requires `time.sleep`; hosts may wake slightly later than requested.

