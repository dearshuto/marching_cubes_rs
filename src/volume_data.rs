use crate::IVolumeData;
use num_traits::Unsigned;
use std::marker::Copy;

pub struct VolumeData<T: Unsigned + std::marker::Copy, const N: usize>
where
    [T; N * N * N]: Sized,
{
    _data: [T; N * N * N],
    _marker: std::marker::PhantomData<T>,
}

impl<T: Unsigned + std::marker::Copy, const N: usize> VolumeData<T, N>
where
    [T; N * N * N]: Sized,
{
    pub fn new() -> Self {
        Self {
            _data: [T::zero(); N * N * N],
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get(&self, x: u32, y: u32, z: u32) -> T {
        let index = self.to_index(x, y, z);
        self._data[index]
    }

    pub fn set(&mut self, x: u32, y: u32, z: u32, value: T) {
        let index = self.to_index(x, y, z);
        self._data[index] = value;
    }

    pub fn get_width(&self) -> usize {
        4
    }

    pub fn get_height(&self) -> usize {
        4
    }

    fn to_index(&self, x: u32, y: u32, z: u32) -> usize {
        (x as usize) + N * (y as usize) + N * N * (z as usize)
    }
}

impl<T: Unsigned + Copy, const TRESOLUTION: usize> IVolumeData<T, TRESOLUTION>
    for VolumeData<T, TRESOLUTION>
where
    [T; TRESOLUTION * TRESOLUTION * TRESOLUTION]: Sized,
{
    fn get(&self, x: u32, y: u32, z: u32) -> T {
        let index = self.to_index(x, y, z);
        self._data[index]
    }

    fn set(&mut self, x: u32, y: u32, z: u32, value: T) {
        let index = self.to_index(x, y, z);
        self._data[index] = value;
    }
}
