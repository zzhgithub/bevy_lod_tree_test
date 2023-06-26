use bevy::prelude::{Query, Res, ResMut, Resource, Transform, With};
use bevy_flycam::FlyCam;

use crate::{clipmap::Sphere3, uils::vec3_to_veci3, RADIUS};

#[derive(Resource)]
pub struct ClipSpheres {
    pub old_sphere: Sphere3,
    pub new_sphere: Sphere3,
}

impl ClipSpheres {
    pub fn new(sphere: Sphere3) -> Self {
        Self {
            old_sphere: sphere,
            new_sphere: sphere,
        }
    }
}

pub fn clip_spheres_system(
    // config: Res<MapConfig>,
    mut query: Query<&mut Transform, With<FlyCam>>,
    mut clip_spheres: ResMut<ClipSpheres>,
) {
    // 这里找到的是 FlyCam的相机 这里的相机后续应该是一种配置？？ 似乎不需要 应该可以切换？ 这里令人迷惑！！！！
    let position = if let Some(trf) = query.iter().next() {
        trf.translation
    } else {
        return;
    };
    // println!("position update: {:?}", position);
    clip_spheres.old_sphere = clip_spheres.new_sphere;
    clip_spheres.new_sphere = Sphere3 {
        center: vec3_to_veci3(position),
        radius: RADIUS,
    }
}
