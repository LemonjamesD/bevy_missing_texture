#![warn(missing_docs)]

pub struct MissingTexturePlugin;

impl Plugin for MissingTexturePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ReplaceIfMissing(Vec<Handle<Image>>);

#[derive(Component)]
struct MissingTexture(Handle<Image>);

fn missing_texture_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("missing_texture.png");
    commands.spawn(SpriteBundle {
        texture: handle.clone(),
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
        ..Default::default()
    });
    commands.spawn(MissingTexture(handle));
}

fn missing_texture(
    asset_server: Res<AssetServer>,
    mut checked: ResMut<CheckedImages>,
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
