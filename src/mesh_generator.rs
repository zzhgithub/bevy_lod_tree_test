//生成mesh

use bevy::prelude::{Assets, Commands, Entity, Mesh, ResMut, Resource};
use grid_tree::{glam::IVec3, NodeKey};

use crate::{
    chunk::ChunkTreeMap, clip_sphere::ClipSpheres, clipmap::find_chunk_to_render, SmallKeyHashMap,
    DETAIL, OCTREE_HEIGHT,
};

#[derive(Default, Resource)]
pub struct ChunkMeshes {
    // Map from chunk key to mesh entity.
    entities: SmallKeyHashMap<NodeKey<IVec3>, Entity>,
}

pub fn mesh_generator_system(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkTreeMap>,
    mut clip_spheres: ResMut<ClipSpheres>,
    mut chunk_meshes: ResMut<ChunkMeshes>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // 创建 spawn or despawn mesh prb boudle

    // 获取要生成mesh的Nodekey 要保证没有被加载过
    find_chunk_to_render(
        OCTREE_HEIGHT,
        DETAIL,
        clip_spheres.new_sphere,
        |(key, is_render)| {
            if (is_render && !chunk_meshes.entities.contains_key(&key)) {
                // 现在才能把任务抛进来
            }
        },
    )
    // 生成每个tunk下的mesh
    
    // spawn
}
