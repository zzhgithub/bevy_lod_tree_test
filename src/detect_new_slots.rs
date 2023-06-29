use bevy::prelude::Res;
use grid_tree::{glam::IVec3, NodeKey};

use crate::{
    clip_sphere::ClipSpheres, clipmap::find_slot_by_sphere, sync_batch::SyncBatch, DETAIL,
    OCTREE_HEIGHT,
};

pub struct NewSlot {
    pub key: NodeKey<IVec3>,
    pub min: IVec3,
    pub max: IVec3,
    pub is_render: bool,
}

impl NewSlot {
    pub fn new(key: NodeKey<IVec3>, is_render: bool) -> Self {
        let coords = key.coordinates;
        let scale_factor = 2i32.pow(key.level as u32); //(2.level次方？)
        let child_min = coords * scale_factor;
        let child_max = child_min + IVec3::splat(scale_factor);
        NewSlot {
            key: key,
            min: child_min,
            max: child_max,
            is_render: is_render,
        }
    }

    pub fn min_u32(&self) -> [u32; 3] {
        [self.min.x as u32, self.min.y as u32, self.min.z as u32]
    }

    pub fn max_u32(&self) -> [u32; 3] {
        [self.max.x as u32, self.max.y as u32, self.max.z as u32]
    }
}

pub fn detect_new_slots_system(
    clip_spheres: Res<ClipSpheres>,
    frame_new_slots: Res<SyncBatch<NewSlot>>,
) {
    let mut new_slots = Vec::new();
    find_slot_by_sphere(
        OCTREE_HEIGHT,
        DETAIL,
        clip_spheres.old_sphere,
        clip_spheres.new_sphere,
        |new_slot| new_slots.push(new_slot),
    );
    frame_new_slots.extend(new_slots.into_iter().map(|s| NewSlot::new(s.0, s.1)));
    println!("Finish slot;")
}
