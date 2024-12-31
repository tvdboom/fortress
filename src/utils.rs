use std::time::Duration;
use bevy_egui::egui;
use bevy_egui::egui::{epaint, Response, TextureId, Ui, WidgetText};

pub fn scale_duration(duration: Duration, scale: f32) -> Duration {
    let sec = (duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9) * scale;
    Duration::new(sec.trunc() as u64, (sec.fract() * 1e9) as u32)
}

/// Add to get the string name of Enums
pub trait EnumDisplay {
    fn name(&self) -> String;
}

impl<T> EnumDisplay for T
where
    T: std::fmt::Debug,
{
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

/// Custom syntax sugar for repetitive UI elements
pub trait CustomUi {
    fn add_button(&mut self, text: impl Into<WidgetText>) -> Response;
    fn add_image(&mut self, id: impl Into<TextureId>, size: impl Into<epaint::Vec2>) -> Response;
}

impl CustomUi for Ui {
    fn add_button(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add_sized([120., 40.], egui::Button::new(text))
    }

    fn add_image(&mut self, id: impl Into<TextureId>, size: impl Into<epaint::Vec2>) -> Response {
        self.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
            id, size,
        )))
    }
}
