use std::collections::BTreeMap;

use crate::line::{LineId, LineIndices};

use crate::face::{FaceId, FaceIndices};

#[derive(Debug, Clone)]
pub struct Lines {
    pub lines: Vec<LineId>,
    pub face_lines: BTreeMap<FaceId, Vec<LineId>>,
    pub line_indices: BTreeMap<LineId, LineIndices>,
}

impl Lines {
    pub fn new(face_indices: &FaceIndices) -> Self {
        let mut line_head = 0;
        let mut lines = Vec::<LineId>::default();
        let mut face_lines = BTreeMap::<FaceId, Vec<LineId>>::default();
        let mut line_indices = BTreeMap::<LineId, LineIndices>::default();

        for (face_id, indices) in face_indices {
            if indices.len() < 2 {
                continue;
            }

            for i in 0..indices.len() - 1 {
                let line_id = LineId(line_head);
                line_head += 1;

                lines.push(line_id);
                face_lines.entry(*face_id).or_default().push(line_id);
                line_indices.insert(
                    line_id,
                    LineIndices {
                        v0: indices[i],
                        v1: indices[i + 1],
                    },
                );
            }

            let line_id = LineId(line_head);
            line_head += 1;

            lines.push(line_id);
            face_lines.entry(*face_id).or_default().push(line_id);
            line_indices.insert(
                line_id,
                LineIndices {
                    v0: indices[indices.len() - 1],
                    v1: indices[0],
                },
            );
        }

        Lines {
            lines,
            face_lines,
            line_indices,
        }
    }
}
