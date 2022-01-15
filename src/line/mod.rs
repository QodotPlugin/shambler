mod line_duplicates;
mod line_faces;
mod line_id;

pub use line_duplicates::*;
pub use line_faces::*;
pub use line_id::*;
use usage::Usage;

use std::collections::BTreeMap;

use crate::face::{FaceIndices, FaceLines};

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub i0: usize,
    pub i1: usize,
}

pub enum LinesTag {}
pub type Lines = Usage<LinesTag, BTreeMap<LineId, Line>>;

pub fn lines(face_indices: &FaceIndices) -> (Lines, FaceLines) {
    let mut line_head = 0;

    let mut face_lines = FaceLines::default();
    let mut line_indices = Lines::default();

    for (face_id, indices) in face_indices.iter() {
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
