use bevy::prelude::Resource;
use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};
use grid_tree::{NodeKey, NodePtr, OctreeI32, Tree};
use ndshape::{ConstShape3i32, Shape};

#[derive(Clone, Copy, PartialEq)]
pub struct WorldVoxel {
    pub id: i32,
}

impl WorldVoxel {
    pub const EMPTY: Self = Self { id: 1 };
    pub const FILLED: Self = Self { id: -1 };
}

// 这里实现这两个类型保证 可以在后续中使用 block_mesh 来进行greed mesh算法 生成网格
impl Voxel for WorldVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.id < 0 {
            VoxelVisibility::Opaque
        } else {
            VoxelVisibility::Empty
        }
    }
}

impl MergeVoxel for WorldVoxel {
    type MergeValue = i32;

    fn merge_value(&self) -> Self::MergeValue {
        self.id
    }
}
