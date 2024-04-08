#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::App;
use tygydyk::feature::plugin::TygydykClientPlugin;

fn main() {
    App::new().add_plugins(TygydykClientPlugin).run();
}
