use std::collections::BTreeMap;

use crate::line::{Line, LineId};

use super::{FaceId, FaceIndices};

#[derive(Debug, Clone)]
pub struct FaceLines(BTreeMap<FaceId, BTreeMap<LineId, Line>>);

impl FaceLines {
    /// Generate line indices
    pub fn new(face_indices: &FaceIndices) -> Self {
        let mut line_head = 0;

        let mut plane_line_indices = BTreeMap::<FaceId, BTreeMap<LineId, Line>>::default();
        for (plane_id, indices) in face_indices {
            if indices.len() < 2 {
                continue;
            }

            for i in 0..indices.len() - 1 {
                plane_line_indices
                    .entry(*plane_id)
                    .or_default()
                    .insert(LineId(line_head), Line(indices[i], indices[i + 1]));

                line_head += 1;
            }

            plane_line_indices.entry(*plane_id).or_default().insert(
                LineId(line_head),
                Line(indices[indices.len() - 1], indices[0]),
            );
            line_head += 1;
        }
        FaceLines(plane_line_indices)
    }

    pub fn get(&self, face_id: &FaceId) -> Option<&BTreeMap<LineId, Line>> {
        self.0.get(face_id)
    }
}

impl std::ops::Index<&FaceId> for FaceLines {
    type Output = BTreeMap<LineId, Line>;

    fn index(&self, index: &FaceId) -> &Self::Output {
        &self.0[index]
    }
}
