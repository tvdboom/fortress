use crate::constants::MESSAGE_DURATION;
use bevy::prelude::*;
use bevy_egui::egui::WidgetText;
use bevy_egui::EguiContexts;
use egui_notify::{Anchor, Toast, Toasts};
use std::time::Duration;

#[derive(Resource)]
pub struct Messages(pub Toasts);

impl Messages {
    pub fn info(&mut self, message: impl Into<WidgetText>) -> &mut Toast {
        self.0
            .info(message)
            .duration(Some(Duration::from_secs(MESSAGE_DURATION)))
    }

    pub fn warning(&mut self, message: impl Into<WidgetText>) -> &mut Toast {
        self.0
            .warning(message)
            .duration(Some(Duration::from_secs(MESSAGE_DURATION)))
    }

    pub fn error(&mut self, message: impl Into<WidgetText>) -> &mut Toast {
        self.0
            .error(message)
            .duration(Some(Duration::from_secs(MESSAGE_DURATION)))
            .closable(true)
    }
}

pub struct MessagesPlugin {
    builder: Option<fn() -> Toasts>,
}

impl Default for MessagesPlugin {
    fn default() -> Self {
        Self {
            builder: Some(|| {
                Toasts::default()
                    .with_margin([0., 30.].into())
                    .with_anchor(Anchor::TopLeft)
            }),
        }
    }
}

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Messages(self.builder.map(|f| f()).unwrap_or_default()))
            .add_systems(Update, update_messages);
    }
}

fn update_messages(contexts: EguiContexts, mut messages: ResMut<Messages>) {
    messages.0.show(contexts.ctx());
}
