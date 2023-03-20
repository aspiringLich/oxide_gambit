use std::ops::Index;

use crate::*;
use bevy::prelude::*;
use engine::state::board_state::PIECE_INFO;
use rules::piece::Piece;

#[derive(Resource)]
pub struct PieceAssets(Handle<TextureAtlas>);

impl PieceAssets {
    pub fn get_sprite(&self, piece: Piece) -> Option<SpriteSheetBundle> {
        let piece = unsafe { PIECE_INFO.get(piece as usize)?.as_ref()? };
        Some(SpriteSheetBundle {
            texture_atlas: self.0.clone(),
            sprite: TextureAtlasSprite {
                index: piece.sprite_index,
                ..default()
            },
            ..default()
        })
    }
}

/// Initialize piece assets
pub fn init_piece_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let atlas = asset_server.load("assets/pieces.png");
    let texture_atlas = TextureAtlas::from_grid(
        atlas,
        [11.0, 23.0].into(),
        8,
        2,
        Some([1.0, 1.0].into()),
        None,
    );
    let handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(PieceAssets(handle))
}
