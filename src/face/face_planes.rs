use std::collections::BTreeMap;

use shalrath::repr::Triangle;

use crate::Plane3d;

use super::FaceId;

#[derive(Debug, Clone)]
pub struct FacePlanes(BTreeMap<FaceId, Plane3d>);

impl FacePlanes {
    pub fn new(face_planes: &BTreeMap<FaceId, Triangle>) -> Self {
        FacePlanes(
            face_planes
                .iter()
                .map(|(plane_id, face_plane)| (*plane_id, Plane3d::from(face_plane)))
                .collect::<BTreeMap<_, _>>(),
        )
    }

    pub fn get(&self, face_id: &FaceId) -> Option<&Plane3d> {
        self.0.get(face_id)
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl std::ops::Index<&FaceId> for FacePlanes {
    type Output = Plane3d;

    fn index(&self, index: &FaceId) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> IntoIterator for &'a FacePlanes {
    type Item = (&'a FaceId, &'a Plane3d);

    type IntoIter = std::collections::btree_map::Iter<'a, FaceId, Plane3d>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for FacePlanes {
    type Item = (FaceId, Plane3d);

    type IntoIter = std::collections::btree_map::IntoIter<FaceId, Plane3d>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
