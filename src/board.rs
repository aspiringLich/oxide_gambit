use engine::{chess::square::Square, rules::Rules, state::board_state::BoardState};

use crate::{
    assets::PieceAssets,
    theme::{rgb_hex, Theme},
    *,
};

pub const TILE_SIZE: f32 = 15.0;
pub const TILE_SPRITE_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE + 2.0);

#[derive(Resource)]
pub struct Board {
    pub active: bool,
    pub state: state::State,
}

impl std::ops::Deref for Board {
    type Target = BoardState;

    fn deref(&self) -> &Self::Target {
        &self.state.board_state
    }
}

#[derive(Component)]
pub struct DecorationEntity;

#[derive(Clone, Copy)]
pub enum Decoration {
    Highlight(Square),
    Move(Square),
    Clear,
}

impl Decoration {
    fn info(self) -> SpriteBundle {
        use Decoration::*;

        let (dx, dy, dz) = match self {
            Highlight(_) => (0.0, 1.0, 1.0),
            Move(_) => (0.0, 0.0, 2.0),
            Clear => unreachable!(),
        };
        let (color, size) = match self {
            Highlight(_) => (Color::rgba_u8(252, 219, 3, 63), Vec2::splat(TILE_SIZE)),
            Move(_) => (Color::rgba_u8(0, 0, 0, 127), Vec2::splat(4.0)),
            Clear => unreachable!(),
        };

        let square = match self {
            Highlight(square) | Move(square) => square,
            Clear => unreachable!(),
        };

        let transform = xy_to_transform(square.x() as usize, square.y() as usize, dx, dy, dz)
            .with_scale(size.extend(1.0));

        SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform,
            ..default()
        }
    }
}

pub fn draw_decorations(
    mut commands: Commands,
    q_decorations: Query<Entity, With<DecorationEntity>>,
    mut events: EventReader<Decoration>,
) {
    use Decoration::*;
    let mut clear = false;

    for decoration in events.iter() {
        match decoration {
            Clear => {
                if !clear {
                    for e in q_decorations.iter() {
                        commands.entity(e).despawn();
                    }
                    clear = true;
                }
            }
            _ => {
                commands.spawn((decoration.info(), DecorationEntity));
            }
        }
    }
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
    });
    // commands.init_resource::<Decorations>();
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

fn xy_to_transform(x: usize, y: usize, dx: f32, dy: f32, dz: f32) -> Transform {
    let x = x as f32 - 3.5;
    let _y = y as f32 - 3.5;
    Transform::from_xyz(
        x * TILE_SIZE + dx,
        _y * TILE_SIZE + dy + 1.0,
        8.0 - y as f32 + dz,
    )
}

pub fn spawn_board(
    mut commands: Commands,
    board: Res<Board>,
    assets: Res<PieceAssets>,
    mut decorations: EventWriter<Decoration>,
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

    decorations.send(Decoration::Clear);

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
                .spawn(sprite_sheet)
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
            let c = commands.spawn(sprite).name(&format!("Piece #{}", i)).id();
            children.push(c);
            commands
                .entity(c)
                .insert(TransformBundle::from_transform(xy_to_transform(
                    x, y, 0.0, 7.0, 1.0,
                )));
        }
    }

    // push the children onto the parent
    commands.entity(parent).push_children(&children);
}
