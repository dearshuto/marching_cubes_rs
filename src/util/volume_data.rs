use num_traits::Unsigned;

use crate::IVolumeData;

pub struct VolumeData<T, const TRESOLUTION: usize>
where
    T: Unsigned + Copy,
{
    internal: Vec<T>,
}

impl<T, const TRESOLUTION: usize> VolumeData<T, TRESOLUTION>
where
    T: Unsigned + Copy,
{
    pub fn new() -> Self {
        let size = TRESOLUTION * TRESOLUTION * TRESOLUTION;
        let mut internal = Vec::default();
        internal.resize(size, T::zero());
        Self { internal }
    }
}

impl<T: Unsigned + Copy, const TRESOLUTION: usize> IVolumeData<T, TRESOLUTION>
    for VolumeData<T, TRESOLUTION>
{
    fn get(&self, x: u32, y: u32, z: u32) -> T {
        let index =
            (x as usize) + TRESOLUTION * (y as usize) + TRESOLUTION * TRESOLUTION * (z as usize);
        self.internal[index]
    }

    fn set(&mut self, x: u32, y: u32, z: u32, value: T) {
        let index =
            (x as usize) + TRESOLUTION * (y as usize) + TRESOLUTION * TRESOLUTION * (z as usize);
        self.internal[index] = value;
    }
}
