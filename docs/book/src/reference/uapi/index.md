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

### Functions

- `open(path: string, mode: open-mode) -> result<own<file>, fs-error>`
- `stat(path: string) -> result<file-stat, fs-error>`
- `list(path: string) -> result<list<string>, fs-error>`
- `remove-file(path: string) -> result<_, fs-error>`
- `remove-dir(path: string) -> result<_, fs-error>`
- `mkdir(path: string) -> result<_, fs-error>`
- `rename(from: string, to: string) -> result<_, fs-error>`

### Types

#### `file` resource

#### `file` methods

- `read(n: u32) -> result<list<u8>, fs-error>`
- `write(bytes: list<u8>) -> result<u32, fs-error>`
- `seek-set(pos: u64) -> result<u64, fs-error>`
- `seek-end() -> result<u64, fs-error>`
- `stat() -> result<file-stat, fs-error>`


## `layer36:fs/types@0.1.0`

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

### Functions

- `raw() -> string`


## `layer36:io/log@0.1.0`

### Functions

- `emit(level: log-level, message: string, fields: list<field>)`

### Types

#### `field` record

- `key`: `string`
- `value`: `string`


## `layer36:io/stdio@0.1.0`

### Functions

- `stdin() -> own<input-stream>`
- `stdout() -> own<output-stream>`
- `stderr() -> own<output-stream>`


## `layer36:io/streams@0.1.0`

### Types

#### `input-stream` resource

#### `output-stream` resource

#### `input-stream` methods

- `read(n: u32) -> result<list<u8>, io-error>`
- `read-to-string() -> result<string, io-error>`

#### `output-stream` methods

- `write(bytes: list<u8>) -> result<u32, io-error>`
- `write-all(bytes: list<u8>) -> result<_, io-error>`
- `flush() -> result<_, io-error>`


## `layer36:io/types@0.1.0`

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

### Functions

- `format-date(millis: u64, tz: string, style: date-style, loc: locale-id) -> string`
- `format-number(value: f64, style: number-style, loc: locale-id) -> string`


## `layer36:locale/info@0.1.0`

### Functions

- `current() -> locale-id`
- `timezone() -> string`


## `layer36:locale/types@0.1.0`

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

### Functions

- `get(url: string) -> result<list<u8>, net-error>`
- `fetch(req: request) -> result<response, net-error>`


## `layer36:net/types@0.1.0`

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

### Functions

- `now-millis() -> u64`
- `monotonic-nanos() -> u64`


## `layer36:time/sleep@0.1.0`

### Functions

- `sleep-millis(millis: u32)`

