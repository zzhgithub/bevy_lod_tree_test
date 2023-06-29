use bevy::{
    prelude::Mesh,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
use grid_tree::{glam::IVec3, NodeKey};
use ndshape::{ConstShape, ConstShape3u32};

use crate::{chunk::ChunkTreeMap, detect_new_slots::NewSlot, CHUNK_SIZE};

pub fn create_mesh(key: NodeKey<IVec3>, chunk_map: ChunkTreeMap) -> Option<Mesh> {
    let voxels = if let Some(v) = chunk_map.data_map.get(&key) {
        v.to_owned()
    } else {
        Vec::new()
    };

    if voxels.is_empty() {
        return None;
    }
    let solt = NewSlot::new(key, false);
    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
    type SampleShape = ConstShape3u32<18, 18, 18>;
    let mut buffer = GreedyQuadsBuffer::new(SampleShape::SIZE as usize);
    greedy_quads(
        &voxels,
        &SampleShape {},
        solt.min_u32(),
        solt.max_u32(),
        &faces,
        &mut buffer,
    );
    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;
    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);
    for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
            tex_coords.extend_from_slice(&face.tex_coords(
                RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                true,
                &quad,
            ));
        }
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // 这里没有缩放 每个格子会占据一个图片？
    for uv in tex_coords.iter_mut() {
        for c in uv.iter_mut() {
            *c *= CHUNK_SIZE as f32;
        }
    }

    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
    render_mesh.set_indices(Some(Indices::U32(indices)));
    Some(render_mesh)
}
