use bevy::prelude::App;
use tygydyk::feature::TygydykServerPlugin;

fn main() {
    App::new().add_plugins(TygydykServerPlugin).run();
}
