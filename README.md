[![Crates.io](https://img.shields.io/crates/v/bevy_missing_texture.svg)](https://crates.io/crates/bevy_missing_texture)

# Bevy library for Missing Textures

```rs
use bevy::prelude::*;
use bevy_missing_texture::*;
// Make sure to have `missing_texture.png` in your assets dir
fn main() {
    App::new()
        .add_plugins(MissingTexturePlugin)
        .add_systems(Startup, (will_fail, will_succeed));
}
// This will be replaced by the missing asset
fn will_fail(mut commands: Commands, asset_server: Res<AssetServer>, mut if_missing: ResMut<ReplaceIfMissing>) {
    let handle = asset_server.load("foo.png");
    commands.spawn(SpriteBundle {
        texture: handle.clone(),
        ..Default::default()
    });
    if_missing.push(handle);
}
// This will not be replaced by the missing asset
fn will_succeed(mut commands: Commands, asset_server: Res<AssetServer>, mut if_missing: ResMut<ReplaceIfMissing>) {
    let handle = asset_server.load("bar.png");
    commands.spawn(SpriteBundle {
        texture: handle.clone(),
        ..Default::default()
    });
    if_missing.push(handle);
}
```
