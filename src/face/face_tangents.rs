use std::collections::BTreeMap;

use shalrath::repr::TextureOffset;

use crate::{face::FaceId, vector3_from_texture_plane, FacePlanes, Plane3d, Vector2, Vector3};

// TODO: Replace GeoPlane usage with custom tangent type
//       (Would storing a basis be viable? No need to conform to godot standards)

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Basis {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

#[derive(Debug, Clone)]
pub struct FaceBases(BTreeMap<FaceId, Basis>);

impl FaceBases {
    pub fn new(
        planes: &Vec<FaceId>,
        geo_planes: &FacePlanes,
        face_offsets: &BTreeMap<FaceId, TextureOffset>,
        face_angles: &BTreeMap<FaceId, f32>,
        face_scales: &BTreeMap<FaceId, Vector2>,
    ) -> Self {
        let mut face_bases = BTreeMap::<FaceId, Basis>::default();
        for plane_id in planes {
            face_bases.insert(
                *plane_id,
                face_basis(
                    &geo_planes[plane_id],
                    &face_offsets[plane_id],
                    face_angles[plane_id],
                    face_scales[plane_id],
                ),
            );
        }
        FaceBases(face_bases)
    }

    pub fn get(&self, face_id: &FaceId) -> Option<&Basis> {
        self.0.get(face_id)
    }
}

impl std::ops::Index<&FaceId> for FaceBases {
    type Output = Basis;

    fn index(&self, index: &FaceId) -> &Self::Output {
        &self.0[index]
    }
}

fn face_basis(
    geo_plane: &Plane3d,
    offset: &TextureOffset,
    angle: f32,
    scale: Vector2,
) -> Basis {
    match &offset {
        shalrath::repr::TextureOffset::Standard { .. } => standard_basis(geo_plane, angle, scale),
        shalrath::repr::TextureOffset::Valve { .. } => valve_basis(geo_plane, offset),
    }
}

fn standard_basis(plane: &Plane3d, angle: f32, scale: Vector2) -> Basis {
    let up_vector: &Vector3 = &Vector3::z_axis();
    let right_vector: &Vector3 = &Vector3::y_axis();
    let forward_vector: &Vector3 = &Vector3::x_axis();

    let normal = plane.normal();

    let du = normal.dot(up_vector);
    let dr = normal.dot(right_vector);
    let df = normal.dot(forward_vector);

    let du_abs = du.abs();
    let dr_abs = dr.abs();
    let df_abs = df.abs();

    let du_sign = du.signum();
    let dr_sign = dr.signum();
    let df_sign = df.signum();

    let quat = nalgebra::UnitQuaternion::new(normal * -angle.to_radians());
    if du_abs >= dr_abs && du_abs >= df_abs {
        let z = *plane.normal() * du_sign;
        let x = z.cross(forward_vector).normalize();
        let y = z.cross(right_vector).normalize();
        Basis {
            x,
            y,
            z,
        }
    } else if dr_abs >= du_abs && dr_abs >= df_abs {
        let z = *plane.normal() * dr_sign;
        let x = z.cross(up_vector).normalize();
        let y = z.cross(forward_vector).normalize();
        Basis {
            x,
            y,
            z,
        }
    } else if df_abs >= du_abs && df_abs >= dr_abs {
        let z = *plane.normal() * df_sign;
        let x = z.cross(up_vector).normalize();
        let y = z.cross(right_vector).normalize();
        Basis {
            x,
            y,
            z,
        }
    } else {
        panic!("Failed to generate basis")
    }
}

fn valve_basis(plane: &Plane3d, texture_offset: &TextureOffset) -> Basis {
    if let shalrath::repr::TextureOffset::Valve { u, v } = &texture_offset {
        let u = vector3_from_texture_plane(u);
        let v = vector3_from_texture_plane(v);
        Basis {
            x: u,
            y: v,
            z: *plane.normal(),
        }
    } else {
        panic!("Not a valve UV");
    }
}
