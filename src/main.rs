#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self};
use std::{fs, path::PathBuf};
use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args = Args::parse();
    match args.input {
        Some(x) => {
            if x.is_file() {
                println!("SINGLE IMAGE VIEWING IS NOT CURRENTLY PERMITTED. PLEASE TELL ME TO FIX");
                _ = egui_init("/home/gerrit/Desktop/loading screens/".to_owned(), args.random);
            }
            else {
                _ = egui_init(x.to_string_lossy().into_owned(), args.random);
            }
        },
        None => {
            _ = egui_init(".".to_owned(), args.random);
        }
    };
}

fn egui_init(base: String, random: bool) -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: None,
        ..Default::default()
    };

    let prelim_paths = fs::read_dir(base)
        .unwrap().filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .filter(|x| {
            match x.split("/").last().unwrap().split(".").last().unwrap() {
                "jpg" | "png" | "jpeg" => true,
                _ => false
            }})
        .collect::<Vec<_>>();
    let mut paths = prelim_paths;
    if random { paths.shuffle(&mut thread_rng()) }

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

#[derive(Parser)]
struct Args {
    ///Where to view from
    input: Option<PathBuf>,

    ///do you want your images randomized
    #[arg(short, long)]
    random: bool
}

struct MyApp<'a> {
    images: Vec<egui::ImageSource<'a>>,
    index: usize
}

impl<'a> MyApp<'a> {
    fn new(paths: Vec<String>) -> MyApp<'a> {
        let mut image_sets = vec![];
        for path in paths {
            image_sets.push(egui::ImageSource::Uri(std::borrow::Cow::Owned("file://".to_owned() + &path.clone())))
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
    fn previous_image(&mut self) {
        if self.index > 0 { self.index -= 1}
        else {self.index = self.images.len() - 1}
    }
}

impl<'a> eframe::App for MyApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.next_image();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::H)) {
            self.previous_image();
        }
        if ctx.input (|i| i.key_pressed(egui::Key::Q) || i.key_pressed(egui::Key::Escape)) {
            _frame.close()
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                //yes this is definitely the best way to handle dirs with no images
                //look into ui.set_visable
                //might need closure/ui.group stuff
                if self.images.len() > 0 {
                    ui.group(|ui|{
                        ui.set_visible(true);
                        ui.image(self.images[self.index].clone());
                    });
                    //ui.image(self.images[self.index].clone());
                } 
            });
        });
    }
}
