use bitflags::bitflags;

use super::simulation::*;

bitflags! {
    pub struct PhysicsLayer: u32 {
        const PLAYER      = 1 << 0;
        const PLATFORM    = 1 << 1;
    }
}

impl From<PhysicsLayer> for Group {
    fn from(layer: PhysicsLayer) -> Self {
        unsafe { Self::from_bits_unchecked(layer.bits) }
    }
}

impl From<PhysicsLayer> for CollisionGroups {
    fn from(layer: PhysicsLayer) -> Self {
        Self::new(layer.into(), Group::all())
    }
}

impl From<PhysicsLayer> for InteractionGroups {
    fn from(layer: PhysicsLayer) -> Self {
        Self::new(layer.bits.into(), layer.bits.into())
    }
}

impl From<PhysicsLayer> for QueryFilter<'_> {
    fn from(layer: PhysicsLayer) -> Self {
        Self {
            groups: Some(layer.into()),
            ..Default::default()
        }
    }
}
