use std::{path::PathBuf, time::Duration};

use bevy::ecs::component::Mutable;
use bevy::input::InputPlugin;
use bevy::mesh::MeshPlugin;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::sprite::SpritePlugin;
use bevy::state::app::StatesPlugin;
use bevy::text::TextPlugin;
use bevy::window::WindowResolution;
use cucumber::World;

use bevy::prelude::*;

use surviving_the_trip::core_logic::{
    CoreLogic, GameOverTime, MovementTime, progressing::HungerBarTime, setting::*,
};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct MockGame {
    app: App,
    pub tiled_map_path: PathBuf,
}

impl MockGame {
    pub fn new() -> Self {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(InputPlugin);
        app.add_plugins(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        });
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(MeshPlugin);
        app.add_plugins(TextPlugin);
        app.add_plugins(RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        });
        app.add_plugins(ImagePlugin::default());
        app.add_plugins(SpritePlugin);
        app.add_plugins(StatesPlugin);
        app.add_plugins(DefaultPickingPlugins);

        let movement_time = MovementTime::new(Duration::from_secs(0));
        let game_over_time = GameOverTime::new(Duration::from_secs(0));
        let hunger_bar_time = HungerBarTime::new(Duration::from_secs(0));
        app.add_plugins(CoreLogic::new(
            movement_time,
            game_over_time,
            hunger_bar_time,
        ));

        Self {
            app,
            tiled_map_path: PathBuf::default(),
        }
    }

    pub fn tick(&mut self) {
        self.app.update();
    }

    pub fn broadcast<T>(&mut self, event: T)
    where
        T: Message,
    {
        self.app
            .world_mut()
            .write_message(event)
            .expect("broadcast: Could not send event.");

        self.tick();
    }

    pub fn get_one<T>(&mut self) -> &T
    where
        T: Component,
    {
        self.app
            .world_mut()
            .query::<&T>()
            .iter(self.app.world_mut())
            .next()
            .expect("get_one: Could not find one instance of the specified component.")
    }

    pub fn get_one_mut<T>(&mut self) -> Mut<'_, T>
    where
        T: Component<Mutability = Mutable>,
    {
        self.app
            .world_mut()
            .query::<&mut T>()
            .iter_mut(self.app.world_mut())
            .next()
            .expect("get_one_mut: Could not find one mutable instance of the specified component")
    }

    pub fn get_at<T>(&mut self, logical_coordinates: &LogicalCoordinates) -> &T
    where
        T: Component,
    {
        self.app
            .world_mut()
            .query::<(&T, &LogicalCoordinates)>()
            .iter(self.app.world_mut())
            .find(|trap| trap.1 == logical_coordinates)
            .expect("get_at: Could not find component at logical coordinates.")
            .0
    }

    pub fn get_with<T, U>(&mut self) -> &T
    where
        T: Component,
        U: Component,
    {
        self.app
            .world_mut()
            .query_filtered::<&T, With<U>>()
            .iter(self.app.world_mut())
            .next()
            .expect("get_with: Could not find component with dependency.")
    }

    pub fn get_all_without<T, F>(&mut self) -> Vec<&T>
    where
        T: Component,
        F: Component + PartialEq,
    {
        self.app
            .world_mut()
            .query_filtered::<&T, Without<F>>()
            .iter(self.app.world_mut())
            .collect()
    }

    pub fn get_resource<T>(&mut self) -> &T
    where
        T: Resource,
    {
        self.app
            .world_mut()
            .get_resource::<T>()
            .expect("get_resource: Could not find the desired resource.")
    }

    pub fn get_resource_mut<T>(&mut self) -> Mut<'_, T>
    where
        T: Resource,
    {
        self.app
            .world_mut()
            .get_resource_mut::<T>()
            .expect("get_resource_mut: Could not find the desired resource.")
    }

    pub fn get_game_state<T>(&mut self) -> &State<T>
    where
        T: States,
    {
        self.get_resource::<State<T>>()
    }
}

impl Default for MockGame {
    fn default() -> Self {
        Self::new()
    }
}
