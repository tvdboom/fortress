use bevy::prelude::NonSend;
use bevy::winit::WinitWindows;
use winit::window::Icon;

#[cfg(target_os = "windows")]
pub fn set_window_icon(windows: NonSend<WinitWindows>) {
    let image = image::open("assets/icons/bug.png").unwrap().into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    let icon = Icon::from_rgba(rgba, width, height).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
