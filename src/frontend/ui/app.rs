use log::{error, info};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use eframe::egui;

/*
enum MenuState {
    MainMenu,
}
*/

pub struct FrontendApp {
    tcp_stream: Option<TcpStream>,
    // Example stuff:
    label: String,
    note_list: Vec<String>,
    //value: f32,
}

impl Default for FrontendApp {
    fn default() -> Self {
        Self {
            //tcp_stream: ,
            tcp_stream: None,
            // Example stuff:
            // label: "Hello World!".to_owned(),
            //value: 2.7,
            label: String::new(),
            note_list: vec![],
        }
    }
}

impl FrontendApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut ret: FrontendApp = Default::default();
       // ret.tcp_stream = Some(TcpStream::connect("0.0.0.0:7878").expect("Daemon not running"));
        ret
    }
}

impl eframe::App for FrontendApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("GAV User Interface");
            ui.label("Enable Window Defender Security Settings");
            if ui.button("Enable").clicked() {
                println!("Button clicked!");
            }

            /*
            ui.horizontal(|ui| {
                ui.label("Search: ");
                ui.add(
                    egui::TextEdit::singleline(&mut self.label)
                        .hint_text("Enter your note title here..."),
                );
            });

            ui.separator();
            if ui.add(egui::Button::new("Search")).clicked() && self.tcp_stream.is_some() {
                // do the tcp call

                let mut borrowed_tcp_stream = self.tcp_stream.take().unwrap();

                borrowed_tcp_stream.write_all(self.label.as_bytes()).expect("tcp error");

                let peer = borrowed_tcp_stream
                    .peer_addr()
                    .unwrap_or_else(|_| "unknown".parse().unwrap());

                let mut buffer = [0; 512];
                match borrowed_tcp_stream.read(&mut buffer) {
                    Ok(n) => {
                        ui.label(String::from_utf8_lossy(&buffer[..n]).to_string() + "..");
                        ui.label("..");
                    },
                    Err(e) => error!("Failed to read from {}: {}", peer, e),
                }
            }
            */

            

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                //powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

/*
fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
*/
