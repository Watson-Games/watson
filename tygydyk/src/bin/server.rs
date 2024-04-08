use bevy::prelude::App;
use tygydyk::feature::plugin::TygydykServerPlugin;

fn main() {
    App::new().add_plugins(TygydykServerPlugin).run();
}
