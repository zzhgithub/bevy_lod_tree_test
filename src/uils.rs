use bevy::prelude::Vec3;
use grid_tree::glam::IVec3;

pub fn vec3_to_veci3(vec: Vec3) -> IVec3 {
    let tmp = vec.as_ivec3();
    IVec3 {
        x: tmp.x,
        y: tmp.y,
        z: tmp.z,
    }
}
