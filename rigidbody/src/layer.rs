use bitflags::bitflags;

use super::simulation::Group;

bitflags! {
    pub struct PhysicsLayer: u32 {
        const PLAYER      = 1 << 0;
        const PLATFORM    = 1 << 1;
    }
}

impl From<PhysicsLayer> for Group {
    fn from(layer: PhysicsLayer) -> Self {
        unsafe { Group::from_bits_unchecked(layer.bits) }
    }
}
