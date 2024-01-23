use bevy::prelude::App;
use surrounded::feature::SurroundedPlugin;

fn main() {
    App::new().add_plugins(SurroundedPlugin).run()
}
