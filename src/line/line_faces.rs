use std::collections::BTreeMap;

use crate::face::{FaceId, FaceLines};

use super::LineId;

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LineFaces(pub BTreeMap<LineId, FaceId>);

impl LineFaces {
    pub fn new(face_lines: &FaceLines) -> Self {
        LineFaces(
            face_lines
                .iter()
                .flat_map(|(face, lines)| lines.iter().map(move |line| (*line, *face)))
                .collect(),
        )
    }
}
