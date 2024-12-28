use mc::{IMeshBuilder, IVolumeData};

fn main() {
    let mut volume_data = mc::util::VolumeData::<u8, 5>::new();
    volume_data.set(1, 1, 1, u8::MAX);
    volume_data.set(1, 2, 1, u8::MAX);
    volume_data.set(1, 3, 2, u8::MAX);
    volume_data.set(2, 2, 1, u8::MAX);
    volume_data.set(2, 2, 2, u8::MAX);

    let mut mesh_data = mc::util::WavefrontObjMeshData::new();
    mc::MarchingCubesMeshBuilder::new()
        .with_threshold(128.0)
        .build(&mut mesh_data, &volume_data);

    mesh_data.export_to_file("volume_mesh.obj");
}
