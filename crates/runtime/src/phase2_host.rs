//! Generated Phase 2 import host backed by the UAPI dispatcher.
//!
//! This is the first real wiring from Wasmtime-generated traits into the
//! runtime dispatcher. Path-level filesystem calls, HTTP, time, locale, log, and
//! stdio handles flow through UCap before reaching a host adapter.

use crate::{
    phase2_bindings::layer36::{fs, io, locale, net, time},
    phase2_bridge as bridge,
    uapi::UapiGuard,
    uapi_dispatch::{FileHandle, HostAdapter, UapiDispatcher},
};

use wasmtime::component::Resource;

pub struct Phase2Host<'a> {
    dispatcher: UapiDispatcher<'a>,
}

impl<'a> Phase2Host<'a> {
    pub fn new(guard: &'a UapiGuard, adapter: &'a dyn HostAdapter) -> Self {
        Self {
            dispatcher: UapiDispatcher::new(guard, adapter),
        }
    }
}

impl fs::files::Host for Phase2Host<'_> {
    fn open(
        &mut self,
        path: String,
        mode: fs::types::OpenMode,
    ) -> wasmtime::Result<Result<Resource<fs::files::File>, fs::types::FsError>> {
        let mode = bridge::open_mode_from_wit(mode);
        let handle = match self.dispatcher.fs_open(&path, mode) {
            Ok(handle) => handle,
            Err(err) => return Ok(Err(bridge::fs_error_to_wit(err))),
        };

        Ok(Ok(resource_from_handle(handle)?))
    }

    fn stat(
        &mut self,
        path: String,
    ) -> wasmtime::Result<Result<fs::types::FileStat, fs::types::FsError>> {
        Ok(self
            .dispatcher
            .fs_stat(&path)
            .map(bridge::file_stat_to_wit)
            .map_err(bridge::fs_error_to_wit))
    }

    fn list(&mut self, path: String) -> wasmtime::Result<Result<Vec<String>, fs::types::FsError>> {
        Ok(self
            .dispatcher
            .fs_list(&path)
            .map_err(bridge::fs_error_to_wit))
    }

    fn remove_file(&mut self, path: String) -> wasmtime::Result<Result<(), fs::types::FsError>> {
        Ok(self
            .dispatcher
            .fs_remove_file(&path)
            .map_err(bridge::fs_error_to_wit))
    }

    fn remove_dir(&mut self, path: String) -> wasmtime::Result<Result<(), fs::types::FsError>> {
        Ok(self
            .dispatcher
            .fs_remove_dir(&path)
            .map_err(bridge::fs_error_to_wit))
    }

    fn mkdir(&mut self, path: String) -> wasmtime::Result<Result<(), fs::types::FsError>> {
        Ok(self
            .dispatcher
            .fs_mkdir(&path)
            .map_err(bridge::fs_error_to_wit))
    }

    fn rename(
        &mut self,
        from: String,
        to: String,
    ) -> wasmtime::Result<Result<(), fs::types::FsError>> {
        Ok(self
            .dispatcher
            .fs_rename(&from, &to)
            .map_err(bridge::fs_error_to_wit))
    }
}

impl fs::files::HostFile for Phase2Host<'_> {
    fn read(
        &mut self,
        _self_: Resource<fs::files::File>,
        _n: u32,
    ) -> wasmtime::Result<Result<Vec<u8>, fs::types::FsError>> {
        Ok(Err(resource_io_not_wired()))
    }

    fn write(
        &mut self,
        _self_: Resource<fs::files::File>,
        _bytes: Vec<u8>,
    ) -> wasmtime::Result<Result<u32, fs::types::FsError>> {
        Ok(Err(resource_io_not_wired()))
    }

    fn seek_set(
        &mut self,
        _self_: Resource<fs::files::File>,
        _pos: u64,
    ) -> wasmtime::Result<Result<u64, fs::types::FsError>> {
        Ok(Err(resource_io_not_wired()))
    }

    fn seek_end(
        &mut self,
        _self_: Resource<fs::files::File>,
    ) -> wasmtime::Result<Result<u64, fs::types::FsError>> {
        Ok(Err(resource_io_not_wired()))
    }

    fn stat(
        &mut self,
        _self_: Resource<fs::files::File>,
    ) -> wasmtime::Result<Result<fs::types::FileStat, fs::types::FsError>> {
        Ok(Err(resource_io_not_wired()))
    }

    fn drop(&mut self, _rep: Resource<fs::files::File>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl io::stdio::Host for Phase2Host<'_> {
    fn stdin(&mut self) -> wasmtime::Result<Resource<io::streams::InputStream>> {
        self.dispatcher
            .stdin()
            .map(resource_from_handle)
            .map_err(bridge::dispatch_error_to_trap)?
    }

    fn stdout(&mut self) -> wasmtime::Result<Resource<io::streams::OutputStream>> {
        self.dispatcher
            .stdout()
            .map(resource_from_handle)
            .map_err(bridge::dispatch_error_to_trap)?
    }

    fn stderr(&mut self) -> wasmtime::Result<Resource<io::streams::OutputStream>> {
        self.dispatcher
            .stderr()
            .map(resource_from_handle)
            .map_err(bridge::dispatch_error_to_trap)?
    }
}

impl io::streams::HostInputStream for Phase2Host<'_> {
    fn read(
        &mut self,
        _self_: Resource<io::streams::InputStream>,
        _n: u32,
    ) -> wasmtime::Result<Result<Vec<u8>, io::types::IoError>> {
        Ok(Err(stream_io_not_wired()))
    }

    fn read_to_string(
        &mut self,
        _self_: Resource<io::streams::InputStream>,
    ) -> wasmtime::Result<Result<String, io::types::IoError>> {
        Ok(Err(stream_io_not_wired()))
    }

    fn drop(&mut self, _rep: Resource<io::streams::InputStream>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl io::streams::HostOutputStream for Phase2Host<'_> {
    fn write(
        &mut self,
        _self_: Resource<io::streams::OutputStream>,
        _bytes: Vec<u8>,
    ) -> wasmtime::Result<Result<u32, io::types::IoError>> {
        Ok(Err(stream_io_not_wired()))
    }

    fn write_all(
        &mut self,
        _self_: Resource<io::streams::OutputStream>,
        _bytes: Vec<u8>,
    ) -> wasmtime::Result<Result<(), io::types::IoError>> {
        Ok(Err(stream_io_not_wired()))
    }

    fn flush(
        &mut self,
        _self_: Resource<io::streams::OutputStream>,
    ) -> wasmtime::Result<Result<(), io::types::IoError>> {
        Ok(Err(stream_io_not_wired()))
    }

    fn drop(&mut self, _rep: Resource<io::streams::OutputStream>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl io::log::Host for Phase2Host<'_> {
    fn emit(
        &mut self,
        level: io::types::LogLevel,
        message: String,
        _fields: Vec<io::log::Field>,
    ) -> wasmtime::Result<()> {
        self.dispatcher
            .log(bridge::log_level_to_str(level), &message)
            .map_err(bridge::dispatch_error_to_trap)
    }
}

impl net::http_client::Host for Phase2Host<'_> {
    fn fetch(
        &mut self,
        req: net::types::Request,
    ) -> wasmtime::Result<Result<net::types::Response, net::types::NetError>> {
        Ok(self
            .dispatcher
            .net_fetch(bridge::request_from_wit(req))
            .map(bridge::response_to_wit)
            .map_err(bridge::net_error_to_wit))
    }
}

impl time::clock::Host for Phase2Host<'_> {
    fn now_millis(&mut self) -> wasmtime::Result<u64> {
        self.dispatcher
            .now_millis()
            .map_err(bridge::dispatch_error_to_trap)
    }

    fn monotonic_nanos(&mut self) -> wasmtime::Result<u64> {
        self.dispatcher
            .monotonic_nanos()
            .map_err(bridge::dispatch_error_to_trap)
    }
}

impl time::sleep::Host for Phase2Host<'_> {
    fn sleep_millis(&mut self, millis: u32) -> wasmtime::Result<()> {
        self.dispatcher
            .sleep_millis(millis)
            .map_err(bridge::dispatch_error_to_trap)
    }
}

impl locale::info::Host for Phase2Host<'_> {
    fn current(&mut self) -> wasmtime::Result<locale::types::LocaleId> {
        self.dispatcher
            .current_locale()
            .map(bridge::locale_to_wit)
            .map_err(bridge::dispatch_error_to_trap)
    }

    fn timezone(&mut self) -> wasmtime::Result<String> {
        self.dispatcher
            .timezone()
            .map_err(bridge::dispatch_error_to_trap)
    }
}

impl locale::format::Host for Phase2Host<'_> {
    fn format_date(
        &mut self,
        millis: u64,
        tz: String,
        style: locale::types::DateStyle,
        loc: locale::types::LocaleId,
    ) -> wasmtime::Result<String> {
        let loc = bridge::locale_from_wit(loc);
        self.dispatcher
            .format_date(millis, &tz, bridge::date_style_from_wit(style), &loc)
            .map_err(bridge::dispatch_error_to_trap)
    }

    fn format_number(
        &mut self,
        value: f64,
        style: locale::types::NumberStyle,
        loc: locale::types::LocaleId,
    ) -> wasmtime::Result<String> {
        let loc = bridge::locale_from_wit(loc);
        self.dispatcher
            .format_number(value, bridge::number_style_from_wit(style), &loc)
            .map_err(bridge::dispatch_error_to_trap)
    }
}

fn resource_from_handle<T: 'static>(handle: FileHandle) -> wasmtime::Result<Resource<T>> {
    let id = u32::try_from(handle.id)
        .map_err(|_| wasmtime::Error::msg(format!("resource id {} is too large", handle.id)))?;
    Ok(Resource::new_own(id))
}

fn resource_io_not_wired() -> fs::types::FsError {
    fs::types::FsError::Io("file resource operations are not wired yet".to_string())
}

fn stream_io_not_wired() -> io::types::IoError {
    io::types::IoError::Other("stream resource operations are not wired yet".to_string())
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use layer36_policy::SessionPolicy;

    use crate::uapi_dispatch::{
        AdapterError, DateStyle, FsAdapter, Header, HttpRequest, HttpResponse, IoAdapter,
        LocaleAdapter, LocaleId, NetAdapter, OpenMode, TimeAdapter,
    };

    use super::*;

    #[derive(Clone, Default)]
    struct RecordingAdapter {
        calls: Rc<RefCell<Calls>>,
    }

    #[derive(Default)]
    struct Calls {
        fs_stat: usize,
        log: usize,
        net_fetch: usize,
        sleep: usize,
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
        fn stdin(&self) -> Result<FileHandle, AdapterError> {
            Ok(FileHandle { id: 10 })
        }

        fn stdout(&self) -> Result<FileHandle, AdapterError> {
            Ok(FileHandle { id: 11 })
        }

        fn stderr(&self) -> Result<FileHandle, AdapterError> {
            Ok(FileHandle { id: 12 })
        }

        fn log(&self, _level: &str, _message: &str) -> Result<(), AdapterError> {
            self.calls.borrow_mut().log += 1;
            Ok(())
        }
    }

    impl FsAdapter for RecordingAdapter {
        fn open(&self, _path: &str, _mode: OpenMode) -> Result<FileHandle, AdapterError> {
            Ok(FileHandle { id: 20 })
        }

        fn stat(&self, _path: &str) -> Result<crate::uapi_dispatch::FileStat, AdapterError> {
            self.calls.borrow_mut().fs_stat += 1;
            Ok(crate::uapi_dispatch::FileStat {
                size: 64,
                modified_millis: 1234,
                is_dir: false,
            })
        }

        fn list(&self, _path: &str) -> Result<Vec<String>, AdapterError> {
            Ok(vec!["one.txt".to_string()])
        }

        fn remove_file(&self, _path: &str) -> Result<(), AdapterError> {
            Ok(())
        }

        fn remove_dir(&self, _path: &str) -> Result<(), AdapterError> {
            Ok(())
        }

        fn mkdir(&self, _path: &str) -> Result<(), AdapterError> {
            Ok(())
        }

        fn rename(&self, _from: &str, _to: &str) -> Result<(), AdapterError> {
            Ok(())
        }
    }

    impl NetAdapter for RecordingAdapter {
        fn fetch(&self, req: HttpRequest) -> Result<HttpResponse, AdapterError> {
            self.calls.borrow_mut().net_fetch += 1;
            Ok(HttpResponse {
                status: 200,
                headers: vec![Header {
                    name: "x-url".to_string(),
                    value: req.url,
                }],
                body: b"ok".to_vec(),
            })
        }
    }

    impl TimeAdapter for RecordingAdapter {
        fn now_millis(&self) -> Result<u64, AdapterError> {
            Ok(100)
        }

        fn monotonic_nanos(&self) -> Result<u64, AdapterError> {
            Ok(200)
        }

        fn sleep_millis(&self, _millis: u32) -> Result<(), AdapterError> {
            self.calls.borrow_mut().sleep += 1;
            Ok(())
        }
    }

    impl LocaleAdapter for RecordingAdapter {
        fn current(&self) -> Result<LocaleId, AdapterError> {
            Ok(LocaleId {
                bcp47: "en-US".to_string(),
            })
        }

        fn timezone(&self) -> Result<String, AdapterError> {
            Ok("UTC".to_string())
        }

        fn format_date(
            &self,
            millis: u64,
            tz: &str,
            style: DateStyle,
            loc: &LocaleId,
        ) -> Result<String, AdapterError> {
            Ok(format!("{millis}:{tz}:{style:?}:{}", loc.bcp47))
        }

        fn format_number(
            &self,
            value: f64,
            style: crate::uapi_dispatch::NumberStyle,
            loc: &LocaleId,
        ) -> Result<String, AdapterError> {
            Ok(format!("{value}:{style:?}:{}", loc.bcp47))
        }
    }

    #[test]
    fn generated_net_host_calls_dispatcher() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::from_grants(["net.connect:example.com:443"
            .parse()
            .unwrap()]));
        let mut host = Phase2Host::new(&guard, &adapter);

        let response = net::http_client::Host::fetch(
            &mut host,
            net::types::Request {
                method: net::types::HttpMethod::Get,
                url: "https://example.com/path".to_string(),
                headers: Vec::new(),
                body: Vec::new(),
                timeout_millis: None,
            },
        )
        .unwrap()
        .unwrap();

        assert_eq!(response.status, 200);
        assert_eq!(adapter.calls.borrow().net_fetch, 1);
    }

    #[test]
    fn generated_net_host_returns_wit_permission_error() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::default());
        let mut host = Phase2Host::new(&guard, &adapter);

        let err = net::http_client::Host::fetch(
            &mut host,
            net::types::Request {
                method: net::types::HttpMethod::Get,
                url: "https://example.com/path".to_string(),
                headers: Vec::new(),
                body: Vec::new(),
                timeout_millis: None,
            },
        )
        .unwrap()
        .unwrap_err();

        assert!(matches!(err, net::types::NetError::PermissionDenied));
        assert_eq!(adapter.calls.borrow().net_fetch, 0);
    }

    #[test]
    fn generated_fs_and_stdio_hosts_call_dispatcher() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::from_grants([
            "fs.read:/tmp/data.txt".parse().unwrap(),
            "io.stdin".parse().unwrap(),
        ]));
        let mut host = Phase2Host::new(&guard, &adapter);

        let stat = fs::files::Host::stat(&mut host, "/tmp/data.txt".to_string())
            .unwrap()
            .unwrap();
        let stdin = io::stdio::Host::stdin(&mut host).unwrap();

        assert_eq!(stat.size, 64);
        assert_eq!(stdin.rep(), 10);
        assert_eq!(adapter.calls.borrow().fs_stat, 1);
    }

    #[test]
    fn generated_time_locale_and_log_hosts_call_dispatcher() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::default());
        let mut host = Phase2Host::new(&guard, &adapter);

        assert_eq!(time::clock::Host::now_millis(&mut host).unwrap(), 100);
        time::sleep::Host::sleep_millis(&mut host, 1).unwrap();
        assert_eq!(
            locale::info::Host::current(&mut host).unwrap().bcp47,
            "en-US"
        );
        io::log::Host::emit(
            &mut host,
            io::types::LogLevel::Info,
            "hello".to_string(),
            Vec::new(),
        )
        .unwrap();

        assert_eq!(adapter.calls.borrow().sleep, 1);
        assert_eq!(adapter.calls.borrow().log, 1);
    }

    #[test]
    fn resource_methods_are_explicitly_not_wired_yet() {
        let adapter = RecordingAdapter::default();
        let guard = UapiGuard::new(SessionPolicy::default());
        let mut host = Phase2Host::new(&guard, &adapter);

        let err = io::streams::HostOutputStream::write_all(
            &mut host,
            Resource::new_own(11),
            b"hello".to_vec(),
        )
        .unwrap()
        .unwrap_err();

        assert!(matches!(err, io::types::IoError::Other(message) if message.contains("not wired")));
    }
}
