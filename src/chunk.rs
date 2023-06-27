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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    // 需要知道的状态？
    pub chunk_state: SmallKeyHashMap<NodeKey<IVec3>, ChunkState>,
}

impl<T> ChunkMap<T> {
    pub fn new() -> Self {
        let data_map = SmallKeyHashMap::<NodeKey<IVec3>, T>::new();
        let chunk_state = SmallKeyHashMap::<NodeKey<IVec3>, ChunkState>::new();
        ChunkMap {
            data_map: data_map,
            chunk_state: chunk_state,
        }
    }

    // 写数据到chunk中
    pub fn write_chunk(&mut self, key: NodeKey<IVec3>, chunk: T) -> Option<T> {
        // 这里自动设置一下
        self.chunk_state.insert(key, ChunkState::Render);
        self.data_map.insert(key, chunk)
    }

    // 擅长chunk中的数据
    pub fn remove_chunk(&mut self, key: NodeKey<IVec3>) {
        self.chunk_state.remove(&key);
        self.data_map.remove(&key);
    }
    pub fn marked_as_loading(&mut self, key: NodeKey<IVec3>) {
        // 查看原来的位置是否还需要Loading
        let old_state = self.chunk_state.get(&key);
        match old_state {
            Some(ChunkState::Loading) => {
                self.chunk_state.insert(key, ChunkState::Loading);
            }
            Some(ChunkState::Render) => (),
            None => {
                self.chunk_state.insert(key, ChunkState::Loading);
            }
        }
    }

    // 这种情况下怎么知道哪些是 删除的数据呢？
    // 循环操作loading的数据？
    pub fn clipmap_loading_slots(&self, mut rx: impl FnMut(NodeKey<IVec3>)) {
        for ele in self.chunk_state.iter() {
            if (ele.1.clone() == ChunkState::Loading) {
                rx(ele.0.clone());
            }
        }
    }
}

// 保存数据的最终map
pub type ChunkTreeMap = ChunkMap<Vec<WorldVoxel>>;
