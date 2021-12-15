use std::collections::BTreeMap;

use crate::face::FaceId;

use super::FaceIndices;

#[derive(Debug, Clone)]
pub struct FaceTriangleIndices(BTreeMap<FaceId, Vec<usize>>);

impl FaceTriangleIndices {
    /// Generate triangle indices
    pub fn new(face_indices: &FaceIndices) -> Self {
        let mut plane_indices = BTreeMap::<FaceId, Vec<usize>>::default();
        for (plane_id, indices) in face_indices {
            if indices.len() < 3 {
                continue;
            }

            for i in 0..indices.len() - 2 {
                plane_indices
                    .entry(*plane_id)
                    .or_default()
                    .extend([indices[0], indices[i + 1], indices[i + 2]].iter().copied());
            }
        }
        FaceTriangleIndices(plane_indices)
    }

    pub fn get(&self, face_id: &FaceId) -> Option<&Vec<usize>> {
        self.0.get(face_id)
    }
}

impl std::ops::Index<&FaceId> for FaceTriangleIndices {
    type Output = Vec<usize>;

    fn index(&self, index: &FaceId) -> &Self::Output {
        &self.0[index]
    }
}
