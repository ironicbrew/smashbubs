use bevy::prelude::*;

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .insert_resource(WindowDescriptor {
                title: "Smash Poopers!".to_string(),
                width: 600.,
                height: 600.,
                ..Default::default()
            });
    }
}
