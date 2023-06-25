use bevy_egui::{egui, EguiContexts};

pub(super) fn developer_menu(mut egui_contexts: EguiContexts) {
    egui::Window::new("Developer menu").show(egui_contexts.ctx_mut(), |ui| {
        // Add new developer tools in this window

        ui.label("Camera:");
        // ui.checkbox(
        //     &mut main_camera_config.edge_scrolling_enabled,
        //     "Edge scrolling",
        // );

        ui.label("Build:");
        if ui.button("Example building").clicked() {}
    });
}
