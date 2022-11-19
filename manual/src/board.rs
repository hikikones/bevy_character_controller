use bevy::prelude::*;

use bevy_bootstrap::{MaterialName, MeshName, MyAssets};
use bevy_grid::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Platforms::new())
            .add_startup_system_set(
                SystemSet::new()
                    .with_system(set_platforms)
                    .with_system(spawn_platforms.after(set_platforms)),
            );
    }
}

#[derive(Resource, Deref, DerefMut)]
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
    Skate,
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

fn set_platforms(mut platforms: ResMut<Platforms>) {
    platforms.set_tile(SquareCell::ZERO, Platform::Ground);

    for cell in CellDirectionIter::new(SquareCell::ZERO, 1) {
        platforms.set_tile(cell, Platform::Ground);
    }

    for cell in CellBoxIter::new(SquareCell::new(-8, -8), 16, 4) {
        platforms.set_tile(cell, Platform::Ice);
    }

    for cell in CellBoxIter::new(SquareCell::new(-8, 8), 16, 4) {
        platforms.set_tile(cell, Platform::Skate);
    }
}

fn spawn_platforms(platforms: Res<Platforms>, assets: Res<MyAssets>, mut commands: Commands) {
    for (cell, platform) in platforms.iter() {
        let material = match platform {
            Platform::Ground => MaterialName::Black,
            Platform::Ice => MaterialName::Cyan,
            Platform::Skate => MaterialName::MidnightBlue,
        };

        commands.spawn(PbrBundle {
            mesh: assets.mesh(MeshName::Cube),
            material: assets.material(material),
            transform: Transform {
                translation: platforms.get_point(*cell) - Vec3::Y * 0.5,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
