// 这里的一个方法生成要计算出的槽位！

use grid_tree::{glam::IVec3, NodeEntry, NodeKey, NodePtr, OctreeI32, VisitCommand};

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
    new_sphere: Sphere3,
    // 处理回调函数
    mut rx: impl FnMut(NodeKey<IVec3>),
) {
    // 先设置一个最开始的 值
    let mut tree = OctreeI32::new(hegiht);
    let target_key = NodeKey::new(0, new_sphere.center);
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
                    // 这里可以知道 Level == 0 的位置的大小
                    // 比例数
                    // let scale_factor = 2i32.pow(child_ptr.level() as u32); //(2.level次方？)
                    // let child_min = child_coords * scale_factor;
                    // let child_max = child_min + IVec3::splat(scale_factor);

                    if child_ptr.level() == root_key.level {
                        // 这里是根节点
                    } else if child_ptr.level() == 0 {
                        // 这里的条件 应该和 观察的半径共同起作用
                        // 这里是要加载的最终节点 这个一个节点 就是16 x 16 x 16 的数据
                        rx(NodeKey::new(child_ptr.level(), child_coords));
                    } else {
                        // 这里非 最终节点
                    };
                    VisitCommand::Continue
                },
            );
        });
}

#[derive(Clone, Copy)]
pub struct Sphere3 {
    pub center: IVec3,
    // todo 在里面这个值没有用到
    pub radius: f32,
}
