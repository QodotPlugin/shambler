use std::collections::BTreeMap;

use super::{FaceBases, FaceId, FacePlanes, FaceVertices};
use crate::{line::Lines, EPSILON};

#[derive(Debug, Clone)]
pub struct FaceFaceContainment(BTreeMap<FaceId, Vec<FaceId>>);

impl FaceFaceContainment {
    // Find contained faces
    pub fn new(
        faces: &Vec<FaceId>,
        face_planes: &FacePlanes,
        face_bases: &FaceBases,
        face_vertices: &FaceVertices,
        lines: &Lines,
    ) -> Self {
        let mut contained_faces = BTreeMap::<FaceId, Vec<FaceId>>::default();
        for lhs_id in faces {
            let lhs_verts = face_vertices.vertices(&lhs_id).unwrap();
            let lhs_plane = &face_planes[&lhs_id];
            let lhs_basis = &face_bases[&lhs_id];

            for rhs_id in faces {
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

                let mut contained = true;
                'lines: for line_id in &lines.face_lines[lhs_id] {
                    let line = lines.line_indices[line_id];

                    let v0 = lhs_verts[line.v0];
                    let v1 = lhs_verts[line.v1];

                    let vd0 = nalgebra::vector![v0.dot(&lhs_basis.x), v0.dot(&lhs_basis.y)];
                    let vd1 = nalgebra::vector![v1.dot(&lhs_basis.x), v1.dot(&lhs_basis.y)];

                    let u = (vd1 - vd0).normalize();
                    let v = nalgebra::vector![-u.y, u.x];

                    for vert in rhs_verts {
                        let vert =
                            nalgebra::vector![vert.dot(&lhs_basis.x), vert.dot(&lhs_basis.y)];
                        if vert.dot(&v) > vd0.dot(&v) + EPSILON {
                            contained = false;
                            break 'lines;
                        }
                    }
                }

                if !contained {
                    continue;
                }

                contained_faces.entry(*lhs_id).or_default().push(*rhs_id);
            }
        }
        FaceFaceContainment(contained_faces)
    }

    pub fn get_contained_faces(&self, face_id: &FaceId) -> Option<&Vec<FaceId>> {
        self.0.get(face_id)
    }

    pub fn is_contained(&self, face_id: &FaceId) -> bool {
        self.0.values().flatten().any(|id| id == face_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FaceId, &Vec<FaceId>)> {
        self.0.iter()
    }
}
