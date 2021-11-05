use crate::{IMeshBuilder, MarchingCubesMeshBuilder, MeshDataSimple, VolumeData};

#[no_mangle]
pub extern "C" fn create_volume_data_8() -> *mut VolumeData<u8, 8> {
    Box::into_raw(Box::new(VolumeData::new()))
}

#[no_mangle]
pub extern "C" fn destroy_volume_data_8(ptr: *mut VolumeData<u8, 8>) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn create_volume_data_16() -> *mut VolumeData<u8, 16> {
    Box::into_raw(Box::new(VolumeData::new()))
}

#[no_mangle]
pub extern "C" fn destroy_volume_data_16(ptr: *mut VolumeData<u8, 16>) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn create_volume_data_32() -> *mut VolumeData<u8, 32> {
    Box::into_raw(Box::new(VolumeData::new()))
}

#[no_mangle]
pub extern "C" fn destroy_volume_data_32(ptr: *mut VolumeData<u8, 32>) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn create_volume_data_64() -> *mut VolumeData<u8, 64> {
    Box::into_raw(Box::new(VolumeData::new()))
}

#[no_mangle]
pub extern "C" fn destroy_volume_data_64(ptr: *mut VolumeData<u8, 64>) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn create_volume_data_128() -> *mut VolumeData<u8, 128> {
    Box::into_raw(Box::new(VolumeData::new()))
}

#[no_mangle]
pub extern "C" fn destroy_volume_data_128(ptr: *mut VolumeData<u8, 128>) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn write_volume_data_8(
    ptr: *mut VolumeData<u8, 8>,
    x: u32,
    y: u32,
    z: u32,
    value: u8,
) {
    unsafe { (*ptr).set(x, y, z, value) };
}

#[no_mangle]
pub extern "C" fn write_volume_data_16(
    ptr: *mut VolumeData<u8, 16>,
    x: u32,
    y: u32,
    z: u32,
    value: u8,
) {
    unsafe { (*ptr).set(x, y, z, value) };
}

#[no_mangle]
pub extern "C" fn write_volume_data_32(
    ptr: *mut VolumeData<u8, 32>,
    x: u32,
    y: u32,
    z: u32,
    value: u8,
) {
    unsafe { (*ptr).set(x, y, z, value) };
}

#[no_mangle]
pub extern "C" fn write_volume_data_64(
    ptr: *mut VolumeData<u8, 64>,
    x: u32,
    y: u32,
    z: u32,
    value: u8,
) {
    unsafe { (*ptr).set(x, y, z, value) };
}

#[no_mangle]
pub extern "C" fn write_volume_data_128(
    ptr: *mut VolumeData<u8, 128>,
    x: u32,
    y: u32,
    z: u32,
    value: u8,
) {
    unsafe { (*ptr).set(x, y, z, value) };
}

#[no_mangle]
pub extern "C" fn create_mesh_data() -> *mut MeshDataSimple<f32, u32> {
    Box::into_raw(Box::new(MeshDataSimple::new()))
}

#[no_mangle]
pub extern "C" fn destroy_mesh_data(ptr: *mut MeshDataSimple<f32, u32>) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn build_8_with_marching_cubes(
    mesh_data: *mut MeshDataSimple<f32, u32>,
    volume_data: *const VolumeData<u8, 8>,
    threshold: f32,
) {
    unsafe {
        MarchingCubesMeshBuilder::new()
            .with_threshold(threshold)
            .build(&mut (*mesh_data), &(*volume_data));
    }
}

#[no_mangle]
pub extern "C" fn build_16_with_marching_cubes(
    mesh_data: *mut MeshDataSimple<f32, u32>,
    volume_data: *const VolumeData<u8, 16>,
    threshold: f32,
) {
    unsafe {
        MarchingCubesMeshBuilder::new()
            .with_threshold(threshold)
            .build(&mut (*mesh_data), &(*volume_data));
    }
}

#[no_mangle]
pub extern "C" fn build_32_with_marching_cubes(
    mesh_data: *mut MeshDataSimple<f32, u32>,
    volume_data: *const VolumeData<u8, 32>,
    threshold: f32,
) {
    unsafe {
        MarchingCubesMeshBuilder::new()
            .with_threshold(threshold)
            .build(&mut (*mesh_data), &(*volume_data));
    }
}

#[no_mangle]
pub extern "C" fn build_64_with_marching_cubes(
    mesh_data: *mut MeshDataSimple<f32, u32>,
    volume_data: *const VolumeData<u8, 64>,
    threshold: f32,
) {
    unsafe {
        MarchingCubesMeshBuilder::new()
            .with_threshold(threshold)
            .build(&mut (*mesh_data), &(*volume_data));
    }
}

#[no_mangle]
pub extern "C" fn build_128_with_marching_cubes(
    mesh_data: *mut MeshDataSimple<f32, u32>,
    volume_data: *const VolumeData<u8, 128>,
    threshold: f32,
) {
    unsafe {
        MarchingCubesMeshBuilder::new()
            .with_threshold(threshold)
            .build(&mut (*mesh_data), &(*volume_data));
    }
}

#[no_mangle]
pub extern "C" fn get_vertex_count(mesh_data: *const MeshDataSimple<f32, u32>) -> usize {
    unsafe { (*mesh_data).get_vertex_data().len() / 3 }
}

#[no_mangle]
pub extern "C" fn get_index_count(mesh_data: *const MeshDataSimple<f32, u32>) -> usize {
    unsafe { (*mesh_data).get_index_data().len() }
}

#[no_mangle]
pub extern "C" fn get_vertex(
    mesh_data: *const MeshDataSimple<f32, u32>,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
    index: usize,
) {
    let actual_index = 3 * index;
    let vertex_data = unsafe { (*mesh_data).get_vertex_data() };
    unsafe {
        *x = vertex_data[actual_index + 0];
        *y = vertex_data[actual_index + 1];
        *z = vertex_data[actual_index + 2];
    }
}

#[no_mangle]
pub extern "C" fn get_index(mesh_data: *const MeshDataSimple<f32, u32>, index: usize) -> u32 {
    unsafe { (*mesh_data).get_index_data()[index] }
}
