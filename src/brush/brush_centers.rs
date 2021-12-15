use std::collections::BTreeMap;

use super::BrushId;
use crate::{
    face::{FaceCenters, FaceId},
    Vector3,
};

#[derive(Debug, Clone)]
pub struct BrushCenters(BTreeMap<BrushId, Vector3>);

impl BrushCenters {
    // Calculate brush centers
    pub fn new(
        brush_planes: &BTreeMap<BrushId, Vec<FaceId>>,
        face_centers: &FaceCenters,
    ) -> Self {
        let mut brush_centers = BTreeMap::<BrushId, Vector3>::default();
        for (brush_id, plane_ids) in brush_planes {
            let mut center = Vector3::zeros();

            for plane_id in plane_ids {
                center += face_centers[plane_id];
            }
            center /= plane_ids.len() as f32;

            brush_centers.insert(*brush_id, center);
        }
        BrushCenters(brush_centers)
    }

    pub fn get(&self, brush_id: &BrushId) -> Option<&Vector3> {
        self.0.get(brush_id)
    }
}

impl std::ops::Index<&BrushId> for BrushCenters {
    type Output = Vector3;

    fn index(&self, index: &BrushId) -> &Self::Output {
        &self.0[index]
    }
}
