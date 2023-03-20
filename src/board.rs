use engine::rules::Rules;

use crate::{assets::PieceAssets, theme::Theme, *};

pub const TILE_SPRITE_SIZE: Vec2 = Vec2::new(11.0, 13.0);
pub const TILE_SIZE: f32 = 11.0;

#[derive(Resource)]
pub struct Board {
    pub active: bool,
    pub state: state::State,
}

pub fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(Board {
        active: true,
        state: state::State::from_FEN(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ",
            Rules::standard(),
        )
        .unwrap(),
    })
}

#[derive(Component)]
pub struct BoardEntity;

#[derive(Resource, Deref)]
pub struct TileAsset(Handle<TextureAtlas>);

impl FromWorld for TileAsset {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let img = asset_server.load("tile.png");
        let mut texture_atlas = TextureAtlas::new_empty(img, TILE_SPRITE_SIZE * (Vec2::X * 2.0));
        texture_atlas.add_texture(Rect::from_corners(Vec2::ZERO, TILE_SPRITE_SIZE));
        texture_atlas.add_texture(Rect::from_corners(
            Vec2::X * TILE_SPRITE_SIZE.x,
            TILE_SPRITE_SIZE * (Vec2::X * 2.0),
        ));

        let handle = texture_atlases.add(texture_atlas);

        Self(handle)
    }
}

pub fn spawn_board(
    mut commands: Commands,
    board: Res<Board>,
    assets: Res<PieceAssets>,
    mut active: Local<bool>,
    tile_asset: Local<TileAsset>,
    theme: Res<Theme>,
    q_entity: Query<Entity, With<BoardEntity>>,
) {
    // if the board is changed we reset it
    if board.is_changed() || theme.is_changed() {
        for entity in q_entity.iter() {
            commands.entity(entity).despawn();
        }
        *active = board.active;
    }

    // if the board is not active we don't spawn it
    if *active {
        return;
    }

    let xy_to_transform = |x: usize, y: usize, dx: f32, dy: f32| {
        let x = x as f32;
        let y = y as f32;
        Transform::from_xyz(x * TILE_SIZE + dx, y * TILE_SIZE + dy, 8.0 - y + dy)
    };

    // spawn the board
    for y in 0..8 {
        for x in 0..8 {
            let i = (x + y) % 2;
            let color = theme.square[i];

            commands.spawn((
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: i,
                        color,
                        ..default()
                    },
                    texture_atlas: tile_asset.clone(),
                    transform: xy_to_transform(x, y, 0.0, 0.0),
                    ..default()
                },
                BoardEntity,
            ));
        }
    }
}
