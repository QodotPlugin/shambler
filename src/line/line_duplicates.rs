use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::BTreeSet;
use usage::Usage;

use super::{LineId, Lines};

use crate::{
    face::{FaceDuplicates, FaceLines, FaceVertices},
    BrushFaces, Brushes, Vector3, EPSILON,
};

pub enum LineDuplicatesTag {}
pub type LineDuplicates = Usage<LineDuplicatesTag, BTreeSet<(LineId, LineId)>>;

pub fn line_duplicates(
    brushes: &Brushes,
    lines: &Lines,
    brush_faces: &BrushFaces,
    face_duplicates: &FaceDuplicates,
    face_vertices: &FaceVertices,
    face_lines: &FaceLines,
) -> LineDuplicates {
    brushes
        .par_iter()
        .flat_map(|brush_a| {
            let faces_a = &brush_faces[brush_a];

            // Iterate over LHS brush faces
            faces_a
                .par_iter()
                .filter(|f| !face_duplicates.iter().any(|(a, _)| a == *f))
                .flat_map(move |face_a| {
                    // Fetch LHS vertex and line data
                    let verts_a = &face_vertices[&face_a];
                    let lines_a = &face_lines[&face_a];

                    // Iterate over LHS face lines
                    lines_a.par_iter().flat_map(move |line_id_a| {
                        // Fetch LHS line indices
                        let line_a = lines[line_id_a];

                        // Fetch LHS line vertices
                        let v0_a = verts_a[line_a.i0];
                        let v1_a = verts_a[line_a.i1];

                        // Iterate over brushes again to compare
                        brushes
                            .par_iter()
                            .flat_map(|brush_b| {
                                // Skip comparing with self
                                if brush_a == brush_b {
                                    return None;
                                }

                                // Fetch each brush's faces
                                let faces_b = &brush_faces[brush_b];

                                // Iterate over RHS brush faces
                                Some(
                                    faces_b
                                        .par_iter()
                                        .filter(|f| !face_duplicates.iter().any(|(a, _)| a == *f))
                                        .flat_map(|face_b| {
                                            // Fetch RHS vertex and line data
                                            let verts_b = &face_vertices[face_b];
                                            let lines_b = &face_lines[face_b];

                                            // Iterate over RHS face lines
                                            lines_b
                                                .par_iter()
                                                .flat_map(move |line_id_b| {
                                                    // Fetch RHS line indices
                                                    let line_b = lines[line_id_b];

                                                    // Fetch RHS line vertices
                                                    let v0_b = verts_b[line_b.i0];
                                                    let v1_b = verts_b[line_b.i1];

                                                    // If the lines are equivalent, add them to the set
                                                    if line_eq(v0_a, v1_a, v0_b, v1_b) {
                                                        Some([
                                                            (*line_id_a, *line_id_b),
                                                            (*line_id_b, *line_id_a),
                                                        ])
                                                    } else {
                                                        None
                                                    }
                                                })
                                                .flatten()
                                        }),
                                )
                            })
                            .flatten()
                            .collect::<BTreeSet<_>>()
                    })
                })
        })
        .collect()
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
