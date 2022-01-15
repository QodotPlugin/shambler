use std::collections::BTreeMap;

use crate::{brush::BrushId, BrushFaces};

use super::FaceId;

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FaceBrushes(pub BTreeMap<FaceId, BrushId>);

impl FaceBrushes {
    pub fn new(brush_faces: &BrushFaces) -> Self {
        FaceBrushes(
            brush_faces
                .iter()
                .flat_map(|(brush, faces)| faces.iter().map(move |face| (*face, *brush)))
                .collect(),
        )
    }
}
