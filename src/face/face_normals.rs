use crate::Vector3;
use std::collections::BTreeMap;

use super::{FaceId, FacePlanes, FaceVertices};

#[derive(Debug, Clone)]
pub struct FaceNormals(BTreeMap<FaceId, Vec<Vector3>>);

impl FaceNormals {
    /// Copy normals from face planes
    pub fn flat(face_vertices: &FaceVertices, face_planes: &FacePlanes) -> Self {
        let mut face_normals = BTreeMap::<FaceId, Vec<Vector3>>::default();

        for (face_id, vertices) in face_vertices.iter_vertices() {
            let face_plane = &face_planes[face_id];

            face_normals.insert(
                *face_id,
                vertices.iter().map(|_| *face_plane.normal()).collect(),
            );
        }

        FaceNormals(face_normals)
    }

    /// Average normals from vertex planes with each plane contributing equally
    ///
    /// Good for spherical objects
    pub fn phong_averaged(face_vertices: &FaceVertices, face_planes: &FacePlanes) -> Self {
        let mut face_normals = BTreeMap::<FaceId, Vec<Vector3>>::default();

        for (face_id, vertex_planes) in face_vertices.iter_vertex_planes() {
            face_normals.insert(
                *face_id,
                vertex_planes
                    .iter()
                    .map(|(p0, p1, p2)| {
                        let p0 = &face_planes[p0];
                        let p1 = &face_planes[p1];
                        let p2 = &face_planes[p2];
                        (p0.normal() + p1.normal() + p2.normal()).normalize()
                    })
                    .collect(),
            );
        }

        FaceNormals(face_normals)
    }

    /// Average normals from vertex planes using an angular threshold given in degrees
    ///
    /// Good for cylindrical objects
    pub fn phong_threshold(
        face_vertices: &FaceVertices,
        face_planes: &FacePlanes,
        threshold: f32,
    ) -> Self {
        let mut face_normals = BTreeMap::<FaceId, Vec<Vector3>>::default();

        for (face_id, vertex_planes) in face_vertices.iter_vertex_planes() {
            face_normals.insert(
                *face_id,
                vertex_planes
                    .iter()
                    .map(|(p0, p1, p2)| {
                        let p0 = &face_planes[p0];
                        let p1 = &face_planes[p1];
                        let p2 = &face_planes[p2];

                        const ONE_DEGREE: f32 = 0.017_453_3;

                        let threshold = ((threshold + 0.01) * ONE_DEGREE).cos();
                        let mut normal = *p0.normal();
                        if p0.normal().dot(p1.normal()) > threshold {
                            normal += p1.normal()
                        }
                        if p0.normal().dot(p2.normal()) > threshold {
                            normal += p2.normal()
                        }
                        normal.normalize()
                    })
                    .collect(),
            );
        }

        FaceNormals(face_normals)
    }

    pub fn get(&self, face_id: &FaceId) -> Option<&Vec<Vector3>> {
        self.0.get(face_id)
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl std::ops::Index<&FaceId> for FaceNormals {
    type Output = Vec<Vector3>;

    fn index(&self, index: &FaceId) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> IntoIterator for &'a FaceNormals {
    type Item = (&'a FaceId, &'a Vec<Vector3>);

    type IntoIter = std::collections::btree_map::Iter<'a, FaceId, Vec<Vector3>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for FaceNormals {
    type Item = (FaceId, Vec<Vector3>);

    type IntoIter = std::collections::btree_map::IntoIter<FaceId, Vec<Vector3>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
