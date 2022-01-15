use std::collections::BTreeMap;

use super::FaceId;
use crate::line::LineId;

pub type FaceLines = BTreeMap<FaceId, Vec<LineId>>;
