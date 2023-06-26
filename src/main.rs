mod chunk;
mod clip_sphere;
mod clipmap;
mod detect_new_slots;
mod sync_batch;
mod uils;
mod voxel_map;

use bevy::{
    prelude::{bevy_main, App, Commands},
    DefaultPlugins,
};
use bevy_flycam::PlayerPlugin;
use chunk::ChunkTreeMap;
use clip_sphere::{clip_spheres_system, ClipSpheres};
use clipmap::Sphere3;
use detect_new_slots::{detect_new_slots_system, NewSlot};
use grid_tree::{glam::IVec3, Level};
use ndshape::ConstShape3i32;
use sync_batch::SyncBatch;
pub type SmallKeyHashMap<K, V> = ahash::AHashMap<K, V>;

pub const OCTREE_HEIGHT: Level = 10;
pub const DETAIL: i32 = 2;
pub const RADIUS: f32 = 1500.00;

type ChunkShape = ConstShape3i32<16, 16, 16>;

#[bevy_main]
fn main() {
    let mut app_builder = App::new();

    app_builder
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_system(clip_spheres_system)
        .add_system(detect_new_slots_system)
        .run();
}

fn setup(mut commands: Commands) {
    // 设置地图模块
    let chunk_map = ChunkTreeMap::new();
    commands.insert_resource(chunk_map);

    // 设置探测圆
    let clip_spheres = ClipSpheres::new(Sphere3 {
        center: IVec3::ZERO,
        radius: RADIUS,
    });
    commands.insert_resource(clip_spheres);

    // 设置slot的接受器
    let new_slots_batch = SyncBatch::<NewSlot>::default();
    commands.insert_resource(new_slots_batch);
}
