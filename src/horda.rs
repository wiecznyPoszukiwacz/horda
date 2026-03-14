pub struct HordaApp {
    pub(crate) hotkey_receiver: std::sync::mpsc::Receiver<String>,
    pub(crate) visible: bool,
    pub(crate) input: String,
}

impl eframe::App for HordaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(komunikat) = self.hotkey_receiver.try_recv() {
            println!("{}", komunikat);
            self.visible = !self.visible;
        };

        if self.visible {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            egui::CentralPanel::default().show(ctx, |ui| ui.text_edit_singleline(&mut self.input));
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }
    }
}
