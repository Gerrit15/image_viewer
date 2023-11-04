#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self};

fn main() {
    let _ = egui_init();   
}

fn egui_init() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: None,
        ..Default::default()
    };

    let paths = vec![
        "file:///home/gerrit/Downloads/eye-texture.jpg",
        "file:///home/gerrit/wallpapers/dark_sun_wallpaper.jpg",
    ];

    let framebox = Box::new(MyApp::new(paths));
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
    images: Vec<egui::ImageSource<'a>>,
    index: usize
}

impl<'a> MyApp<'a> {
    fn new(path: Vec<&str>) -> MyApp {
        let mut image_sets = vec![];
        for i in 0..path.len() {
            image_sets.push(egui::ImageSource::Uri(std::borrow::Cow::Borrowed(path[i])))
        }
        MyApp {
            images: image_sets,
            index: 0
        }
    }
    fn next_image(&mut self) {
        if self.index < self.images.len() - 1 { self.index += 1}
        else { self.index = 0 }
    }
}

impl<'a> eframe::App for MyApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.next_image();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                ui.image(self.images[self.index].clone());
            });
        });
    }
}
