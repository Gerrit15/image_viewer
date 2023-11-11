#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, SizeHint, TextureOptions};
use std::{fs, path::PathBuf};
use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args = Args::parse_inputs();
    match args {
        Some(a) => {
            match a.1 {
                Some(b) => _ = egui_init(a.0.to_string_lossy().into_owned(), !b, b),
                None => _ = egui_init(a.0.to_string_lossy().into_owned(), false, false)
            }
        },
        None => println!("Cannot currently handle single image viewing")
    }
}

#[derive(Parser)]
struct Args {
    ///Where to view from
    input: Option<PathBuf>,

    ///do you want your images randomized
    #[arg(short, long)]
    random: bool,

    ///do you want your images sorted alphabetically
    #[arg(short, long)]
    alphabet: bool
}

impl Args {
    //Some<(...)> if the location is a directory 
    //Some<bool> for if there is a.) random || alphabetical sorting and b.) if it is alphabetical
    //(due to defaulting)
    fn parse_inputs() -> Option<(PathBuf, Option<bool>)> {
        let inputs = Args::parse();
        match inputs.input {
            Some(a) => {
                if a.is_dir() {
                    if inputs.random || inputs.alphabet {
                        if inputs.random && inputs.random {
                            println!("Cannot sort by both random and alphabetical order, defeaulting to alphabetical");
                            Some((a, Some(true)))
                        }
                        else if inputs.alphabet {
                            Some((a, Some(true)))
                        }
                        else {
                            Some((a, Some(false)))
                        }
                    }
                    else {return Some((a, None))}
                }
                else { None }
            },
            None => None
        }
    }
}


fn egui_init(base: String, random: bool, alphabet: bool) -> Result<(), eframe::Error> {
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
    if alphabet {
        paths.sort_by_key(|name| name.to_lowercase());
    }
    else if random { paths.shuffle(&mut thread_rng()) }

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
    index: usize,
    preload_magnatude: usize,
    //listen. if it works.
    first_frame: bool
}

impl<'a> MyApp<'a> {
    fn new(paths: Vec<String>) -> MyApp<'a> {
        let mut image_sets = vec![];
        for path in paths {
            image_sets.push(egui::ImageSource::Uri(std::borrow::Cow::Owned("file://".to_owned() + &path.clone())))
        }
        MyApp {
            images: image_sets,
            index: 0,
            preload_magnatude: 2,
            first_frame: true
        }
    }
    fn next_image(&mut self) {
        if self.index < (self.images.len() - 1) && self.images.len() > 1 { self.index += 1}
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
            if self.images.len()-1-self.index >= self.preload_magnatude{
                for i in 0..self.preload_magnatude {
                    let _ = self.images[self.index + i].clone().load(ctx, TextureOptions::default(), SizeHint::Size(1920, 1080));
                }
            }
            else {
                let images_remaining = self.images.len()-1-self.index;
                for i in 0..images_remaining {
                    let _ = self.images[self.index + i].clone().load(ctx, TextureOptions::default(), SizeHint::Size(1920, 1080));
                }
                for i in 0..(self.preload_magnatude - images_remaining) {
                    let _ = self.images[i].clone().load(ctx, TextureOptions::default(), SizeHint::Size(1920, 1080));
                }
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::H)) {
            self.previous_image();
            if self.index >= self.preload_magnatude {
                for i in 0..self.preload_magnatude {
                    let _ = self.images[self.index - i].clone().load(ctx, TextureOptions::default(), SizeHint::Size(1920, 1080));
                }
            }
            else {
                let images_remaining = self.preload_magnatude - self.index;
                for i in 0..self.index {
                    let _ = self.images[self.index + i].clone().load(ctx, TextureOptions::default(), SizeHint::Size(1920, 1080));
                }
                for i in 0..images_remaining {
                    let _ = self.images[self.images.len() - 1 - i].clone().load(ctx, TextureOptions::default(), SizeHint::Size(1920, 1080));
                }
            }
        }
        if ctx.input (|i| i.key_pressed(egui::Key::Q) || i.key_pressed(egui::Key::Escape)) {
            _frame.close()
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                if self.images.len() > 0 {
                    ui.image(self.images[self.index].clone());
                    if self.first_frame && self.images.len() > 1{
                        let _ = self.images[1].clone().load(ctx, TextureOptions::default(), SizeHint::Size(1920, 1080));
                        let _ = self.images[self.images.len() - 1].clone().load(ctx, TextureOptions::default(), SizeHint::Size(1920, 1080));
                        self.first_frame = false;
                    }
                } 
            });
        });
    }
}
