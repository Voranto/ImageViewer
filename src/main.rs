use macroquad::prelude::*;
use core::time;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;


#[macroquad::main("BasicShapes")]
async fn main() -> io::Result<()>{
    let data = fs::read("Dog_Breeds.ppm").unwrap();
    let mut cursor = 0;

    // Parse header
    assert!(data.starts_with(b"P6"));

    // Skip magic number
    cursor += 2;
    println!("{} ",cursor);
    // Skip whitespace
    while data[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    while data[cursor] == b'#' {
        while data[cursor] != b'\n' {
            cursor += 1;
        }
        cursor += 1; // skip newline
    }

    println!("{} ",cursor);
    // Read width
    let mut w = 0;
    while data[cursor].is_ascii_digit() {
        w = w * 10 + (data[cursor] - b'0') as usize;
        cursor += 1;
    }

    // Skip whitespace
    while data[cursor].is_ascii_whitespace() {
        cursor += 1;
    }

    // Read height
    let mut h = 0;
    while data[cursor].is_ascii_digit() {
        h = h * 10 + (data[cursor] - b'0') as usize;
        cursor += 1;
    }
    println!("{} and {}",w,h);
    // Skip whitespace
    while data[cursor].is_ascii_whitespace() {
        cursor += 1;
    }

    // Read max color value (skip it)
    while data[cursor].is_ascii_digit() {
        cursor += 1;
    }

    // Skip single whitespace after maxval
    cursor += 1;
    println!("{} ",cursor);
    macroquad::window::request_new_screen_size(w as f32, h as f32);
    next_frame().await;
    loop{
    // Now pixel data starts
    let rgb = &data[cursor..];

    let mut idx = 0;
    for y in 0..h {
        for x in 0..w {
            if(idx+2 >= rgb.len()){break;}
            let r = rgb[idx];
            let g = rgb[idx + 1];
            let b = rgb[idx + 2];
            idx += 3;
            //println!("{} , {} with values {},{},{}",x as f32, y as f32, r, g, b);
            draw_rectangle(
                x as f32,
                y as f32,
                1.0,
                1.0,
                Color::from_rgba(r as u8, g as u8, b as u8, 255),
            );
        }
    }
    
    
    next_frame().await;
    }
    Ok(())
}
