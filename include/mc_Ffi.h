#include <cstdint>

extern "C" void* create_volume_data_8();
extern "C" void destroy_volume_data_8(void* ptr);

extern "C" void* create_volume_data_16();
extern "C" void destroy_volume_data_16(void* ptr);

extern "C" void* create_volume_data_32();
extern "C" void destroy_volume_data_32(void* ptr);

extern "C" void* create_volume_data_64();
extern "C" void destroy_volume_data_64(void* ptr);

extern "C" void* create_volume_data_128();
extern "C" void destroy_volume_data_128(void* ptr);

extern "C" void* create_mesh_data();
extern "C" void destroy_mesh_data(void* ptr);

extern "C" void write_volue_data_8(void* ptr, uint32_t x, uint32_t y, uint32_t z, uint8_t value);
extern "C" void write_volue_data_16(void* ptr, uint32_t x, uint32_t y, uint32_t z, uint8_t value);
extern "C" void write_volue_data_32(void* ptr, uint32_t x, uint32_t y, uint32_t z, uint8_t value);
extern "C" void write_volue_data_64(void* ptr, uint32_t x, uint32_t y, uint32_t z, uint8_t value);
extern "C" void write_volue_data_128(void* ptr, uint32_t x, uint32_t y, uint32_t z, uint8_t value);

extern "C" void build_8_with_marching_cubes(void* mesh_data_ptr, const void* volume_data_ptr, float threshold);
extern "C" void build_16_with_marching_cubes(void* mesh_data_ptr, const void* volume_data_ptr, float threshold);
extern "C" void build_32_with_marching_cubes(void* mesh_data_ptr, const void* volume_data_ptr, float threshold);
extern "C" void build_64_with_marching_cubes(void* mesh_data_ptr, const void* volume_data_ptr, float threshold);
extern "C" void build_128_with_marching_cubes(void* mesh_data_ptr, const void* volume_data_ptr, float threshold);
