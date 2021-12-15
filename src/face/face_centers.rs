use std::collections::BTreeMap;

use super::{FaceId, FaceVertices};
use crate::Vector3;

#[derive(Debug, Clone)]
pub struct FaceCenters(BTreeMap<FaceId, Vector3>);

impl FaceCenters {
    // Calculate face centers
    pub fn new(face_vertices: &FaceVertices) -> Self {
        let mut plane_world_centers: BTreeMap<FaceId, Vector3> = Default::default();
        for (face_id, vertices) in face_vertices.iter_vertices() {
            let mut center = Vector3::zeros();
            for world_vertex in vertices {
                center += world_vertex;
            }
            center /= vertices.len() as f32;
            plane_world_centers.insert(*face_id, center);
        }
        FaceCenters(plane_world_centers)
    }

    pub fn get(&self, face_id: &FaceId) -> Option<&Vector3> {
        self.0.get(face_id)
    }
}

impl std::ops::Index<&FaceId> for FaceCenters {
    type Output = Vector3;

    fn index(&self, index: &FaceId) -> &Self::Output {
        &self.0[index]
    }
}
