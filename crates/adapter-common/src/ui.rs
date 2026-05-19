//! Shared Phase 3 UI adapter draft types.
//!
//! This module is intentionally host-neutral. It does not create real native
//! windows yet. It gives the Linux, macOS, and Windows adapter work a shared
//! shape for ids, size validation, lifecycle state, and early event routing.

use std::collections::BTreeMap;

use thiserror::Error;

/// Stable host-side id for a window in one runtime session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(u64);

impl WindowId {
    /// Return the raw id value used by WIT and logs.
    pub fn get(self) -> u64 {
        self.0
    }
}

/// Logical size in device-independent pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    /// Create a validated logical window size.
    pub fn new(width: u32, height: u32) -> Result<Self, UiAdapterError> {
        if width == 0 || height == 0 {
            return Err(UiAdapterError::InvalidSize { width, height });
        }
        if width > MAX_WINDOW_EDGE || height > MAX_WINDOW_EDGE {
            return Err(UiAdapterError::InvalidSize { width, height });
        }

        Ok(Self { width, height })
    }
}

/// Host window state requested by an app or reported by an adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Host color preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    Unknown,
}

/// Options used when creating a window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowOptions {
    pub title: String,
    pub size: WindowSize,
    pub state: WindowState,
}

impl WindowOptions {
    /// Create a validated window request.
    pub fn new(title: impl Into<String>, size: WindowSize) -> Result<Self, UiAdapterError> {
        let title = title.into();
        validate_title(&title)?;

        Ok(Self {
            title,
            size,
            state: WindowState::Normal,
        })
    }
}

/// State record tracked by the draft window registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowRecord {
    pub id: WindowId,
    pub title: String,
    pub size: WindowSize,
    pub state: WindowState,
    pub visible: bool,
    pub closed: bool,
}

/// Small host-neutral event shape for the first Phase 3 prototype.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent {
    WindowCreated(WindowId),
    WindowShown(WindowId),
    WindowClosed(WindowId),
    RedrawRequested(WindowId),
    Resized { id: WindowId, size: WindowSize },
    TitleChanged { id: WindowId, title: String },
}

/// Draft in-memory window registry used until OS-backed adapters land.
#[derive(Debug, Default)]
pub struct DraftWindowRegistry {
    next_id: u64,
    windows: BTreeMap<WindowId, WindowRecord>,
    events: Vec<UiEvent>,
}

impl DraftWindowRegistry {
    /// Create a window record and return its id.
    pub fn create_window(&mut self, options: WindowOptions) -> WindowId {
        self.next_id += 1;
        let id = WindowId(self.next_id);
        let record = WindowRecord {
            id,
            title: options.title,
            size: options.size,
            state: options.state,
            visible: false,
            closed: false,
        };
        self.windows.insert(id, record);
        self.events.push(UiEvent::WindowCreated(id));
        id
    }

    /// Mark a created window as visible.
    pub fn show_window(&mut self, id: WindowId) -> Result<(), UiAdapterError> {
        let window = self.open_window_mut(id)?;
        window.visible = true;
        self.events.push(UiEvent::WindowShown(id));
        Ok(())
    }

    /// Mark a window as closed.
    pub fn close_window(&mut self, id: WindowId) -> Result<(), UiAdapterError> {
        let window = self.open_window_mut(id)?;
        window.closed = true;
        window.visible = false;
        self.events.push(UiEvent::WindowClosed(id));
        Ok(())
    }

    /// Change a window title.
    pub fn set_title(
        &mut self,
        id: WindowId,
        title: impl Into<String>,
    ) -> Result<(), UiAdapterError> {
        let title = title.into();
        validate_title(&title)?;
        let window = self.open_window_mut(id)?;
        window.title = title.clone();
        self.events.push(UiEvent::TitleChanged { id, title });
        Ok(())
    }

    /// Change a window size.
    pub fn set_size(&mut self, id: WindowId, size: WindowSize) -> Result<(), UiAdapterError> {
        let window = self.open_window_mut(id)?;
        window.size = size;
        self.events.push(UiEvent::Resized { id, size });
        Ok(())
    }

    /// Queue a redraw request for a window.
    pub fn request_redraw(&mut self, id: WindowId) -> Result<(), UiAdapterError> {
        self.open_window(id)?;
        self.events.push(UiEvent::RedrawRequested(id));
        Ok(())
    }

    /// Read one window record.
    pub fn window(&self, id: WindowId) -> Option<&WindowRecord> {
        self.windows.get(&id)
    }

    /// Drain queued draft events.
    pub fn drain_events(&mut self) -> Vec<UiEvent> {
        std::mem::take(&mut self.events)
    }

    fn open_window(&self, id: WindowId) -> Result<&WindowRecord, UiAdapterError> {
        let window = self
            .windows
            .get(&id)
            .ok_or(UiAdapterError::InvalidWindow { id: id.get() })?;
        if window.closed {
            return Err(UiAdapterError::WindowClosed { id: id.get() });
        }
        Ok(window)
    }

    fn open_window_mut(&mut self, id: WindowId) -> Result<&mut WindowRecord, UiAdapterError> {
        let window = self
            .windows
            .get_mut(&id)
            .ok_or(UiAdapterError::InvalidWindow { id: id.get() })?;
        if window.closed {
            return Err(UiAdapterError::WindowClosed { id: id.get() });
        }
        Ok(window)
    }
}

/// Errors surfaced by the Phase 3 UI adapter draft.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum UiAdapterError {
    #[error("invalid window id {id}")]
    InvalidWindow { id: u64 },
    #[error("window {id} is closed")]
    WindowClosed { id: u64 },
    #[error("invalid window size {width}x{height}")]
    InvalidSize { width: u32, height: u32 },
    #[error("window title is empty")]
    EmptyTitle,
    #[error("window title is too long")]
    TitleTooLong,
    #[error("unsupported UI feature: {0}")]
    Unsupported(String),
}

const MAX_WINDOW_EDGE: u32 = 16_384;
const MAX_TITLE_CHARS: usize = 512;

fn validate_title(title: &str) -> Result<(), UiAdapterError> {
    if title.trim().is_empty() {
        return Err(UiAdapterError::EmptyTitle);
    }
    if title.chars().count() > MAX_TITLE_CHARS {
        return Err(UiAdapterError::TitleTooLong);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_registry_allocates_stable_window_ids() {
        let mut registry = DraftWindowRegistry::default();
        let size = WindowSize::new(800, 600).expect("size");
        let first =
            registry.create_window(WindowOptions::new("Notes", size).expect("window options"));
        let second =
            registry.create_window(WindowOptions::new("Preview", size).expect("window options"));

        assert_eq!(first.get(), 1);
        assert_eq!(second.get(), 2);
        assert_eq!(registry.window(first).expect("first").title, "Notes");
        assert_eq!(registry.window(second).expect("second").title, "Preview");
    }

    #[test]
    fn draft_registry_tracks_visibility_resize_redraw_and_close() {
        let mut registry = DraftWindowRegistry::default();
        let size = WindowSize::new(800, 600).expect("size");
        let id = registry.create_window(WindowOptions::new("Notes", size).expect("options"));

        registry.show_window(id).expect("show");
        let resized = WindowSize::new(1024, 768).expect("resized");
        registry.set_size(id, resized).expect("resize");
        registry.request_redraw(id).expect("redraw");
        registry.close_window(id).expect("close");

        let window = registry.window(id).expect("window");
        assert_eq!(window.size, resized);
        assert!(!window.visible);
        assert!(window.closed);
        assert_eq!(
            registry.drain_events(),
            vec![
                UiEvent::WindowCreated(id),
                UiEvent::WindowShown(id),
                UiEvent::Resized { id, size: resized },
                UiEvent::RedrawRequested(id),
                UiEvent::WindowClosed(id),
            ]
        );
    }

    #[test]
    fn draft_registry_rejects_invalid_window_operations() {
        let mut registry = DraftWindowRegistry::default();
        let id = WindowId(42);

        assert_eq!(
            registry.show_window(id),
            Err(UiAdapterError::InvalidWindow { id: 42 })
        );

        let size = WindowSize::new(640, 480).expect("size");
        let id = registry.create_window(WindowOptions::new("Notes", size).expect("options"));
        registry.close_window(id).expect("close");

        assert_eq!(
            registry.request_redraw(id),
            Err(UiAdapterError::WindowClosed { id: id.get() })
        );
    }

    #[test]
    fn validates_window_size_and_title() {
        assert_eq!(
            WindowSize::new(0, 10),
            Err(UiAdapterError::InvalidSize {
                width: 0,
                height: 10,
            })
        );
        assert_eq!(
            WindowOptions::new(" ", WindowSize::new(100, 100).expect("size")),
            Err(UiAdapterError::EmptyTitle)
        );
    }
}
