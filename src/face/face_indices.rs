use std::{cmp::Ordering, collections::BTreeMap};

use shalrath::repr::Triangle;

use super::{FaceCenters, FaceId, FaceVertices};
use crate::{vector3_from_point, FacePlanes, Plane3d, Vector3};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FaceWinding {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone)]
pub struct FaceIndices(BTreeMap<FaceId, Vec<usize>>);

impl FaceIndices {
    // Generate face indices with the specified winding
    pub fn new(
        face_planes: &BTreeMap<FaceId, Triangle>,
        geo_planes: &FacePlanes,
        face_vertices: &FaceVertices,
        face_centers: &FaceCenters,
        winding: FaceWinding,
    ) -> FaceIndices {
        let mut wound_indices = BTreeMap::<FaceId, Vec<usize>>::default();
        for (plane_id, vertices) in face_vertices.iter_vertices() {
            let face_plane = &face_planes[&plane_id];
            let plane = &geo_planes[&plane_id];
            let plane_center = &face_centers[&plane_id];

            let plane_v0 = vector3_from_point(face_plane.v0);
            let plane_v1 = vector3_from_point(face_plane.v1);
            let u_axis = (plane_v1 - plane_v0).normalize();
            let v_axis = plane.normal().cross(&u_axis);

            let mut indices = (0..vertices.len()).collect::<Vec<_>>();
            indices.sort_unstable_by(|lhs, rhs| {
                let lhs_v = &vertices[*lhs] - plane_center;
                let rhs_v = &vertices[*rhs] - plane_center;

                let lhs_pu = lhs_v.dot(&u_axis);
                let lhs_pv = lhs_v.dot(&v_axis);

                let rhs_pu = rhs_v.dot(&u_axis);
                let rhs_pv = rhs_v.dot(&v_axis);

                let lhs_angle = lhs_pv.atan2(lhs_pu);
                let rhs_angle = rhs_pv.atan2(rhs_pu);

                if winding == FaceWinding::CounterClockwise {
                    rhs_angle.partial_cmp(&lhs_angle)
                } else {
                    lhs_angle.partial_cmp(&rhs_angle)
                }
                .unwrap_or(Ordering::Equal)
            });
            wound_indices.insert(*plane_id, indices);
        }
        FaceIndices(wound_indices)
    }

    pub fn get(&self, face_id: &FaceId) -> Option<&Vec<usize>> {
        self.0.get(face_id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::ops::Index<&FaceId> for FaceIndices {
    type Output = Vec<usize>;

    fn index(&self, index: &FaceId) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> IntoIterator for &'a FaceIndices {
    type Item = (&'a FaceId, &'a Vec<usize>);

    type IntoIter = std::collections::btree_map::Iter<'a, FaceId, Vec<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for FaceIndices {
    type Item = (FaceId, Vec<usize>);

    type IntoIter = std::collections::btree_map::IntoIter<FaceId, Vec<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
