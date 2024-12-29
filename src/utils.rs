use bevy_egui::egui;
use bevy_egui::egui::{epaint, Response, TextureId, Ui, WidgetText};

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