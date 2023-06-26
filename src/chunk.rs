use bevy::prelude::Resource;
use grid_tree::{glam::IVec3, NodeKey, OctreeI32};
use ndshape::{ConstShape, ConstShape3i32};

use crate::{voxel_map::WorldVoxel, ChunkShape, SmallKeyHashMap};

#[derive(Clone, Copy, PartialEq)]
pub struct Chunk {
    // chunk的关键点
    pub chunk_key: NodeKey<IVec3>,
    // 记录当前chunk的状态？
    pub state: ChunkState,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    /// This bit is set if the chunk is currently loading.
    Loading = 0,
    /// This bit is set if the chunk is currently being rendered.
    Render = 1,
}

#[derive(Clone, Debug, Resource, Default)]
pub struct ChunkMap<T> {
    // 用于存储 数据
    pub data_map: SmallKeyHashMap<NodeKey<IVec3>, T>,
}

impl<T> ChunkMap<T> {
    pub fn new() -> Self {
        let data_map = SmallKeyHashMap::<NodeKey<IVec3>, T>::new();
        ChunkMap { data_map: data_map }
    }

    // 写数据到chunk中
    pub fn write_chunk(&self, key: NodeKey<IVec3>, chunk: T) -> Option<&T> {
        self.data_map.get(&key)
    }

    // 擅长chunk中的数据
    pub fn remove_chunk(&mut self, key: NodeKey<IVec3>) {
        self.data_map.remove(&key);
    }
}

// 保存数据的最终map
pub type ChunkTreeMap = ChunkMap<[WorldVoxel; ChunkShape::SIZE as usize]>;
