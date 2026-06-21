use std::error::Error;

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn Error>> {
    use layer36_adapter_common::ui::{WindowBackendKind, WindowOptions, WindowSize};
    use layer36_policy::SessionPolicy;
    use layer36_runtime::{
        phase3_ui::{Phase3HostUiMode, Phase3UiRuntime},
        uapi::UapiGuard,
    };

    let guard = UapiGuard::new(SessionPolicy::default());
    let runtime =
        Phase3UiRuntime::try_with_host_adapter_mode(guard, Phase3HostUiMode::NativePrototype)?;
    let dispatcher = runtime.dispatcher();
    let info = dispatcher.adapter_info();
    let window = dispatcher.create_window(WindowOptions::new(
        "Layer36 runtime AppKit smoke",
        WindowSize::new(640, 480)?,
    )?)?;

    dispatcher.show_window(window)?;
    let tick = dispatcher
        .pump_event_loop_once(window)?
        .ok_or("AppKit prototype returned no native tick")?;
    let record = dispatcher
        .window(window)?
        .ok_or("AppKit prototype window record missing")?;

    assert_eq!(runtime.host_mode(), Phase3HostUiMode::NativePrototype);
    assert_eq!(info.backend, "macos-appkit-prototype");
    assert_eq!(info.window_backend, WindowBackendKind::AppKit);
    assert!(info.native_windows);
    assert!(info.native_event_loop);
    assert_eq!(tick.window, window);
    assert!(tick.snapshot_refreshed);
    assert_eq!(record.title, "Layer36 runtime AppKit smoke");
    assert!(record.visible);

    dispatcher.close_window(window)?;
    let events = dispatcher.drain_events()?;

    println!("Layer36 Phase 3 AppKit runtime smoke passed");
    println!("- window: {}", window.get());
    println!("- backend: {}", info.backend);
    println!("- callbacks handled: {}", tick.callbacks_handled);
    println!("- snapshot refreshed: {}", tick.snapshot_refreshed);
    println!("- redraw requested: {}", tick.redraw_requested);
    println!("- events observed: {}", events.len());

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Layer36 Phase 3 AppKit runtime smoke skipped: host is not macOS");
    Ok(())
}
