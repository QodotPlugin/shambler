use std::collections::BTreeSet;

use super::{FaceId, FaceVertices};
use crate::{FacePlanes, EPSILON};

/// The set of opposing faces that share the same set of vertices
#[derive(Debug, Clone)]
pub struct FaceDuplicates(BTreeSet<(FaceId, FaceId)>);

impl FaceDuplicates {
    pub fn new(
        planes: &Vec<FaceId>,
        face_planes: &FacePlanes,
        face_vertices: &FaceVertices,
    ) -> Self {
        let mut duplicate_faces = BTreeSet::<(FaceId, FaceId)>::default();
        for lhs_id in planes {
            let lhs_verts = face_vertices.vertices(&lhs_id).unwrap();
            let lhs_plane = &face_planes[&lhs_id];

            for rhs_id in planes {
                let rhs_verts = face_vertices.vertices(&rhs_id).unwrap();
                let rhs_plane = &face_planes[&rhs_id];

                // Skip comparing with self
                if lhs_id == rhs_id {
                    continue;
                }

                // Skip faces that don't lie on the same plane
                if !lhs_plane.opposes(rhs_plane) {
                    continue;
                }

                // Skip comparing with faces of different vertex count
                if lhs_verts.len() != rhs_verts.len() {
                    continue;
                }

                let vert_count = lhs_verts.len();

                // Compare vertices
                let mut identical_count = 0;
                for lhs_vert in lhs_verts {
                    for rhs_vert in rhs_verts {
                        let delta = (lhs_vert - rhs_vert).magnitude();

                        if delta < EPSILON {
                            identical_count += 1;
                        }
                    }
                }

                if identical_count != vert_count {
                    continue;
                }

                // Add faces to set
                duplicate_faces.insert((*lhs_id, *rhs_id));
                duplicate_faces.insert((*rhs_id, *lhs_id));
            }
        }

        FaceDuplicates(duplicate_faces)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(FaceId, FaceId)> {
        self.0.iter()
    }

    pub fn contains(&self, face_id: &FaceId) -> bool {
        self.0.iter().find(|(a, b)| a == face_id || b == face_id).is_some()
    }
}
