use bevy::prelude::{Mut, Res, ResMut};

use crate::{chunk::ChunkTreeMap, detect_new_slots::NewSlot, sync_batch::SyncBatch};

pub fn chunk_generator_system(
    frame_new_slots: Res<SyncBatch<NewSlot>>,
    mut chunk_map: ResMut<ChunkTreeMap>,
) {
    // 更新所有的模块为 loading
    for slot in frame_new_slots.take_all().into_iter() {
        chunk_map.marked_as_loading(slot.key);
    }
    // 循环检查 更新数据 到不同的地方
    let mut generate_slots = Vec::new();
    let mut downsample_slots = Vec::new();
    {
        chunk_map.clipmap_loading_slots(|key| {
            if key.level == 0 {
                generate_slots.push(key);
            } else {
                downsample_slots.push(key);
            }
        });
    }
    println!("这里完成对不同块的生成")
    // 这里在 通过 上面两个不同的 list 去生成不同的 task任务进行处理

    // todo!
}
