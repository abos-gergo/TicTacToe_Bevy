use crate::ScreenState;
use bevy::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>();
        app.add_systems(OnEnter(ScreenState::Game), spawn_board);
    }
}

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(asset_server.load("board.png")));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
enum GameState {
    #[default]
    SearchingForOpponent,
    PlayerTurn,
    OpponentTurn,
    GameOver,
}

/// Representing a single tile, Player's tiles are X, opponents tiles are O.
#[derive(PartialEq, Clone, Default)]
enum Tile {
    #[default]
    None,
    Player,
    Opponent,
}

#[derive(Resource, Default)]
struct Board {
    /// `[Tile ; 3]` is a single column, and there are 3 columns.
    tiles: [[Tile; 3]; 3],
}

impl Board {
    // UNTESTED FUNCTION? JUST A PROTOTYPE
    // Hopefully I can code this without errors.
    fn check_for_win(&self) -> Tile {
        //Columns
        for col in self.tiles.iter() {
            if col.iter().all(|t| *t == Tile::Opponent) || col.iter().all(|t| *t == Tile::Player) {
                return col[0].clone();
            }
        }

        //Rows
        for i in 0..3 {
            if self.tiles[0][i] == self.tiles[1][i] && self.tiles[1][i] == self.tiles[2][i] {
                return self.tiles[0][i].clone();
            }
        }

        //Diagonals
        if self.tiles[0][0] == self.tiles[1][1] && self.tiles[1][1] == self.tiles[2][2] {
            return self.tiles[0][0].clone();
        }
        if self.tiles[0][2] == self.tiles[1][1] && self.tiles[1][1] == self.tiles[2][0] {
            return self.tiles[0][2].clone();
        }

        return Tile::None;
    }

    fn place_tile(&mut self, x: usize, y: usize, tile: Tile) {
        #[cfg(debug_assertions)]
        if self.tiles[x][y] != Tile::None {
            warn!("Tile placed on an existing tile!, x: {}, y: {}", x, y);
        }

        self.tiles[x][y] = tile;
    }
}
