use std::path::Path;

use num_traits::{Float, Unsigned};

use crate::mesh_builder::IMeshData;

pub struct WavefrontObjMeshData {
    obj_set: obj_exporter::ObjSet,
}

impl WavefrontObjMeshData {
    pub fn new() -> Self {
        Self {
            obj_set: obj_exporter::ObjSet {
                material_library: None,
                objects: vec![obj_exporter::Object {
                    name: String::from("obj"),
                    vertices: Vec::default(),
                    tex_vertices: Vec::default(),
                    normals: Vec::default(),
                    geometry: Vec::default(),
                }],
            },
        }
    }

    pub fn export_to_file<TPath>(&self, path: TPath)
    where
        TPath: AsRef<Path>,
    {
        obj_exporter::export_to_file(&self.obj_set, path).unwrap();
    }
}

impl<F, U> IMeshData<F, U> for WavefrontObjMeshData
where
    F: Float,
    U: Unsigned + Into<usize>,
{
    fn push_vertex(&mut self, x: F, y: F, z: F) {
        let x: f64 = num_traits::cast(x).unwrap();
        let y: f64 = num_traits::cast(y).unwrap();
        let z: f64 = num_traits::cast(z).unwrap();
        let vertex = obj_exporter::Vertex { x, y, z };
        self.obj_set.objects[0].vertices.push(vertex);
    }

    fn push_triangle(&mut self, x: U, y: U, z: U) {
        let x: usize = x.into();
        let y: usize = y.into();
        let z: usize = z.into();

        self.obj_set.objects[0]
            .geometry
            .push(obj_exporter::Geometry {
                material_name: None,
                shapes: vec![obj_exporter::Shape {
                    primitive: obj_exporter::Primitive::Triangle(
                        (x, Some(x), Some(0)),
                        (y, Some(y), Some(0)),
                        (z, Some(z), Some(0)),
                    ),
                    groups: Vec::default(),
                    smoothing_groups: Vec::default(),
                }],
            });
    }
}
