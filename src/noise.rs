use grid_tree::{glam::IVec3, NodeKey};
use ndshape::ConstShape;
use simdnoise::NoiseBuilder;

use crate::{detect_new_slots::NewSlot, voxel_map::WorldVoxel, ChunkShape, CHUNK_SIZE};

pub fn generate_noise_vec(node_key: NodeKey<IVec3>, freq: f32, seed: i32, octaves: u8) -> Vec<f32> {
    let slot = NewSlot::new(node_key, false);
    let min = slot.min;
    let (noise, _min_val, _max_val) = NoiseBuilder::fbm_3d_offset(
        min.x as f32,
        CHUNK_SIZE as usize,
        min.y as f32,
        CHUNK_SIZE as usize,
        min.z as f32,
        CHUNK_SIZE as usize,
    )
    .with_freq(freq)
    .with_seed(seed)
    .with_octaves(octaves)
    .generate();
    noise
}

pub fn generate_noise_chunk(
    node_key: NodeKey<IVec3>,
    freq: f32,
    scale: f32,
    seed: i32,
    octaves: u8,
) -> Option<Vec<WorldVoxel>> {
    // todo 这里转成 data 这里仅仅使用某个算法保证正确？
    let res = generate_noise_vec(node_key, freq, seed, octaves);
    let mut result: Vec<WorldVoxel> = Vec::new();
    res.into_iter().for_each(|a| {
        if a > 0.0 {
            result.push(WorldVoxel::FILLED)
        } else {
            result.push(WorldVoxel::EMPTY)
        }
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise() {
        // 测试 生成的 噪声
        let node = NodeKey::new(0, IVec3::ZERO);
        let freq = 0.25;
        let scale = 5.0;
        let seed = 1010;
        let octaves = 6;
        let res = generate_noise_vec(node, freq, seed, octaves);
        // 这里生成的数据的长度应该是 16 x 16 x 16
        assert!(res.len() == 16 * 16 * 16);
    }
}
