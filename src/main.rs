#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self};
use std::fs;

fn main() {
    //let path = "/home/gerrit/Downloads/eye-texture.jpg"
    //    .split("/").last().unwrap()
    //    .split(".").last().unwrap();
    //println!("{}", path);
    let _ = egui_init();   
}

fn egui_init() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: None,
        ..Default::default()
    };

    /*let paths = {
        let mut files = vec![];
        let paths = fs::read_dir("/home/gerrit/Downloads/")
            .unwrap().filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        /*for i in 0..paths.len() {
            let a = paths[i].clone();
            let is_supported = match a.clone().as_str().split("/").last().unwrap().split(".").last().unwrap() {
                ".png" | ".jpg" | ".jpeg" | ".tif" | ".tiff" | ".bmp" | ".webm" => true,
                _ => false
            };
            if is_supported {files.push(a.as_str())}
        }*/

        files.push(paths[0].as_str());
        //files.push("file:///home/gerrit/Downloads/eye-texture.jpg");
        files
    };*/
    /*let paths = vec![
        "file:///home/gerrit/Downloads/eye-texture.jpg",
        "file:///home/gerrit/wallpapers/dark_sun_wallpaper.jpg",
    ];*/
    let prelim_paths = fs::read_dir("/home/gerrit/Downloads/")
        .unwrap().filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let paths = vec![prelim_paths[0]];

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
    fn new(path: Vec<String>) -> MyApp<'a> {
        let mut image_sets = vec![];
        for i in 0..path.len() {
            image_sets.push(egui::ImageSource::Uri(std::borrow::Cow::Borrowed(path[i].as_str())))
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
