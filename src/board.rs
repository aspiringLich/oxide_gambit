use engine::rules::Rules;

use crate::{assets::PieceAssets, theme::Theme, *};

pub const TILE_SIZE: f32 = 15.0;
pub const TILE_SPRITE_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE + 2.0);

#[derive(Resource)]
pub struct Board {
    pub active: bool,
    pub state: state::State,
}

pub fn init(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.2;
    commands.spawn(camera);

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
        let texture_atlas = TextureAtlas::from_grid(img, TILE_SPRITE_SIZE, 2, 1, None, None);

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
        if let Ok(e) = q_entity.get_single() {
            commands.entity(e).despawn_recursive();
        }
        *active = board.active;
    } else {
        return;
    }

    // if the board is not active we don't spawn it
    if !*active {
        return;
    }

    let xy_to_transform = |x: usize, y: usize, dx: f32, dy: f32, dz: f32| {
        let x = x as f32 - 3.5;
        let _y = y as f32 - 3.5;
        Transform::from_xyz(x * TILE_SIZE + dx, _y * TILE_SIZE + dy + 1.0, 8.0 - y as f32 + dy + dz)
    };

    let parent = commands
        .spawn((
            BoardEntity,
            TransformBundle::default(),
            VisibilityBundle {
                visibility: Visibility::Visible,
                ..default()
            },
        ))
        .name("Board Entities")
        .id();
    let mut children = vec![];

    // spawn the board
    for y in 0..8 {
        for x in 0..8 {
            let i = (x + y) % 2;
            let color = theme.square[i];

            let sprite_sheet = SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: i,
                    color,
                    ..default()
                },
                texture_atlas: tile_asset.clone(),
                transform: xy_to_transform(x, y, 0.0, 0.0, 0.0),
                ..default()
            };

            let c = commands
                .spawn((sprite_sheet, BoardEntity))
                .name(&format!("Tile #{}", y * 8 + x))
                .id();
            children.push(c);
        }
    }

    let board_state = &board.state.board_state;
    for (i, piece) in board_state.board().iter().enumerate() {
        if let Some(mut sprite) = assets.get_sprite(*piece.get(board_state.pieces())) {
            sprite.sprite.color = theme.piece[board_state.get_info(*piece).unwrap().team as usize];
            let x = i % 8;
            let y = i / 8;
            let c = commands
                .spawn((sprite, BoardEntity))
                .name(&format!("Piece #{}", i))
                .id();
            children.push(c);
            commands
                .entity(c)
                .insert(TransformBundle::from_transform(xy_to_transform(
                    x, y, 0.0, 6.0, 1.0
                )));
        }
    }

    // push the children onto the parent
    commands.entity(parent).push_children(&children);
}
