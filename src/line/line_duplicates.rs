use std::collections::BTreeSet;

use super::{LineId, Lines};

use crate::{
    face::{FaceDuplicates, FaceId, FaceLines, FaceVertices},
    BrushFaces, EntityBrushes, Vector3, EPSILON,
};

#[derive(Debug, Clone)]
pub struct LineDuplicates(BTreeSet<(LineId, LineId)>);

impl LineDuplicates {
    pub fn new(
        lines: &Lines,
        entity_brushes: &EntityBrushes,
        brush_faces: &BrushFaces,
        face_duplicates: &FaceDuplicates,
        face_vertices: &FaceVertices,
        face_lines: &FaceLines,
    ) -> Self {
        let mut line_duplicates = BTreeSet::<(LineId, LineId)>::default();

        // For each entity's set of brushes
        for (_, brushes) in entity_brushes {
            // Iterate over the entity's brushes
            for brush_a in brushes {
                let faces_a = &brush_faces[brush_a];

                // Iterate over LHS brush faces
                for face_a in faces_a.iter().filter(|f| !face_duplicates.contains(f)) {
                    // Fetch LHS vertex and line data
                    let verts_a = face_vertices.vertices(&face_a).unwrap();
                    let lines_a = face_lines.get(&face_a).unwrap();

                    // Calculate whether this is an interior face
                    let mut connected_lines = 0;

                    // Iterate over LHS face lines
                    for line_id_a in lines_a.iter() {
                        // Fetch LHS line indices
                        let line_a = lines[line_id_a];

                        // Fetch LHS line vertices
                        let v0_a = verts_a[line_a.i0];
                        let v1_a = verts_a[line_a.i1];

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
                                let lines_b = face_lines.get(&face_b).unwrap();

                                // Iterate over RHS face lines
                                for line_id_b in lines_b.iter() {
                                    // Fetch RHS line indices
                                    let line_b = lines[line_id_b];

                                    // Fetch RHS line vertices
                                    let v0_b = verts_b[line_b.i0];
                                    let v1_b = verts_b[line_b.i1];

                                    // If the lines are equivalent, this line is connected
                                    if line_eq(v0_a, v1_a, v0_b, v1_b) {
                                        line_duplicates.insert((*line_id_a, *line_id_b));
                                        line_duplicates.insert((*line_id_b, *line_id_a));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        LineDuplicates(line_duplicates)
    }

    pub fn contains(&self, line_id: &LineId) -> bool {
        self.0.iter().any(|(a, b)| a == line_id || b == line_id)
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
