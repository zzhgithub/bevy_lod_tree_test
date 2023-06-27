mod chunk;
mod chunk_generator;
mod clip_sphere;
mod clipmap;
mod detect_new_slots;
mod noise;
mod sync_batch;
mod uils;
mod voxel_map;

use bevy::{
    prelude::{bevy_main, App, Commands},
    DefaultPlugins,
};
use bevy_flycam::PlayerPlugin;
use chunk::ChunkTreeMap;
use chunk_generator::{chunk_generator_system, GenerateTasks};
use clip_sphere::{clip_spheres_system, ClipSpheres};
use clipmap::Sphere3;
use detect_new_slots::{detect_new_slots_system, NewSlot};
use grid_tree::{glam::IVec3, Level};
use ndshape::ConstShape3i32;
use sync_batch::SyncBatch;
pub type SmallKeyHashMap<K, V> = ahash::AHashMap<K, V>;

pub const OCTREE_HEIGHT: Level = 10;
pub const DETAIL: i32 = 1;
pub const DECT_LEVEL: Level = 2;
pub const RADIUS: f32 = 1500.00;

pub const CHUNK_SIZE: i32 = 16;

type ChunkShape = ConstShape3i32<16, 16, 16>;

#[bevy_main]
fn main() {
    let mut app_builder = App::new();

    app_builder
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        // 球体更新
        .add_system(clip_spheres_system)
        // 检查更新插槽
        .add_system(detect_new_slots_system)
        // 生成chunk数据!
        .add_system(chunk_generator_system)
        .run();
}

fn setup(mut commands: Commands) {
    // 设置地图模块
    let chunk_map = ChunkTreeMap::new();
    commands.insert_resource(chunk_map);

    // 设置探测圆
    let clip_spheres = ClipSpheres::new(Sphere3 {
        center: [0.0, 0.0, 0.0],
        radius: RADIUS,
        is_init: true,
    });
    commands.insert_resource(clip_spheres);

    // 设置slot的接受器
    let new_slots_batch = SyncBatch::<NewSlot>::default();
    commands.insert_resource(new_slots_batch);

    // 设置任务列表
    commands.insert_resource(GenerateTasks::default());
}
