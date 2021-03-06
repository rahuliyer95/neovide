#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bridge;
mod editor;
mod window;
mod renderer;
mod error_handling;
mod redraw_scheduler;
mod settings;

#[macro_use] extern crate derive_new;
#[macro_use] extern crate rust_embed;
#[macro_use] extern crate lazy_static;

use lazy_static::initialize;

use bridge::BRIDGE;
use window::ui_loop;

pub const INITIAL_DIMENSIONS: (u64, u64) = (100, 50);

fn main() {
    initialize(&BRIDGE);
    ui_loop();
}
