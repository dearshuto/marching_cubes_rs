use mc::IMeshBuilder;

fn main() {
    let mut volume_data = mc::VolumeData::<u8, 5>::new();
    volume_data.set(1, 1, 1, u8::MAX);
    volume_data.set(1, 2, 1, u8::MAX);
    volume_data.set(1, 3, 2, u8::MAX);
    volume_data.set(2, 2, 1, u8::MAX);
    volume_data.set(2, 2, 2, u8::MAX);

    let mut mesh_data = mc::MeshDataSimple::<f64, u32>::new();
    mc::MarchingCubesMeshBuilder::new()
        .with_threshold(128.0)
        .build(&mut mesh_data, &volume_data);

    let set = obj_exporter::ObjSet {
        material_library: None,
        objects: vec![obj_exporter::Object {
            name: "UtahTeapot".to_owned(),
            vertices: (0..mesh_data.get_vertex_data().len() / 3)
                .into_iter()
                .map(|x| obj_exporter::Vertex {
                    x: mesh_data.get_vertex_data()[3 * x + 0],
                    y: mesh_data.get_vertex_data()[3 * x + 1],
                    z: mesh_data.get_vertex_data()[3 * x + 2],
                })
                .collect(),
            tex_vertices: vec![],
            normals: vec![],
            geometry: vec![obj_exporter::Geometry {
                material_name: None,
                shapes: (0..mesh_data.get_index_data().len() / 3)
                    .into_iter()
                    .map(|index| {
                        let i0 = mesh_data.get_index_data()[3 * index + 0] as usize;
                        let i1 = mesh_data.get_index_data()[3 * index + 1] as usize;
                        let i2 = mesh_data.get_index_data()[3 * index + 2] as usize;
                        obj_exporter::Shape {
                            primitive: obj_exporter::Primitive::Triangle(
                                (i0, Some(i0), Some(0)),
                                (i1, Some(i1), Some(0)),
                                (i2, Some(i2), Some(0)),
                            ),
                            groups: vec![],
                            smoothing_groups: vec![],
                        }
                    })
                    .collect(),
            }],
        }],
    };
    obj_exporter::export_to_file(&set, "volume_mesh.obj").unwrap();
}
