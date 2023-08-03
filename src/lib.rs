//! Bevy library for Missing Textures
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_missing_texture::*;
//!
//! // Make sure to have `missing_texture.png` in your assets dir
//! fn main() {
//!     App::new()
//!         .add_plugins(MissingTexturePlugin)
//!         .add_systems(Startup, (will_fail, will_succeed));
//! }

//! // This will be replaced by the missing asset
//! fn will_fail(mut commands: Commands, asset_server: Res<AssetServer>, mut if_missing: ResMut<ReplaceIfMissing>) {
//!     let handle = asset_server.load("foo.png");
//!     commands.spawn(SpriteBundle {
//!         texture: handle.clone(),
//!         ..Default::default()
//!     });
//!     if_missing.push(handle);
//! }

//! // This will not be replaced by the missing asset
//! fn will_succeed(mut commands: Commands, asset_server: Res<AssetServer>, mut if_missing: ResMut<ReplaceIfMissing>) {
//!     let handle = asset_server.load("bar.png");
//!     commands.spawn(SpriteBundle {
//!         texture: handle.clone(),
//!         ..Default::default()
//!     });
//!     if_missing.push(handle);
//! }
//! ```

#![warn(missing_docs)]
use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Startup;
use bevy::app::Update;
use bevy::asset::AssetServer;
use bevy::asset::Assets;
use bevy::asset::Handle;
use bevy::asset::LoadState;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::ecs::system::Resource;
use bevy::log::info;
use bevy::prelude::Deref;
use bevy::prelude::DerefMut;
use bevy::render::texture::Image;

static mut MISSING_TEXTURE_PATH: &'static str = "missing_texture.png";

/// Plugin for setting up Missing Textures
pub struct MissingTexturePlugin;

impl MissingTexturePlugin {
    /// This overrides the default path which is `missing_texture.png` in your assets_dir
    pub fn new(path: &'static str) -> Self {
        unsafe {
            MISSING_TEXTURE_PATH = path;
        }
        Self
    }
}

impl Plugin for MissingTexturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReplaceIfMissing>()
            .add_systems(Startup, missing_texture_startup)
            .add_systems(Update, missing_texture);
    }
}

/// Use this to add a Image Handle to track an image handle and if it failed to load replace it
#[derive(Resource, Default, Deref, DerefMut)]
pub struct ReplaceIfMissing(Vec<Handle<Image>>);

#[derive(Component)]
struct MissingTexture(Handle<Image>);

fn missing_texture_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MissingTexture(
        asset_server.load(unsafe { MISSING_TEXTURE_PATH }),
    ));
}

fn missing_texture(
    asset_server: Res<AssetServer>,
    mut checked: ResMut<ReplaceIfMissing>,
    mut images: ResMut<Assets<Image>>,
    missing_texture: Query<&MissingTexture>,
) {
    if checked.len() == 0 {
        return;
    }
    let Some(missing_texture) = images.get(&missing_texture.single().0) else {
        return;
    };
    let missing_texture = missing_texture.clone();
    info!("Checking {} images", checked.len());
    checked.retain(|handle| {
        info!("{:?}", asset_server.get_handle_path(handle));
        let load_state = asset_server.get_load_state(handle);
        info!("{:?}", load_state);
        if load_state == LoadState::Loading {
            return true;
        }
        if load_state == LoadState::Failed {
            _ = images.set(handle, missing_texture.clone());
        }
        false
    });
}
