use std::fs;
use std::path::Path;

const CANVAS_WIDTH: i32 = 30;
const CANVAS_HEIGHT: i32 = 30;
const BLANK_CHARACTER: char = '⬛';
const CANVAS: &'static str = "canvas_data.txt";
const TEMP_CANVAS: &'static str = "temp_canvas_data.txt";

pub struct Config {
    pub width: i32,
    pub height: i32,
    pub blank_character: char,
    pub canvas_location: String,
    pub temp_canvas_location: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() > 0 {
            // we want a fresh canvas, so clean-up
            if Path::new(&CANVAS).exists() {
                fs::remove_file(&CANVAS).unwrap();
            }
        }

        args.next();

        let width = match args.next() {
            Some(width) => width.parse::<i32>().unwrap_or_else(|err| {
                println!("Problem parsing width: {}", err);
                return CANVAS_WIDTH;
            }),
            None => CANVAS_WIDTH,
        };

        let height = match args.next() {
            Some(height) => height.parse::<i32>().unwrap_or_else(|err| {
                println!("Problem parsing height: {}", err);
                return CANVAS_HEIGHT;
            }),
            None => CANVAS_HEIGHT,
        };

        let blank_character = match args.next() {
            Some(blank_character) => blank_character
                .chars()
                .take(1)
                .next()
                .unwrap_or_else(|| {
                    println!("Problem parsing blank character");
                    return BLANK_CHARACTER;
                }
            ),
            None => BLANK_CHARACTER,
        };

        let canvas_location = match args.next() {
            Some(canvas) => canvas,
            None => String::from(CANVAS),
        };

        let temp_canvas_location = match args.next() {
            Some(canvas) => canvas,
            None => String::from(TEMP_CANVAS),
        };

        Ok(Config { width, height, blank_character, canvas_location, temp_canvas_location })
    }
}