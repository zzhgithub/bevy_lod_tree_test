// 这里的一个方法生成要计算出的槽位！

use grid_tree::{glam::IVec3, NodeEntry, NodeKey, NodePtr, OctreeI32, VisitCommand};
use ndshape::ConstShape;

use crate::{detect_new_slots::NewSlot, ChunkShape, CHUNK_SIZE, DECT_LEVEL};

trait CanSubdivide {
    fn can_subdivide(&self, node: Self, detail: i32) -> bool;
}

impl CanSubdivide for NodeKey<IVec3> {
    /// Adapted from https://github.com/Dimev/lodtree
    // 是否可以分割
    fn can_subdivide(&self, node_key: Self, detail: i32) -> bool {
        if node_key.level < self.level {
            return false;
        }

        let level_difference = node_key.level - self.level;
        let [s_x, s_y, s_z] = self.coordinates.to_array();
        let [n_x, n_y, n_z] = node_key.coordinates.to_array();

        // minimum corner of the bounding box
        let min = (
            (n_x << (level_difference + 1))
                .saturating_sub(((detail + 1) << level_difference) - (1 << level_difference)),
            (n_y << (level_difference + 1))
                .saturating_sub(((detail + 1) << level_difference) - (1 << level_difference)),
            (n_z << (level_difference + 1))
                .saturating_sub(((detail + 1) << level_difference) - (1 << level_difference)),
        );

        // max as well
        let max = (
            (n_x << (level_difference + 1))
                .saturating_add(((detail + 1) << level_difference) + (1 << level_difference)),
            (n_y << (level_difference + 1))
                .saturating_add(((detail + 1) << level_difference) + (1 << level_difference)),
            (n_z << (level_difference + 1))
                .saturating_add(((detail + 1) << level_difference) + (1 << level_difference)),
        );

        // local position of the target
        let local = (s_x << 1, s_y << 1, s_z << 1);

        // check if the target is inside of the bounding box
        local.0 >= min.0
            && local.0 < max.0
            && local.1 >= min.1
            && local.1 < max.1
            && local.2 >= min.2
            && local.2 < max.2
    }
}

pub fn find_slot_by_sphere(
    hegiht: u8,
    detail: i32,
    old_sphere: Sphere3,
    new_sphere: Sphere3,
    // 处理回调函数
    mut rx: impl FnMut((NodeKey<IVec3>, bool)),
) {
    // 先设置一个最开始的 值
    let mut tree = OctreeI32::new(hegiht);
    let target_key = NodeKey::new(0, new_sphere.to_ivec3());
    let root_key = NodeKey::new(tree.root_level(), IVec3::ZERO);
    // 先设置好这个值
    tree.fill_tree_from_root(root_key, 0, |key, entry| -> VisitCommand {
        match entry {
            // 不透明
            NodeEntry::Occupied(_) => {}
            // Vacant 空的
            NodeEntry::Vacant(v) => {
                v.insert(());
            }
        }

        if target_key.can_subdivide(key, detail) {
            VisitCommand::Continue
        } else {
            VisitCommand::SkipDescendants
        }
    });

    // 在遍历获取的不同的lod
    tree.iter_roots()
        .map(|(root_key, root_node)| (root_key, NodePtr::new(root_key.level, root_node.self_ptr)))
        .for_each(|(root_key, root_ptr)| {
            tree.visit_tree_depth_first(
                root_ptr,
                root_key.coordinates,
                0,
                |child_ptr, child_coords| {
                    // let scale_factor = 2i32.pow(child_ptr.level() as u32); //(2.level次方？)
                    // let child_min = child_coords * scale_factor;
                    // let child_max = child_min + IVec3::splat(scale_factor);

                    // if child_ptr.level() == root_key.level {
                    //     // 这里是根节点
                    // } else if child_ptr.level() == 0 {
                    //     // 这里的条件 应该和 观察的半径共同起作用
                    //     // 这里是要加载的最终节点 这个一个节点 就是16 x 16 x 16 的数据
                    //     rx(NodeKey::new(child_ptr.level(), child_coords));
                    // } else {
                    //     // 这里非 最终节点
                    // };
                    // 这里才要进行处理！！！
                    if (child_ptr.level() <= DECT_LEVEL) {
                        let node_key = NodeKey::new(child_ptr.level(), child_coords);
                        let chunk_shpere = chunk_bound_sphere(node_key);
                        let dist_to_old_clip_sphere =
                            old_sphere.distance(chunk_shpere) * CHUNK_SIZE as f32;
                        let dist_to_new_clip_sphere =
                            new_sphere.distance(chunk_shpere) * CHUNK_SIZE as f32;

                        let node_intersects_old_clip_sphere =
                            dist_to_old_clip_sphere - chunk_shpere.radius < old_sphere.radius
                                && !old_sphere.is_init;
                        let node_intersects_new_clip_sphere =
                            dist_to_new_clip_sphere - chunk_shpere.radius < new_sphere.radius;
                        println!("1");
                        if !node_intersects_new_clip_sphere {
                            // There are no events for this node or any of its descendants.
                            VisitCommand::Continue;
                        }
                        if !node_intersects_old_clip_sphere {
                            let is_render_candidate = node_key.level == 0
                                || dist_to_new_clip_sphere / chunk_shpere.radius
                                    > DECT_LEVEL as f32;
                            rx((
                                NodeKey::new(child_ptr.level(), child_coords),
                                is_render_candidate,
                            ));
                        }
                    }
                    // 这里与根节点无关！要判断一下 距离的问题！
                    VisitCommand::Continue
                },
            );
        });
}

#[derive(Clone, Copy)]
pub struct Sphere3 {
    pub center: [f32; 3],
    // todo 在里面这个值没有用到
    pub radius: f32,
    pub is_init: bool,
}

impl Sphere3 {
    pub fn to_ivec3(&self) -> IVec3 {
        IVec3::new(
            self.center[0] as i32,
            self.center[1] as i32,
            self.center[2] as i32,
        )
    }

    pub fn distance(&self, other: Self) -> f32 {
        ((self.center[0] - other.center[0]).powf(2.0)
            + (self.center[1] - other.center[1]).powf(2.0)
            + (self.center[2] - other.center[2]).powf(2.0))
        .sqrt()
    }
}

pub fn chunk_bound_sphere(key: NodeKey<IVec3>) -> Sphere3 {
    // 获取到 最大位置和最小位置？
    let slot = NewSlot::new(key, false);
    let shape = slot.max - slot.min;
    let center_x = (CHUNK_SIZE as f32) * slot.min.x as f32 + shape.x as f32 / 2.0;
    let center_y = (CHUNK_SIZE as f32) * slot.min.y as f32 + shape.y as f32 / 2.0;
    let center_z = (CHUNK_SIZE as f32) * slot.min.z as f32 + shape.z as f32 / 2.0;
    let radius = (1 << shape.x) as f32 * 3f32.sqrt() * CHUNK_SIZE as f32;
    Sphere3 {
        center: [center_x, center_y, center_z],
        radius: radius,
        is_init: false,
    }
}
