//! Phase 2 UAPI dispatcher scaffolding.
//!
//! This is the narrow waist between generated WIT imports and native host
//! adapters. Each method checks UCap first, then calls the adapter trait. The
//! traits are intentionally small while the WIT is still draft-stage.

use crate::uapi::{FsCall, IoCall, LocaleCall, NetCall, TimeCall, UapiCall, UapiError, UapiGuard};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileHandle {
    pub id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenMode {
    Read,
    Write,
    ReadWrite,
    Append,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStat {
    pub size: u64,
    pub modified_millis: u64,
    pub is_dir: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
    pub timeout_millis: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleId {
    pub bcp47: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateStyle {
    Short,
    Medium,
    Long,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberStyle {
    Decimal,
    Percent,
    Currency,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AdapterError {
    #[error("operation is not supported by this host adapter yet")]
    Unsupported,
    #[error("path was invalid")]
    InvalidPath,
    #[error("resource was not found")]
    NotFound,
    #[error("permission was denied by the host")]
    PermissionDenied,
    #[error("host I/O error: {0}")]
    Io(String),
    #[error("host network error: {0}")]
    Network(String),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FsDispatchError {
    #[error("permission denied")]
    PermissionDenied,
    #[error("adapter error: {0}")]
    Adapter(#[from] AdapterError),
    #[error("policy error: {0}")]
    Policy(String),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NetDispatchError {
    #[error("invalid URL")]
    InvalidUrl,
    #[error("permission denied")]
    PermissionDenied,
    #[error("adapter error: {0}")]
    Adapter(#[from] AdapterError),
    #[error("policy error: {0}")]
    Policy(String),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DispatchError {
    #[error("permission denied")]
    PermissionDenied,
    #[error("adapter error: {0}")]
    Adapter(#[from] AdapterError),
    #[error("policy error: {0}")]
    Policy(String),
}

pub type DispatchResult<T> = std::result::Result<T, DispatchError>;

pub trait HostAdapter {
    fn io(&self) -> &dyn IoAdapter;
    fn fs(&self) -> &dyn FsAdapter;
    fn net(&self) -> &dyn NetAdapter;
    fn time(&self) -> &dyn TimeAdapter;
    fn locale(&self) -> &dyn LocaleAdapter;
}

pub trait IoAdapter {
    fn stdin(&self) -> std::result::Result<FileHandle, AdapterError>;
    fn stdout(&self) -> std::result::Result<FileHandle, AdapterError>;
    fn stderr(&self) -> std::result::Result<FileHandle, AdapterError>;
    fn args_raw(&self) -> std::result::Result<String, AdapterError>;
    fn read_stream(
        &self,
        handle: &FileHandle,
        n: u32,
    ) -> std::result::Result<Vec<u8>, AdapterError>;
    fn read_stream_to_string(
        &self,
        handle: &FileHandle,
    ) -> std::result::Result<String, AdapterError>;
    fn write_stream(
        &self,
        handle: &FileHandle,
        bytes: &[u8],
    ) -> std::result::Result<u32, AdapterError>;
    fn write_all_stream(
        &self,
        handle: &FileHandle,
        bytes: &[u8],
    ) -> std::result::Result<(), AdapterError>;
    fn flush_stream(&self, handle: &FileHandle) -> std::result::Result<(), AdapterError>;
    fn log(&self, level: &str, message: &str) -> std::result::Result<(), AdapterError>;
}

pub trait FsAdapter {
    fn open(&self, path: &str, mode: OpenMode) -> std::result::Result<FileHandle, AdapterError>;
    fn read(&self, handle: &FileHandle, n: u32) -> std::result::Result<Vec<u8>, AdapterError>;
    fn write(&self, handle: &FileHandle, bytes: &[u8]) -> std::result::Result<u32, AdapterError>;
    fn seek_set(&self, handle: &FileHandle, pos: u64) -> std::result::Result<u64, AdapterError>;
    fn seek_end(&self, handle: &FileHandle) -> std::result::Result<u64, AdapterError>;
    fn stat_handle(&self, handle: &FileHandle) -> std::result::Result<FileStat, AdapterError>;
    fn stat(&self, path: &str) -> std::result::Result<FileStat, AdapterError>;
    fn list(&self, path: &str) -> std::result::Result<Vec<String>, AdapterError>;
    fn remove_file(&self, path: &str) -> std::result::Result<(), AdapterError>;
    fn remove_dir(&self, path: &str) -> std::result::Result<(), AdapterError>;
    fn mkdir(&self, path: &str) -> std::result::Result<(), AdapterError>;
    fn rename(&self, from: &str, to: &str) -> std::result::Result<(), AdapterError>;
}

pub trait NetAdapter {
    fn fetch(&self, req: HttpRequest) -> std::result::Result<HttpResponse, AdapterError>;
}

pub trait TimeAdapter {
    fn now_millis(&self) -> std::result::Result<u64, AdapterError>;
    fn monotonic_nanos(&self) -> std::result::Result<u64, AdapterError>;
    fn sleep_millis(&self, millis: u32) -> std::result::Result<(), AdapterError>;
}

pub trait LocaleAdapter {
    fn current(&self) -> std::result::Result<LocaleId, AdapterError>;
    fn timezone(&self) -> std::result::Result<String, AdapterError>;
    fn format_date(
        &self,
        millis: u64,
        tz: &str,
        style: DateStyle,
        loc: &LocaleId,
    ) -> std::result::Result<String, AdapterError>;
    fn format_number(
        &self,
        value: f64,
        style: NumberStyle,
        loc: &LocaleId,
    ) -> std::result::Result<String, AdapterError>;
}

pub struct UapiDispatcher<'a> {
    guard: &'a UapiGuard,
    adapter: &'a dyn HostAdapter,
}

impl<'a> UapiDispatcher<'a> {
    pub fn new(guard: &'a UapiGuard, adapter: &'a dyn HostAdapter) -> Self {
        Self { guard, adapter }
    }

    pub fn stdin(&self) -> DispatchResult<FileHandle> {
        self.check(&UapiCall::Io(IoCall::Stdin))?;
        self.adapter.io().stdin().map_err(Into::into)
    }

    pub fn stdout(&self) -> DispatchResult<FileHandle> {
        self.check(&UapiCall::Io(IoCall::Stdout))?;
        self.adapter.io().stdout().map_err(Into::into)
    }

    pub fn stderr(&self) -> DispatchResult<FileHandle> {
        self.check(&UapiCall::Io(IoCall::Stderr))?;
        self.adapter.io().stderr().map_err(Into::into)
    }

    pub fn args_raw(&self) -> DispatchResult<String> {
        self.check(&UapiCall::Io(IoCall::Args))?;
        self.adapter.io().args_raw().map_err(Into::into)
    }

    pub fn log(&self, level: &str, message: &str) -> DispatchResult<()> {
        self.check(&UapiCall::Io(IoCall::Log))?;
        self.adapter.io().log(level, message).map_err(Into::into)
    }

    pub fn fs_open(
        &self,
        path: &str,
        mode: OpenMode,
    ) -> std::result::Result<FileHandle, FsDispatchError> {
        self.check_fs(open_call(path, mode))?;
        self.adapter.fs().open(path, mode).map_err(Into::into)
    }

    pub fn fs_read(
        &self,
        handle: &FileHandle,
        n: u32,
    ) -> std::result::Result<Vec<u8>, FsDispatchError> {
        self.adapter.fs().read(handle, n).map_err(Into::into)
    }

    pub fn fs_write(
        &self,
        handle: &FileHandle,
        bytes: &[u8],
    ) -> std::result::Result<u32, FsDispatchError> {
        self.adapter.fs().write(handle, bytes).map_err(Into::into)
    }

    pub fn fs_seek_set(
        &self,
        handle: &FileHandle,
        pos: u64,
    ) -> std::result::Result<u64, FsDispatchError> {
        self.adapter.fs().seek_set(handle, pos).map_err(Into::into)
    }

    pub fn fs_seek_end(&self, handle: &FileHandle) -> std::result::Result<u64, FsDispatchError> {
        self.adapter.fs().seek_end(handle).map_err(Into::into)
    }

    pub fn fs_stat_handle(
        &self,
        handle: &FileHandle,
    ) -> std::result::Result<FileStat, FsDispatchError> {
        self.adapter.fs().stat_handle(handle).map_err(Into::into)
    }

    pub fn fs_stat(&self, path: &str) -> std::result::Result<FileStat, FsDispatchError> {
        self.check_fs(UapiCall::Fs(FsCall::Read {
            path: path.to_string(),
        }))?;
        self.adapter.fs().stat(path).map_err(Into::into)
    }

    pub fn fs_list(&self, path: &str) -> std::result::Result<Vec<String>, FsDispatchError> {
        self.check_fs(UapiCall::Fs(FsCall::List {
            path: path.to_string(),
        }))?;
        self.adapter.fs().list(path).map_err(Into::into)
    }

    pub fn fs_remove_file(&self, path: &str) -> std::result::Result<(), FsDispatchError> {
        self.check_fs(UapiCall::Fs(FsCall::Remove {
            path: path.to_string(),
        }))?;
        self.adapter.fs().remove_file(path).map_err(Into::into)
    }

    pub fn fs_remove_dir(&self, path: &str) -> std::result::Result<(), FsDispatchError> {
        self.check_fs(UapiCall::Fs(FsCall::Remove {
            path: path.to_string(),
        }))?;
        self.adapter.fs().remove_dir(path).map_err(Into::into)
    }

    pub fn fs_mkdir(&self, path: &str) -> std::result::Result<(), FsDispatchError> {
        self.check_fs(UapiCall::Fs(FsCall::Mkdir {
            path: path.to_string(),
        }))?;
        self.adapter.fs().mkdir(path).map_err(Into::into)
    }

    pub fn fs_rename(&self, from: &str, to: &str) -> std::result::Result<(), FsDispatchError> {
        self.check_fs(UapiCall::Fs(FsCall::Remove {
            path: from.to_string(),
        }))?;
        self.check_fs(UapiCall::Fs(FsCall::Write {
            path: to.to_string(),
        }))?;
        self.adapter.fs().rename(from, to).map_err(Into::into)
    }

    pub fn read_stream(&self, handle: &FileHandle, n: u32) -> DispatchResult<Vec<u8>> {
        self.adapter.io().read_stream(handle, n).map_err(Into::into)
    }

    pub fn read_stream_to_string(&self, handle: &FileHandle) -> DispatchResult<String> {
        self.adapter
            .io()
            .read_stream_to_string(handle)
            .map_err(Into::into)
    }

    pub fn write_stream(&self, handle: &FileHandle, bytes: &[u8]) -> DispatchResult<u32> {
        self.adapter
            .io()
            .write_stream(handle, bytes)
            .map_err(Into::into)
    }

    pub fn write_all_stream(&self, handle: &FileHandle, bytes: &[u8]) -> DispatchResult<()> {
        self.adapter
            .io()
            .write_all_stream(handle, bytes)
            .map_err(Into::into)
    }

    pub fn flush_stream(&self, handle: &FileHandle) -> DispatchResult<()> {
        self.adapter.io().flush_stream(handle).map_err(Into::into)
    }

    pub fn net_fetch(
        &self,
        req: HttpRequest,
    ) -> std::result::Result<HttpResponse, NetDispatchError> {
        let endpoint = endpoint_from_url(&req.url).ok_or(NetDispatchError::InvalidUrl)?;
        self.guard
            .check(&UapiCall::Net(NetCall::Connect {
                host: endpoint.host,
                port: endpoint.port,
            }))
            .map_err(map_net_policy)?;
        self.adapter.net().fetch(req).map_err(Into::into)
    }

    pub fn now_millis(&self) -> DispatchResult<u64> {
        self.check(&UapiCall::Time(TimeCall::Clock))?;
        self.adapter.time().now_millis().map_err(Into::into)
    }

    pub fn monotonic_nanos(&self) -> DispatchResult<u64> {
        self.check(&UapiCall::Time(TimeCall::Monotonic))?;
        self.adapter.time().monotonic_nanos().map_err(Into::into)
    }

    pub fn sleep_millis(&self, millis: u32) -> DispatchResult<()> {
        self.check(&UapiCall::Time(TimeCall::Sleep))?;
        self.adapter.time().sleep_millis(millis).map_err(Into::into)
    }

    pub fn current_locale(&self) -> DispatchResult<LocaleId> {
        self.check(&UapiCall::Locale(LocaleCall::Info))?;
        self.adapter.locale().current().map_err(Into::into)
    }

    pub fn timezone(&self) -> DispatchResult<String> {
        self.check(&UapiCall::Locale(LocaleCall::Info))?;
        self.adapter.locale().timezone().map_err(Into::into)
    }

    pub fn format_date(
        &self,
        millis: u64,
        tz: &str,
        style: DateStyle,
        loc: &LocaleId,
    ) -> DispatchResult<String> {
        self.check(&UapiCall::Locale(LocaleCall::Format))?;
        self.adapter
            .locale()
            .format_date(millis, tz, style, loc)
            .map_err(Into::into)
    }

    pub fn format_number(
        &self,
        value: f64,
        style: NumberStyle,
        loc: &LocaleId,
    ) -> DispatchResult<String> {
        self.check(&UapiCall::Locale(LocaleCall::Format))?;
        self.adapter
            .locale()
            .format_number(value, style, loc)
            .map_err(Into::into)
    }

    fn check(&self, call: &UapiCall) -> DispatchResult<()> {
        self.guard
            .check(call)
            .map(|_| ())
            .map_err(map_dispatch_policy)
    }

    fn check_fs(&self, call: UapiCall) -> std::result::Result<(), FsDispatchError> {
        self.guard.check(&call).map(|_| ()).map_err(map_fs_policy)
    }
}

fn open_call(path: &str, mode: OpenMode) -> UapiCall {
    let path = path.to_string();
    match mode {
        OpenMode::Read => UapiCall::Fs(FsCall::Read { path }),
        OpenMode::Write | OpenMode::ReadWrite | OpenMode::Append => {
            UapiCall::Fs(FsCall::Write { path })
        }
    }
}

fn map_dispatch_policy(err: UapiError) -> DispatchError {
    if matches!(
        err,
        UapiError::Policy(layer36_policy::PolicyError::Denied { .. })
    ) {
        DispatchError::PermissionDenied
    } else {
        DispatchError::Policy(err.to_string())
    }
}

fn map_fs_policy(err: UapiError) -> FsDispatchError {
    if matches!(
        err,
        UapiError::Policy(layer36_policy::PolicyError::Denied { .. })
    ) {
        FsDispatchError::PermissionDenied
    } else {
        FsDispatchError::Policy(err.to_string())
    }
}

fn map_net_policy(err: UapiError) -> NetDispatchError {
    if matches!(
        err,
        UapiError::Policy(layer36_policy::PolicyError::Denied { .. })
    ) {
        NetDispatchError::PermissionDenied
    } else {
        NetDispatchError::Policy(err.to_string())
    }
}

struct Endpoint {
    host: String,
    port: u16,
}

fn endpoint_from_url(url: &str) -> Option<Endpoint> {
    let (scheme, rest) = url.split_once("://")?;
    let default_port = match scheme {
        "http" => 80,
        "https" => 443,
        _ => return None,
    };

    let authority = rest.split(['/', '?', '#']).next()?;
    if authority.is_empty() || authority.contains('@') {
        return None;
    }

    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) if !host.is_empty() => {
            let parsed = port.parse::<u16>().ok()?;
            (host, parsed)
        }
        _ => (authority, default_port),
    };

    Some(Endpoint {
        host: host.to_string(),
        port,
    })
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use layer36_policy::SessionPolicy;

    use super::*;

    #[derive(Default)]
    struct Calls {
        args: usize,
        fs_open: usize,
        net_fetch: usize,
        stdout: usize,
        sleep: usize,
    }

    #[derive(Clone, Default)]
    struct RecordingAdapter {
        calls: Rc<RefCell<Calls>>,
    }

    impl HostAdapter for RecordingAdapter {
        fn io(&self) -> &dyn IoAdapter {
            self
        }

        fn fs(&self) -> &dyn FsAdapter {
            self
        }

        fn net(&self) -> &dyn NetAdapter {
            self
        }

        fn time(&self) -> &dyn TimeAdapter {
            self
        }

        fn locale(&self) -> &dyn LocaleAdapter {
            self
        }
    }

    impl IoAdapter for RecordingAdapter {
        fn stdin(&self) -> std::result::Result<FileHandle, AdapterError> {
            Ok(FileHandle { id: 1 })
        }

        fn stdout(&self) -> std::result::Result<FileHandle, AdapterError> {
            self.calls.borrow_mut().stdout += 1;
            Ok(FileHandle { id: 2 })
        }

        fn stderr(&self) -> std::result::Result<FileHandle, AdapterError> {
            Ok(FileHandle { id: 3 })
        }

        fn args_raw(&self) -> std::result::Result<String, AdapterError> {
            self.calls.borrow_mut().args += 1;
            Ok("notes.txt".to_string())
        }

        fn read_stream(
            &self,
            _handle: &FileHandle,
            _n: u32,
        ) -> std::result::Result<Vec<u8>, AdapterError> {
            Ok(b"stdin".to_vec())
        }

        fn read_stream_to_string(
            &self,
            _handle: &FileHandle,
        ) -> std::result::Result<String, AdapterError> {
            Ok("stdin".to_string())
        }

        fn write_stream(
            &self,
            _handle: &FileHandle,
            bytes: &[u8],
        ) -> std::result::Result<u32, AdapterError> {
            Ok(bytes.len() as u32)
        }

        fn write_all_stream(
            &self,
            _handle: &FileHandle,
            _bytes: &[u8],
        ) -> std::result::Result<(), AdapterError> {
            Ok(())
        }

        fn flush_stream(&self, _handle: &FileHandle) -> std::result::Result<(), AdapterError> {
            Ok(())
        }

        fn log(&self, _level: &str, _message: &str) -> std::result::Result<(), AdapterError> {
            Ok(())
        }
    }

    impl FsAdapter for RecordingAdapter {
        fn open(
            &self,
            _path: &str,
            _mode: OpenMode,
        ) -> std::result::Result<FileHandle, AdapterError> {
            self.calls.borrow_mut().fs_open += 1;
            Ok(FileHandle { id: 4 })
        }

        fn read(
            &self,
            _handle: &FileHandle,
            _n: u32,
        ) -> std::result::Result<Vec<u8>, AdapterError> {
            Ok(b"file".to_vec())
        }

        fn write(
            &self,
            _handle: &FileHandle,
            bytes: &[u8],
        ) -> std::result::Result<u32, AdapterError> {
            Ok(bytes.len() as u32)
        }

        fn seek_set(
            &self,
            _handle: &FileHandle,
            pos: u64,
        ) -> std::result::Result<u64, AdapterError> {
            Ok(pos)
        }

        fn seek_end(&self, _handle: &FileHandle) -> std::result::Result<u64, AdapterError> {
            Ok(4)
        }

        fn stat_handle(&self, _handle: &FileHandle) -> std::result::Result<FileStat, AdapterError> {
            Ok(FileStat {
                size: 4,
                modified_millis: 0,
                is_dir: false,
            })
        }

        fn stat(&self, _path: &str) -> std::result::Result<FileStat, AdapterError> {
            Ok(FileStat {
                size: 0,
                modified_millis: 0,
                is_dir: false,
            })
        }

        fn list(&self, _path: &str) -> std::result::Result<Vec<String>, AdapterError> {
            Ok(Vec::new())
        }

        fn remove_file(&self, _path: &str) -> std::result::Result<(), AdapterError> {
            Ok(())
        }

        fn remove_dir(&self, _path: &str) -> std::result::Result<(), AdapterError> {
            Ok(())
        }

        fn mkdir(&self, _path: &str) -> std::result::Result<(), AdapterError> {
            Ok(())
        }

        fn rename(&self, _from: &str, _to: &str) -> std::result::Result<(), AdapterError> {
            Ok(())
        }
    }

    impl NetAdapter for RecordingAdapter {
        fn fetch(&self, _req: HttpRequest) -> std::result::Result<HttpResponse, AdapterError> {
            self.calls.borrow_mut().net_fetch += 1;
            Ok(HttpResponse {
                status: 200,
                headers: Vec::new(),
                body: Vec::new(),
            })
        }
    }

    impl TimeAdapter for RecordingAdapter {
        fn now_millis(&self) -> std::result::Result<u64, AdapterError> {
            Ok(1)
        }

        fn monotonic_nanos(&self) -> std::result::Result<u64, AdapterError> {
            Ok(2)
        }

        fn sleep_millis(&self, _millis: u32) -> std::result::Result<(), AdapterError> {
            self.calls.borrow_mut().sleep += 1;
            Ok(())
        }
    }

    impl LocaleAdapter for RecordingAdapter {
        fn current(&self) -> std::result::Result<LocaleId, AdapterError> {
            Ok(LocaleId {
                bcp47: "en-US".to_string(),
            })
        }

        fn timezone(&self) -> std::result::Result<String, AdapterError> {
            Ok("UTC".to_string())
        }

        fn format_date(
            &self,
            _millis: u64,
            _tz: &str,
            _style: DateStyle,
            _loc: &LocaleId,
        ) -> std::result::Result<String, AdapterError> {
            Ok("date".to_string())
        }

        fn format_number(
            &self,
            _value: f64,
            _style: NumberStyle,
            _loc: &LocaleId,
        ) -> std::result::Result<String, AdapterError> {
            Ok("number".to_string())
        }
    }

    #[test]
    fn default_io_grant_reaches_adapter() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::default());
        let dispatcher = UapiDispatcher::new(&guard, &adapter);

        dispatcher.stdout().expect("stdout is default-granted");

        assert_eq!(adapter.calls.borrow().stdout, 1);
    }

    #[test]
    fn default_args_grant_reaches_adapter() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::default());
        let dispatcher = UapiDispatcher::new(&guard, &adapter);

        let args = dispatcher.args_raw().expect("args are default-granted");

        assert_eq!(args, "notes.txt");
        assert_eq!(adapter.calls.borrow().args, 1);
    }

    #[test]
    fn fs_open_denies_before_adapter_when_cap_missing() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::default());
        let dispatcher = UapiDispatcher::new(&guard, &adapter);
        let err = dispatcher
            .fs_open("./notes/today.txt", OpenMode::Read)
            .expect_err("read should need fs grant");

        assert!(matches!(err, FsDispatchError::PermissionDenied));
        assert_eq!(adapter.calls.borrow().fs_open, 0);
    }

    #[test]
    fn fs_open_allows_matching_resource_grant() {
        let adapter = RecordingAdapter::default();
        let policy =
            SessionPolicy::from_cli_grants(&["fs.read:./notes/**".to_string()]).expect("policy");
        let guard = UapiGuard::new(policy);
        let dispatcher = UapiDispatcher::new(&guard, &adapter);

        dispatcher
            .fs_open("./notes/today.txt", OpenMode::Read)
            .expect("read grant should pass");

        assert_eq!(adapter.calls.borrow().fs_open, 1);
    }

    #[test]
    fn net_fetch_checks_url_endpoint_before_adapter() {
        let adapter = RecordingAdapter::default();
        let policy =
            SessionPolicy::from_cli_grants(&["net.connect:api.example.com:443".to_string()])
                .expect("policy");
        let guard = UapiGuard::new(policy);
        let dispatcher = UapiDispatcher::new(&guard, &adapter);
        let req = HttpRequest {
            method: HttpMethod::Get,
            url: "https://api.example.com/v1/ping".to_string(),
            headers: Vec::new(),
            body: Vec::new(),
            timeout_millis: None,
        };

        dispatcher.net_fetch(req).expect("net grant should pass");

        assert_eq!(adapter.calls.borrow().net_fetch, 1);
    }

    #[test]
    fn net_fetch_maps_missing_grant_to_net_permission_denied() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::default());
        let dispatcher = UapiDispatcher::new(&guard, &adapter);
        let req = HttpRequest {
            method: HttpMethod::Get,
            url: "https://api.example.com/v1/ping".to_string(),
            headers: Vec::new(),
            body: Vec::new(),
            timeout_millis: None,
        };
        let err = dispatcher
            .net_fetch(req)
            .expect_err("net should need grant");

        assert!(matches!(err, NetDispatchError::PermissionDenied));
        assert_eq!(adapter.calls.borrow().net_fetch, 0);
    }

    #[test]
    fn sleep_requires_time_sleep_grant() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::default());
        let dispatcher = UapiDispatcher::new(&guard, &adapter);

        dispatcher
            .sleep_millis(1)
            .expect("sleep is default-granted");

        assert_eq!(adapter.calls.borrow().sleep, 1);
    }

    #[test]
    fn endpoint_parser_applies_default_https_port() {
        let endpoint = endpoint_from_url("https://example.com/path").expect("endpoint");

        assert_eq!(endpoint.host, "example.com");
        assert_eq!(endpoint.port, 443);
    }
}
