mod line_id;
mod line_indices;
mod line_faces;

pub use line_id::*;
pub use line_indices::*;
pub use line_faces::*;

use std::collections::BTreeMap;

use crate::face::{FaceId, FaceIndices, FaceLines};

pub type Lines = BTreeMap<LineId, Line>;

pub fn lines(face_indices: &FaceIndices) -> (Lines, FaceLines) {
    let mut line_head = 0;
    let mut face_lines = BTreeMap::<FaceId, Vec<LineId>>::default();
    let mut line_indices = BTreeMap::<LineId, Line>::default();

    for (face_id, indices) in face_indices {
        if indices.len() < 2 {
            continue;
        }

        for i in 0..indices.len() - 1 {
            let line_id = LineId(line_head);
            line_head += 1;

            line_indices.insert(
                line_id,
                Line {
                    i0: indices[i],
                    i1: indices[i + 1],
                },
            );
            face_lines.entry(*face_id).or_default().push(line_id);
        }

        let line_id = LineId(line_head);
        line_head += 1;

        line_indices.insert(
            line_id,
            Line {
                i0: indices[indices.len() - 1],
                i1: indices[0],
            },
        );
        face_lines.entry(*face_id).or_default().push(line_id);
    }

    (line_indices, face_lines)
}
