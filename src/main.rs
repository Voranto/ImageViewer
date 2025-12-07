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


    // HEADER NOT USED
    cursor += 2;

    // Skip whitespace
    while data[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    while data[cursor] == b'#' {
        while data[cursor] != b'\n' {
            cursor += 1;
        }
        cursor += 1; // newline also skip
    }

    // WIDTH
    let mut w = 0;
    while data[cursor].is_ascii_digit() {
        w = w * 10 + (data[cursor] - b'0') as usize;
        cursor += 1;
    }

    // WHITESPACE
    while data[cursor].is_ascii_whitespace() {
        cursor += 1;
    }

    // HEIGHT
    let mut h = 0;
    while data[cursor].is_ascii_digit() {
        h = h * 10 + (data[cursor] - b'0') as usize;
        cursor += 1;
    }
    println!("{} and {}",w,h);
    // WHITESPACE
    while data[cursor].is_ascii_whitespace() {
        cursor += 1;
    }

    // MAX COLOR VALUE -- NOT USED FOR NOW
    while data[cursor].is_ascii_digit() {
        cursor += 1;
    }

    cursor += 1;


    macroquad::window::request_new_screen_size(w as f32, h as f32);

    //Update screen size for next frame
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
