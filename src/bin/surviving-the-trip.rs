use bevy::prelude::*;

use surviving_the_trip::stream_logic::StreamLogic;

fn main() {
    let mut streaming_application = App::new();
    streaming_application.add_plugins(StreamLogic);

    streaming_application.run();
}
