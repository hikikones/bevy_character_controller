use bevy::prelude::*;

use bevy_bootstrap::{MaterialName, MeshName, MyAssets};
use bevy_grid::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Platforms::new());
    }
}

#[derive(Deref, DerefMut)]
pub struct Platforms(GridInfinite<SquareCell, Platform>);

impl Platforms {
    pub fn new() -> Self {
        Self(GridInfinite::new(1.0))
    }
}

#[derive(Default)]
pub enum Platform {
    #[default]
    Ground,
    Ice,
}

impl<C> GridTile<C> for Platform
where
    C: GridCell,
{
    type Neighbors = C::Neighbors;

    fn is_walkable(&self) -> bool {
        false
    }

    fn neighbors(&self, cell: C) -> Self::Neighbors {
        cell.neighbors()
    }
}

fn spawn_platforms(
    mut platforms: ResMut<Platforms>,
    mut commands: Commands,
    assets: Res<MyAssets>,
) {
    //todo
}
