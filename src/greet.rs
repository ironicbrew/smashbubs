struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    _query: Query<&PlayerName, With<player::Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
        //     // println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_player.system())
            .add_system(greet_people.system());
    }
}