use bevy::prelude::*;

use escaping_the_depths::stream_logic::StreamLogic;

fn main() {
    let mut streaming_application = App::new();
    streaming_application.add_plugins(StreamLogic);

    streaming_application.run();
}
