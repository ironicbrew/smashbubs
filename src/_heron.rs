use bevy::prelude::*;
use heron::prelude::*;

pub struct HeronPlugin;
impl Plugin for HeronPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -1000., 0.0)));
    }
}

