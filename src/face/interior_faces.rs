use std::collections::BTreeSet;

use super::{FaceDuplicates, FaceId, FaceNormals, FaceVertices};
use crate::{line::Lines, BrushFaces, EntityBrushes, Vector3, EPSILON};

#[derive(Debug, Clone)]
pub struct InteriorFaces(BTreeSet<FaceId>);

impl InteriorFaces {
    pub fn new(
        entity_brushes: &EntityBrushes,
        brush_faces: &BrushFaces,
        face_duplicates: &FaceDuplicates,
        face_vertices: &FaceVertices,
        face_line_indices: &Lines,
    ) -> Self {
        let line_inds = &face_line_indices.line_indices;

        let mut interior_faces = BTreeSet::default();

        // For each entity's set of brushes
        for (_, brushes) in entity_brushes {
            // Iterate over the entity's brushes
            for brush_a in brushes {
                let faces_a = &brush_faces[brush_a];

                // Iterate over LHS brush faces
                for face_a in faces_a.iter().filter(|f| !face_duplicates.contains(f)) {
                    // Fetch LHS vertex and line data
                    let verts_a = face_vertices.vertices(&face_a).unwrap();
                    let lines_a = face_line_indices.face_lines.get(&face_a).unwrap();

                    // Calculate whether this is an interior face
                    let mut connected_lines = 0;

                    // Iterate over LHS face lines
                    'line_a: for line_id_a in lines_a.iter() {
                        // Fetch LHS line indices
                        let line_a = line_inds[line_id_a];

                        // Fetch LHS line vertices
                        let v0_a = verts_a[line_a.v0];
                        let v1_a = verts_a[line_a.v1];

                        // Iterate over the entity's brushes again to compare
                        for brush_b in brushes {
                            // Skip comparing with self
                            if brush_a == brush_b {
                                continue;
                            }

                            // Fetch each brush's faces
                            let faces_b = &brush_faces[brush_b];
                            // Iterate over RHS brush faces
                            for face_b in faces_b.iter().filter(|f| !face_duplicates.contains(f)) {
                                // Fetch RHS vertex and line data
                                let verts_b = face_vertices.vertices(&face_b).unwrap();
                                let lines_b = face_line_indices.face_lines.get(&face_b).unwrap();

                                // Iterate over RHS face lines
                                for line_id_b in lines_b.iter() {
                                    // Fetch RHS line indices
                                    let line_b = line_inds[line_id_b];

                                    // Fetch RHS line vertices
                                    let v0_b = verts_b[line_b.v0];
                                    let v1_b = verts_b[line_b.v1];

                                    // If the lines are equivalent, this line is connected
                                    if line_eq(v0_a, v1_a, v0_b, v1_b) {
                                        connected_lines += 1;
                                        continue 'line_a;
                                    }
                                }
                            }
                        }
                    }

                    // If all of this face's lines are connected, add it to the interior set
                    if connected_lines >= lines_a.len() {
                        interior_faces.insert(*face_a);
                    }
                }
            }
        }

        InteriorFaces(interior_faces)
    }

    pub fn contains(&self, face_id: &FaceId) -> bool {
        self.0.contains(face_id)
    }
}

fn line_eq(a0: Vector3, a1: Vector3, b0: Vector3, b1: Vector3) -> bool {
    if (a0 - b0).magnitude() < EPSILON && (a1 - b1).magnitude() < EPSILON {
        true
    } else if (a0 - b1).magnitude() < EPSILON && (a1 - b0).magnitude() < EPSILON {
        true
    } else {
        false
    }
}
