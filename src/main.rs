#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

//use std::fs;

use eframe::egui::{self};

fn main() {
//    let paths = fs::read_dir("/home/gerrit/Downloads/").unwrap();
//    for path in paths {println!("Name: {}", path.unwrap().path().display())}
    let _ = egui_init();   
}

fn egui_init() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: None,
        ..Default::default()
    };
    let mut framebox = Box::new(MyApp::new("file:///home/gerrit/Downloads/eye-texture.jpg"));
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            framebox
        }),
    )
}

struct MyApp<'a> {
    image: egui::ImageSource<'a>
}

impl<'a> MyApp<'a> {
    fn new(path: &str) -> MyApp {
        MyApp {
            image: egui::ImageSource::Uri(std::borrow::Cow::Borrowed(path))
        }
    }
}

impl<'a> eframe::App for MyApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                ui.image(self.image.clone());
            });
        });
    }
}
