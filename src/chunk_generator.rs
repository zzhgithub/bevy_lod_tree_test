use bevy::{
    prelude::{Mut, Res, ResMut, Resource},
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use grid_tree::{glam::IVec3, NodeKey};
use ndshape::ConstShape;

use crate::{
    chunk::ChunkTreeMap, detect_new_slots::NewSlot, noise::generate_noise_chunk,
    sync_batch::SyncBatch, voxel_map::WorldVoxel, ChunkShape,
};

pub fn chunk_generator_system(
    frame_new_slots: Res<SyncBatch<NewSlot>>,
    mut chunk_map: ResMut<ChunkTreeMap>,
    mut generate_tasks: ResMut<GenerateTasks>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    let mut generated_chunks = Vec::new();

    // 完成生成任务
    for task in generate_tasks.tasks.drain(..) {
        // PERF: is this the best way to block on many futures?
        let (chunk_key, item) = future::block_on(task);
        generated_chunks.push((chunk_key, item));
    }

    // 更新所有的模块为 loading
    for slot in frame_new_slots.take_all().into_iter() {
        chunk_map.marked_as_loading(slot.key);
    }
    // 添加数据
    {
        for (key, chunk) in generated_chunks.into_iter() {
            if let Some(chunk) = chunk {
                chunk_map.write_chunk(key, chunk);
            } else {
                // TODO: this is a temporary hack to smooth voxels; we can't delete just any "empty" chunks (those without any
                // active edges) because there may be active edges between chunks, and the "empty" chunk might be responsible
                // for generated the surface that intersects those edges
                let fill = [WorldVoxel::EMPTY; ChunkShape::SIZE as usize]
                    .into_iter()
                    .collect::<Vec<WorldVoxel>>();
                chunk_map.write_chunk(key, fill);
                // map.chunks.delete_chunk(key);
            }
        }
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
    println!("这里完成对不同块的生成");
    // 这里在 通过 上面两个不同的 list 去生成不同的 task任务进行处理

    // todo!
    // 这里降低采样生成 比较简单的chunk数据的方法 TODO
    let downsampled_chunks = {
        let downsampled_chunks = thread_pool.scope(|scope| {
            for dst_chunk_key in downsample_slots.drain(..) {
                scope.spawn(async move {
                    // todo
                    // 这里生成低采样的数据
                });
            }
        });
    };

    // 生成新块的任务
    for key in generate_slots.drain(..) {
        let task = thread_pool.spawn(async move {
            // 这里面用于生成 数据
            // todo 后续也可以从文件中读取？
            let freq = 0.25;
            // 这个没有用到!!!!
            let scale = 5.0;
            let seed = 1010;
            let octaves = 6;
            // todo 这里应该从服务器生成获取 甚至 可以获取到 已经生成的数据
            let res = generate_noise_chunk(key, freq, scale, seed, octaves);
            (key, res)
        });
        generate_tasks.tasks.push(task);
    }
}

#[derive(Default, Resource)]
pub struct GenerateTasks {
    tasks: Vec<Task<GenerateTaskOutput>>,
}

pub type GenerateTaskOutput = (NodeKey<IVec3>, Option<Vec<WorldVoxel>>);
