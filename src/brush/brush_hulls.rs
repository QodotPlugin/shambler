use std::collections::BTreeMap;

use super::BrushId;
use crate::{ConvexHull, Plane3d, FacePlanes, face::FaceId};

#[derive(Debug, Clone)]
pub struct BrushHulls(BTreeMap<BrushId, ConvexHull>);

impl BrushHulls {
    pub fn new(
        brush_planes: &BTreeMap<BrushId, Vec<FaceId>>,
        geo_planes: &FacePlanes,
    ) -> Self {
        let mut brush_hulls = BTreeMap::<BrushId, ConvexHull>::default();
        for (brush_id, plane_ids) in brush_planes {
            let mut planes = vec![];
            for plane_id in plane_ids {
                let plane = &geo_planes[plane_id];
                planes.push(*plane);
            }
            brush_hulls.insert(*brush_id, planes.into());
        }
        BrushHulls(brush_hulls)
    }

    pub fn get(&self, brush_id: &BrushId) -> Option<&ConvexHull> {
        self.0.get(brush_id)
    }
}

impl std::ops::Index<&BrushId> for BrushHulls {
    type Output = ConvexHull;

    fn index(&self, index: &BrushId) -> &Self::Output {
        &self.0[index]
    }
}
