use crate::{
    texture::{TextureId, TextureSizes},
    Plane3d, Vector2, Vector3,
};
use shalrath::repr::{TextureOffset, TexturePlane};
use std::collections::BTreeMap;

use super::{FaceId, FacePlanes, FaceVertices};

#[derive(Debug, Clone)]
pub struct FaceUvs(BTreeMap<FaceId, Vec<Vector2>>);

impl FaceUvs {
    pub fn new(
        faces: &Vec<FaceId>,
        textures: &BTreeMap<TextureId, String>,
        face_textures: &BTreeMap<FaceId, TextureId>,
        face_vertices: &FaceVertices,
        face_planes: &FacePlanes,
        face_texture_offsets: &BTreeMap<FaceId, TextureOffset>,
        face_texture_rotations: &BTreeMap<FaceId, f32>,
        face_texture_scales: &BTreeMap<FaceId, Vector2>,
        texture_sizes: &TextureSizes,
    ) -> Self {
        let mut face_uvs = BTreeMap::<FaceId, Vec<Vector2>>::default();

        for face_id in faces {
            let face_texture = &face_textures[face_id];
            let texture_size = texture_sizes.get(face_texture).copied().unwrap_or_else(|| {
                println!(
                    "Warning: Texture {} not found, generating UV with default size of 256x256",
                    &textures[face_texture],
                );
                (256, 256)
            });
            let face_vertices = face_vertices.vertices(face_id).unwrap();
            let face_plane = &face_planes[face_id];
            let face_texture_offset = &face_texture_offsets[face_id];
            let face_texture_rotation = &face_texture_rotations[face_id];
            let face_texture_scale = &face_texture_scales[face_id];

            face_uvs.insert(
                *face_id,
                face_vertices
                    .iter()
                    .map(|vertex| {
                        vertex_uv(
                            *vertex,
                            *face_plane,
                            *face_texture_offset,
                            *face_texture_rotation,
                            *face_texture_scale,
                            nalgebra::vector![texture_size.0 as f32, texture_size.1 as f32],
                        )
                    })
                    .collect(),
            );
        }

        FaceUvs(face_uvs)
    }

    pub fn get(&self, face_id: &FaceId) -> Option<&Vec<Vector2>> {
        self.0.get(face_id)
    }
}

impl std::ops::Index<&FaceId> for FaceUvs {
    type Output = Vec<Vector2>;

    fn index(&self, index: &FaceId) -> &Self::Output {
        &self.0[index]
    }
}

pub fn vertex_uv(
    vertex: Vector3,
    plane: Plane3d,
    texture_offset: TextureOffset,
    texture_rotation: f32,
    texture_scale: Vector2,
    texture_size: Vector2,
) -> Vector2 {
    match texture_offset {
        TextureOffset::Standard { u, v } => standard_uv(
            vertex,
            plane,
            u,
            v,
            texture_rotation,
            texture_scale,
            texture_size,
        ),
        TextureOffset::Valve { u, v } => valve_uv(vertex, u, v, texture_scale, texture_size),
    }
}

fn standard_uv(
    vertex: Vector3,
    brush_plane: Plane3d,
    u_offset: f32,
    v_offset: f32,
    texture_rotation: f32,
    texture_scale: Vector2,
    texture_size: Vector2,
) -> Vector2 {
    let up_vector = Vector3::z_axis();
    let right_vector = Vector3::y_axis();
    let forward_vector = Vector3::x_axis();

    let du = brush_plane.normal().dot(&up_vector).abs();
    let dr = brush_plane.normal().dot(&right_vector).abs();
    let df = brush_plane.normal().dot(&forward_vector).abs();

    let (x, y);
    if du >= dr && du >= df {
        x = vertex.x;
        y = -vertex.y;
    } else if dr >= du && dr >= df {
        x = vertex.x;
        y = -vertex.z;
    } else if df >= du && df >= dr {
        x = vertex.y;
        y = -vertex.z;
    } else {
        panic!("Zero-length normal");
    }

    let rot = nalgebra::Rotation2::new(texture_rotation.to_radians());

    let mut uv = rot * nalgebra::vector![x, y];
    uv.x /= texture_size.x;
    uv.y /= texture_size.y;
    uv.x /= texture_scale.x;
    uv.y /= texture_scale.y;

    let uv = uv + nalgebra::vector![u_offset / texture_size.x, v_offset / texture_size.y];

    uv
}

fn valve_uv(
    vertex: Vector3,
    u_plane: TexturePlane,
    v_plane: TexturePlane,
    texture_scale: Vector2,
    texture_size: Vector2,
) -> Vector2 {
    let un = nalgebra::vector![u_plane.x, u_plane.y, u_plane.z];
    let vn = nalgebra::vector![v_plane.x, v_plane.y, v_plane.z];

    let mut uv = nalgebra::vector![un.dot(&vertex), vn.dot(&vertex)];
    uv.x /= texture_size.x;
    uv.y /= texture_size.y;
    uv.x /= texture_scale.x;
    uv.y /= texture_scale.y;

    let uv = uv + nalgebra::vector![u_plane.d / texture_size.x, v_plane.d / texture_size.y];

    uv
}
