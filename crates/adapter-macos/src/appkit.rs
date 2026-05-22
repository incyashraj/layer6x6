//! Opt-in AppKit window prototype for the macOS adapter.
//!
//! The default macOS adapter still uses the headless draft backend. This module
//! is the first native path behind the checked handle handoff: it creates an
//! AppKit `NSWindow`, keeps that object alive, and binds its raw pointer to a
//! stable Layer36 `WindowId`.

use layer36_adapter_common::ui::{
    NativeWindowHandle, UiAdapterError, WindowAdapter, WindowBackendKind, WindowId, WindowOptions,
};

use crate::MacosUiAdapter;

/// Small native AppKit backend used by the first Phase 3 macOS prototype.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AppKitWindowBackend;

impl AppKitWindowBackend {
    /// Return the native backend kind created by this prototype.
    pub fn backend_kind(&self) -> WindowBackendKind {
        WindowBackendKind::AppKit
    }

    /// Return whether this crate build can create AppKit windows.
    pub fn is_available(&self) -> bool {
        cfg!(target_os = "macos")
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
