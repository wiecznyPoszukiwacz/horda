use crate::command::Command;
use egui::Id;
use x11rb::protocol::xproto::ConnectionExt;

pub struct HordaApp {
    pub(crate) hotkey_receiver: std::sync::mpsc::Receiver<String>,
    pub(crate) visible: bool,
    pub(crate) input: String,
    pub(crate) commands: Vec<Command>,
    x11: x11rb::rust_connection::RustConnection,
    previous_window: u32,
}

impl eframe::App for HordaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(_komunikat) = self.hotkey_receiver.try_recv() {
            if !self.visible {
                self.previous_window = self
                    .x11
                    .get_input_focus()
                    .expect("Can't get current window handler")
                    .reply()
                    .expect("reply error")
                    .focus;
            }
            self.visible = !self.visible;
            if self.visible {
                self.input.clear();
                ctx.memory_mut(|m| m.request_focus(Id::new("tbox")))
            }
        };

        if self.visible {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    // nie zwijaj
                    let result = egui::TextEdit::singleline(&mut self.input)
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .font(egui::FontId::proportional(24.0))
                        .id(Id::new("tbox"))
                        .show(ui);

                    if result.response.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                        && self.find_and_execute()
                    {
                        //
                        self.input.clear();
                        self.visible = false;
                    }

                    if result.response.changed() && self.find_and_execute() {
                        self.input.clear();
                        self.visible = false;
                    }
                });
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }
    }
}

impl HordaApp {
    fn find_and_execute(&self) -> bool {
        let found = self.find_command();
        if found.len() == 1 {
            found[0].action.execute(self.previous_window, &self.x11);
        }

        found.len() == 1
    }
    fn find_command(&self) -> Vec<&Command> {
        self.commands
            .iter()
            .filter(|cmd| cmd.name.starts_with(&self.input))
            .collect()
    }

    pub(crate) fn new(
        hotkey_receiver: std::sync::mpsc::Receiver<String>,
        commands: Vec<Command>,
    ) -> Self {
        let x11 = x11rb::connect(None).expect("Can't connect to X11");

        Self {
            hotkey_receiver,
            commands,
            input: "".to_string(),
            visible: false,
            x11: x11.0,
            previous_window: 0,
        }
    }
}
