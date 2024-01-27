#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::App;
use tygydyk::feature::TygydykPlugin;

fn main() {
    App::new().add_plugins(TygydykPlugin).run();
}
