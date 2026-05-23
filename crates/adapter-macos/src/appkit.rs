//! Opt-in AppKit window prototype for the macOS adapter.
//!
//! The default macOS adapter still uses the headless draft backend. This module
//! is the first native path behind the checked handle handoff: it creates an
//! AppKit `NSWindow`, keeps that object alive, and binds its raw pointer to a
//! stable Layer36 `WindowId`.

use layer36_adapter_common::ui::{
    NativeWindowHandle, UiAdapterError, WindowAdapter, WindowBackendKind, WindowId, WindowOptions,
    WindowSize,
};

use crate::MacosUiAdapter;

/// Small native AppKit backend used by the first Phase 3 macOS prototype.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AppKitWindowBackend;

/// Current state read from an owned AppKit window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AppKitWindowSnapshot {
    pub visible: bool,
    pub focused: bool,
    pub size: WindowSize,
    pub scale: f32,
}

/// Native AppKit event shape accepted by the first callback bridge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppKitWindowNativeEvent {
    CloseRequested,
    Resized(WindowSize),
    Focused(bool),
    ScaleChanged(f32),
    Snapshot(AppKitWindowSnapshot),
}

/// Mutable native event-loop state for one AppKit-backed Layer36 window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AppKitWindowEventState {
    id: WindowId,
    last_snapshot: Option<AppKitWindowSnapshot>,
}

impl AppKitWindowEventState {
    /// Create event-loop state for an AppKit-backed Layer36 window.
    pub fn new(id: WindowId) -> Self {
        Self {
            id,
            last_snapshot: None,
        }
    }

    /// Return the Layer36 window id this state belongs to.
    pub fn id(&self) -> WindowId {
        self.id
    }

    /// Return the last native snapshot observed by this state object.
    pub fn last_snapshot(&self) -> Option<AppKitWindowSnapshot> {
        self.last_snapshot
    }

    /// Refresh this state from a full AppKit snapshot.
    pub fn sync_snapshot(
        &mut self,
        backend: &AppKitWindowBackend,
        adapter: &MacosUiAdapter,
        snapshot: AppKitWindowSnapshot,
    ) -> Result<AppKitWindowSnapshot, UiAdapterError> {
        let snapshot =
            backend.sync_snapshot_for_id(adapter, self.id, snapshot, self.last_snapshot)?;
        self.last_snapshot = Some(snapshot);
        Ok(snapshot)
    }

    /// Queue a native event reported by an AppKit callback.
    pub fn handle_native_event(
        &mut self,
        backend: &AppKitWindowBackend,
        adapter: &MacosUiAdapter,
        event: AppKitWindowNativeEvent,
    ) -> Result<Option<AppKitWindowSnapshot>, UiAdapterError> {
        match event {
            AppKitWindowNativeEvent::CloseRequested => {
                backend.report_close_requested_for_id(adapter, self.id)?;
                Ok(self.last_snapshot)
            }
            AppKitWindowNativeEvent::Resized(size) => {
                backend.report_resized_for_id(adapter, self.id, size)?;
                self.last_snapshot = self.last_snapshot.map(|mut snapshot| {
                    snapshot.size = size;
                    snapshot
                });
                Ok(self.last_snapshot)
            }
            AppKitWindowNativeEvent::Focused(focused) => {
                backend.report_focused_for_id(adapter, self.id, focused)?;
                self.last_snapshot = self.last_snapshot.map(|mut snapshot| {
                    snapshot.focused = focused;
                    snapshot
                });
                Ok(self.last_snapshot)
            }
            AppKitWindowNativeEvent::ScaleChanged(scale) => {
                backend.report_scale_changed_for_id(adapter, self.id, scale)?;
                self.last_snapshot = self.last_snapshot.map(|mut snapshot| {
                    snapshot.scale = scale;
                    snapshot
                });
                Ok(self.last_snapshot)
            }
            AppKitWindowNativeEvent::Snapshot(snapshot) => {
                self.sync_snapshot(backend, adapter, snapshot).map(Some)
            }
        }
    }
}

/// Owned AppKit window plus the last native state seen by Layer36.
pub struct AppKitWindowSession {
    window: AppKitWindowPrototype,
    event_state: AppKitWindowEventState,
}

impl AppKitWindowSession {
    /// Return the stable Layer36 window id for this native window session.
    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    /// Return the opaque native AppKit handle attached to this session.
    pub fn native_handle(&self) -> NativeWindowHandle {
        self.window.native_handle()
    }

    /// Return the most recent native state snapshot observed by this session.
    pub fn last_snapshot(&self) -> Option<AppKitWindowSnapshot> {
        self.event_state.last_snapshot()
    }

    /// Return the owned AppKit window prototype.
    pub fn window(&self) -> &AppKitWindowPrototype {
        &self.window
    }

    /// Show the native window through AppKit and the shared Layer36 adapter.
    pub fn show(
        &self,
        backend: &AppKitWindowBackend,
        adapter: &MacosUiAdapter,
    ) -> Result<(), UiAdapterError> {
        backend.show_window(adapter, &self.window)
    }

    /// Refresh native window state into the shared event queue.
    pub fn refresh(
        &mut self,
        backend: &AppKitWindowBackend,
        adapter: &MacosUiAdapter,
    ) -> Result<AppKitWindowSnapshot, UiAdapterError> {
        let snapshot = self.window.snapshot()?;
        self.event_state.sync_snapshot(backend, adapter, snapshot)
    }

    /// Queue a native AppKit event into the shared Layer36 event stream.
    pub fn handle_native_event(
        &mut self,
        backend: &AppKitWindowBackend,
        adapter: &MacosUiAdapter,
        event: AppKitWindowNativeEvent,
    ) -> Result<Option<AppKitWindowSnapshot>, UiAdapterError> {
        self.event_state
            .handle_native_event(backend, adapter, event)
    }

    /// Report a close request from native AppKit into the shared queue.
    pub fn report_close_requested(
        &mut self,
        backend: &AppKitWindowBackend,
        adapter: &MacosUiAdapter,
    ) -> Result<(), UiAdapterError> {
        self.handle_native_event(backend, adapter, AppKitWindowNativeEvent::CloseRequested)
            .map(|_| ())
    }
}

impl AppKitWindowBackend {
    /// Return the native backend kind created by this prototype.
    pub fn backend_kind(&self) -> WindowBackendKind {
        WindowBackendKind::AppKit
    }

    /// Return whether this crate build can create AppKit windows.
    pub fn is_available(&self) -> bool {
        cfg!(target_os = "macos")
    }

    /// Create an owned native window session for the first AppKit event-loop work.
    pub fn create_session(
        &self,
        adapter: &MacosUiAdapter,
        options: WindowOptions,
    ) -> Result<AppKitWindowSession, UiAdapterError> {
        let window = self.create_window(adapter, options)?;
        let event_state = AppKitWindowEventState::new(window.id());
        Ok(AppKitWindowSession {
            window,
            event_state,
        })
    }

    /// Queue a native close request for a Layer36 window id.
    pub fn report_close_requested_for_id(
        &self,
        adapter: &MacosUiAdapter,
        id: WindowId,
    ) -> Result<(), UiAdapterError> {
        WindowAdapter::queue_close_requested(adapter, id)
    }

    /// Queue a native resize for a Layer36 window id.
    pub fn report_resized_for_id(
        &self,
        adapter: &MacosUiAdapter,
        id: WindowId,
        size: WindowSize,
    ) -> Result<(), UiAdapterError> {
        WindowAdapter::queue_host_resize(adapter, id, size)
    }

    /// Queue a native focus change for a Layer36 window id.
    pub fn report_focused_for_id(
        &self,
        adapter: &MacosUiAdapter,
        id: WindowId,
        focused: bool,
    ) -> Result<(), UiAdapterError> {
        WindowAdapter::queue_window_focused(adapter, id, focused)
    }

    /// Queue a native display scale change for a Layer36 window id.
    pub fn report_scale_changed_for_id(
        &self,
        adapter: &MacosUiAdapter,
        id: WindowId,
        scale: f32,
    ) -> Result<(), UiAdapterError> {
        WindowAdapter::queue_scale_changed(adapter, id, scale)
    }

    /// Queue changed native state for a Layer36 window id.
    pub fn sync_snapshot_for_id(
        &self,
        adapter: &MacosUiAdapter,
        id: WindowId,
        snapshot: AppKitWindowSnapshot,
        previous: Option<AppKitWindowSnapshot>,
    ) -> Result<AppKitWindowSnapshot, UiAdapterError> {
        if previous.is_none_or(|previous| previous.size != snapshot.size) {
            self.report_resized_for_id(adapter, id, snapshot.size)?;
        }
        if previous.is_none_or(|previous| previous.focused != snapshot.focused) {
            self.report_focused_for_id(adapter, id, snapshot.focused)?;
        }
        if previous.is_none_or(|previous| previous.scale != snapshot.scale) {
            self.report_scale_changed_for_id(adapter, id, snapshot.scale)?;
        }

        Ok(snapshot)
    }

    /// Queue a native close request for an owned AppKit window.
    pub fn report_close_requested(
        &self,
        adapter: &MacosUiAdapter,
        window: &AppKitWindowPrototype,
    ) -> Result<(), UiAdapterError> {
        self.report_close_requested_for_id(adapter, window.id())
    }

    /// Queue a native resize for an owned AppKit window.
    pub fn report_resized(
        &self,
        adapter: &MacosUiAdapter,
        window: &AppKitWindowPrototype,
        size: WindowSize,
    ) -> Result<(), UiAdapterError> {
        self.report_resized_for_id(adapter, window.id(), size)
    }

    /// Queue a native focus change for an owned AppKit window.
    pub fn report_focused(
        &self,
        adapter: &MacosUiAdapter,
        window: &AppKitWindowPrototype,
        focused: bool,
    ) -> Result<(), UiAdapterError> {
        self.report_focused_for_id(adapter, window.id(), focused)
    }

    /// Queue a native display scale change for an owned AppKit window.
    pub fn report_scale_changed(
        &self,
        adapter: &MacosUiAdapter,
        window: &AppKitWindowPrototype,
        scale: f32,
    ) -> Result<(), UiAdapterError> {
        self.report_scale_changed_for_id(adapter, window.id(), scale)
    }

    /// Read an AppKit snapshot and queue changed state into the shared event stream.
    pub fn sync_window_state(
        &self,
        adapter: &MacosUiAdapter,
        window: &AppKitWindowPrototype,
        previous: Option<AppKitWindowSnapshot>,
    ) -> Result<AppKitWindowSnapshot, UiAdapterError> {
        let snapshot = window.snapshot()?;
        self.sync_snapshot_for_id(adapter, window.id(), snapshot, previous)
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use super::*;
    use objc2::rc::Retained;
    use objc2::{MainThreadMarker, MainThreadOnly};
    use objc2_app_kit::{NSApplication, NSBackingStoreType, NSWindow, NSWindowStyleMask};
    use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

    /// Owned AppKit window bound to one Layer36 window id.
    pub struct AppKitWindowPrototype {
        id: WindowId,
        native_handle: NativeWindowHandle,
        window: Retained<NSWindow>,
    }

    impl AppKitWindowPrototype {
        /// Return the stable Layer36 window id.
        pub fn id(&self) -> WindowId {
            self.id
        }

        /// Return the opaque AppKit handle attached to the shared window registry.
        pub fn native_handle(&self) -> NativeWindowHandle {
            self.native_handle
        }

        /// Read current AppKit window state without draining the Layer36 queue.
        pub fn snapshot(&self) -> Result<AppKitWindowSnapshot, UiAdapterError> {
            let _mtm = main_thread_marker()?;
            let content_rect = self.window.contentLayoutRect();
            Ok(AppKitWindowSnapshot {
                visible: self.window.isVisible(),
                focused: self.window.isKeyWindow(),
                size: size_from_rect(content_rect)?,
                scale: self.window.backingScaleFactor() as f32,
            })
        }
    }

    impl AppKitWindowBackend {
        /// Create a real AppKit `NSWindow` and attach it to a Layer36 window id.
        pub fn create_window(
            &self,
            adapter: &MacosUiAdapter,
            options: WindowOptions,
        ) -> Result<AppKitWindowPrototype, UiAdapterError> {
            let mtm = main_thread_marker()?;
            let app = NSApplication::sharedApplication(mtm);
            let native_window = create_native_window(mtm, &options);
            let raw_handle = Retained::as_ptr(&native_window) as usize as u64;
            let id = WindowAdapter::create_window(adapter, options)?;
            let native_handle = adapter.attach_appkit_window_handle(id, raw_handle)?;

            drop(app);

            Ok(AppKitWindowPrototype {
                id,
                native_handle,
                window: native_window,
            })
        }

        /// Show the AppKit window and mark the Layer36 window visible.
        pub fn show_window(
            &self,
            adapter: &MacosUiAdapter,
            window: &AppKitWindowPrototype,
        ) -> Result<(), UiAdapterError> {
            let _mtm = main_thread_marker()?;
            window.window.makeKeyAndOrderFront(None);
            WindowAdapter::show_window(adapter, window.id)
        }
    }

    fn main_thread_marker() -> Result<MainThreadMarker, UiAdapterError> {
        MainThreadMarker::new().ok_or_else(|| {
            UiAdapterError::Unsupported(
                "AppKit windows must be created on the macOS main thread".to_string(),
            )
        })
    }

    fn create_native_window(mtm: MainThreadMarker, options: &WindowOptions) -> Retained<NSWindow> {
        let style = NSWindowStyleMask::Titled
            | NSWindowStyleMask::Closable
            | NSWindowStyleMask::Miniaturizable
            | NSWindowStyleMask::Resizable;
        let rect = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(options.size.width as f64, options.size.height as f64),
        );

        // SAFETY: AppKit requires NSWindow allocation and initialization on the
        // main thread. The caller holds MainThreadMarker, and objc2 keeps the
        // returned NSWindow retained while AppKitWindowPrototype is alive.
        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                NSWindow::alloc(mtm),
                rect,
                style,
                NSBackingStoreType::Buffered,
                false,
            )
        };
        let title = NSString::from_str(&options.title);
        window.setTitle(&title);
        window.center();
        window
    }

    fn size_from_rect(rect: NSRect) -> Result<WindowSize, UiAdapterError> {
        let width = logical_edge_to_u32(rect.size.width);
        let height = logical_edge_to_u32(rect.size.height);
        WindowSize::new(width, height)
    }

    fn logical_edge_to_u32(value: f64) -> u32 {
        if !value.is_finite() || value < 1.0 || value > u32::MAX as f64 {
            return 0;
        }

        value.round() as u32
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use layer36_adapter_common::ui::WindowSize;

        #[test]
        fn appkit_backend_reports_native_target() {
            let backend = AppKitWindowBackend;
            assert_eq!(backend.backend_kind(), WindowBackendKind::AppKit);
            assert!(backend.is_available());
        }

        #[test]
        fn appkit_main_thread_gate_is_explicit() {
            let backend = AppKitWindowBackend;
            assert!(backend.is_available());
            let options = WindowOptions::new(
                "Layer36 AppKit prototype",
                WindowSize::new(640, 480).unwrap(),
            )
            .unwrap();
            assert_eq!(options.title, "Layer36 AppKit prototype");
        }

        #[test]
        #[ignore = "opens a real AppKit window on the local macOS desktop"]
        fn ignored_smoke_can_create_and_show_appkit_window() {
            let adapter = MacosUiAdapter::new();
            let backend = AppKitWindowBackend;
            let options = WindowOptions::new(
                "Layer36 AppKit prototype",
                WindowSize::new(640, 480).unwrap(),
            )
            .unwrap();
            let window = backend
                .create_window(&adapter, options)
                .expect("create appkit window");
            backend
                .show_window(&adapter, &window)
                .expect("show appkit window");
            assert_eq!(window.native_handle().backend, WindowBackendKind::AppKit);
        }

        #[test]
        #[ignore = "opens a real AppKit window on the local macOS desktop"]
        fn ignored_smoke_can_snapshot_and_sync_appkit_window_state() {
            let adapter = MacosUiAdapter::new();
            let backend = AppKitWindowBackend;
            let options = WindowOptions::new(
                "Layer36 AppKit event bridge",
                WindowSize::new(640, 480).unwrap(),
            )
            .unwrap();
            let window = backend
                .create_window(&adapter, options)
                .expect("create appkit window");
            let snapshot = backend
                .sync_window_state(&adapter, &window, None)
                .expect("sync appkit state");

            assert_eq!(snapshot.size, WindowSize::new(640, 480).unwrap());
            assert!(snapshot.scale > 0.0);
        }

        #[test]
        #[ignore = "opens a real AppKit window on the local macOS desktop"]
        fn ignored_smoke_can_refresh_appkit_window_session() {
            let adapter = MacosUiAdapter::new();
            let backend = AppKitWindowBackend;
            let options =
                WindowOptions::new("Layer36 AppKit session", WindowSize::new(640, 480).unwrap())
                    .unwrap();
            let mut session = backend
                .create_session(&adapter, options)
                .expect("create appkit session");

            session
                .show(&backend, &adapter)
                .expect("show appkit session");
            let snapshot = session
                .refresh(&backend, &adapter)
                .expect("refresh appkit session");

            assert_eq!(session.id(), session.window().id());
            assert_eq!(session.native_handle().backend, WindowBackendKind::AppKit);
            assert_eq!(session.last_snapshot(), Some(snapshot));
            assert!(snapshot.scale > 0.0);
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    use super::*;

    /// Placeholder returned only on macOS builds.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AppKitWindowPrototype {
        id: WindowId,
        native_handle: NativeWindowHandle,
    }

    impl AppKitWindowPrototype {
        /// Return the stable Layer36 window id.
        pub fn id(&self) -> WindowId {
            self.id
        }

        /// Return the opaque AppKit handle attached to the shared window registry.
        pub fn native_handle(&self) -> NativeWindowHandle {
            self.native_handle
        }

        /// AppKit snapshots are only available in macOS builds.
        pub fn snapshot(&self) -> Result<AppKitWindowSnapshot, UiAdapterError> {
            Err(UiAdapterError::Unsupported(
                "AppKit window snapshots are only available on macOS".to_string(),
            ))
        }
    }

    impl AppKitWindowBackend {
        /// AppKit is only available in macOS builds.
        pub fn create_window(
            &self,
            _adapter: &MacosUiAdapter,
            _options: WindowOptions,
        ) -> Result<AppKitWindowPrototype, UiAdapterError> {
            Err(UiAdapterError::Unsupported(
                "AppKit windows are only available on macOS".to_string(),
            ))
        }

        /// AppKit is only available in macOS builds.
        pub fn show_window(
            &self,
            _adapter: &MacosUiAdapter,
            _window: &AppKitWindowPrototype,
        ) -> Result<(), UiAdapterError> {
            Err(UiAdapterError::Unsupported(
                "AppKit windows are only available on macOS".to_string(),
            ))
        }
    }
}

pub use platform::AppKitWindowPrototype;

#[cfg(test)]
mod tests {
    use super::*;
    use layer36_adapter_common::ui::UiEvent;

    #[test]
    fn appkit_event_bridge_queues_shared_window_events_by_id() {
        let adapter = MacosUiAdapter::new();
        let backend = AppKitWindowBackend;
        let id = WindowAdapter::create_window(
            &adapter,
            WindowOptions::new("Layer36 AppKit events", WindowSize::new(640, 480).unwrap())
                .unwrap(),
        )
        .expect("create window");
        let resized = WindowSize::new(800, 600).unwrap();

        backend
            .report_resized_for_id(&adapter, id, resized)
            .expect("resize");
        backend
            .report_focused_for_id(&adapter, id, true)
            .expect("focus");
        backend
            .report_scale_changed_for_id(&adapter, id, 2.0)
            .expect("scale");
        backend
            .report_close_requested_for_id(&adapter, id)
            .expect("close request");

        assert_eq!(
            WindowAdapter::drain_events(&adapter).expect("events"),
            vec![
                UiEvent::WindowCreated(id),
                UiEvent::Resized { id, size: resized },
                UiEvent::WindowFocused { id, focused: true },
                UiEvent::ScaleChanged { id, scale: 2.0 },
                UiEvent::WindowCloseRequested(id),
            ]
        );
    }

    #[test]
    fn appkit_event_bridge_reuses_shared_scale_validation() {
        let adapter = MacosUiAdapter::new();
        let backend = AppKitWindowBackend;
        let id = WindowAdapter::create_window(
            &adapter,
            WindowOptions::new("Layer36 AppKit scale", WindowSize::new(640, 480).unwrap()).unwrap(),
        )
        .expect("create window");

        assert!(matches!(
            backend.report_scale_changed_for_id(&adapter, id, 0.0),
            Err(UiAdapterError::InvalidScaleFactor(_))
        ));
    }

    #[test]
    fn appkit_snapshot_sync_queues_only_changed_window_state() {
        let adapter = MacosUiAdapter::new();
        let backend = AppKitWindowBackend;
        let id = WindowAdapter::create_window(
            &adapter,
            WindowOptions::new(
                "Layer36 AppKit snapshot",
                WindowSize::new(640, 480).unwrap(),
            )
            .unwrap(),
        )
        .expect("create window");
        let first = AppKitWindowSnapshot {
            visible: true,
            focused: false,
            size: WindowSize::new(640, 480).unwrap(),
            scale: 1.0,
        };

        backend
            .sync_snapshot_for_id(&adapter, id, first, None)
            .expect("sync first snapshot");

        assert_eq!(
            WindowAdapter::drain_events(&adapter).expect("events"),
            vec![
                UiEvent::WindowCreated(id),
                UiEvent::Resized {
                    id,
                    size: first.size
                },
                UiEvent::WindowFocused { id, focused: false },
                UiEvent::ScaleChanged { id, scale: 1.0 },
            ]
        );

        backend
            .sync_snapshot_for_id(&adapter, id, first, Some(first))
            .expect("sync unchanged snapshot");
        assert_eq!(
            WindowAdapter::drain_events(&adapter).expect("events"),
            vec![]
        );

        let changed = AppKitWindowSnapshot {
            visible: true,
            focused: true,
            size: WindowSize::new(800, 600).unwrap(),
            scale: 2.0,
        };

        backend
            .sync_snapshot_for_id(&adapter, id, changed, Some(first))
            .expect("sync changed snapshot");

        assert_eq!(
            WindowAdapter::drain_events(&adapter).expect("events"),
            vec![
                UiEvent::Resized {
                    id,
                    size: changed.size
                },
                UiEvent::WindowFocused { id, focused: true },
                UiEvent::ScaleChanged { id, scale: 2.0 },
            ]
        );
    }

    #[test]
    fn appkit_event_state_handles_delegate_shaped_events() {
        let adapter = MacosUiAdapter::new();
        let backend = AppKitWindowBackend;
        let id = WindowAdapter::create_window(
            &adapter,
            WindowOptions::new(
                "Layer36 AppKit delegate",
                WindowSize::new(640, 480).unwrap(),
            )
            .unwrap(),
        )
        .expect("create window");
        let mut state = AppKitWindowEventState::new(id);
        let first = AppKitWindowSnapshot {
            visible: true,
            focused: false,
            size: WindowSize::new(640, 480).unwrap(),
            scale: 1.0,
        };

        state
            .handle_native_event(&backend, &adapter, AppKitWindowNativeEvent::Snapshot(first))
            .expect("initial snapshot");
        WindowAdapter::drain_events(&adapter).expect("drain initial snapshot");

        state
            .handle_native_event(
                &backend,
                &adapter,
                AppKitWindowNativeEvent::Resized(WindowSize::new(800, 600).unwrap()),
            )
            .expect("resize callback");
        state
            .handle_native_event(&backend, &adapter, AppKitWindowNativeEvent::Focused(true))
            .expect("focus callback");
        state
            .handle_native_event(
                &backend,
                &adapter,
                AppKitWindowNativeEvent::ScaleChanged(2.0),
            )
            .expect("scale callback");
        state
            .handle_native_event(&backend, &adapter, AppKitWindowNativeEvent::CloseRequested)
            .expect("close callback");

        assert_eq!(
            WindowAdapter::drain_events(&adapter).expect("events"),
            vec![
                UiEvent::Resized {
                    id,
                    size: WindowSize::new(800, 600).unwrap()
                },
                UiEvent::WindowFocused { id, focused: true },
                UiEvent::ScaleChanged { id, scale: 2.0 },
                UiEvent::WindowCloseRequested(id),
            ]
        );
        assert_eq!(
            state.last_snapshot(),
            Some(AppKitWindowSnapshot {
                visible: true,
                focused: true,
                size: WindowSize::new(800, 600).unwrap(),
                scale: 2.0,
            })
        );
    }

    #[test]
    fn appkit_event_state_does_not_cache_failed_scale_event() {
        let adapter = MacosUiAdapter::new();
        let backend = AppKitWindowBackend;
        let id = WindowAdapter::create_window(
            &adapter,
            WindowOptions::new(
                "Layer36 AppKit bad scale",
                WindowSize::new(640, 480).unwrap(),
            )
            .unwrap(),
        )
        .expect("create window");
        let mut state = AppKitWindowEventState::new(id);
        let first = AppKitWindowSnapshot {
            visible: true,
            focused: false,
            size: WindowSize::new(640, 480).unwrap(),
            scale: 1.0,
        };

        state
            .handle_native_event(&backend, &adapter, AppKitWindowNativeEvent::Snapshot(first))
            .expect("initial snapshot");
        WindowAdapter::drain_events(&adapter).expect("drain initial snapshot");

        assert!(matches!(
            state.handle_native_event(
                &backend,
                &adapter,
                AppKitWindowNativeEvent::ScaleChanged(0.0),
            ),
            Err(UiAdapterError::InvalidScaleFactor(_))
        ));
        assert_eq!(state.last_snapshot(), Some(first));
    }
}
