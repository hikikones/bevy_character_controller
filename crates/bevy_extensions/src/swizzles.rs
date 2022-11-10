use bevy::math::{IVec2, IVec3, Vec2, Vec3};

pub trait Vec2SwizzlesExt {
    type Vec3;
    type Number;

    fn x0z(self) -> Self::Vec3;
    fn x_z(self, y: Self::Number) -> Self::Vec3;
}

pub trait Vec3SwizzlesExt {
    type Vec2;
    type Number;

    fn xz(self) -> Self::Vec2;
    fn x0z(self) -> Self;
    fn x_z(self, y: Self::Number) -> Self;
}

impl Vec2SwizzlesExt for IVec2 {
    type Vec3 = IVec3;
    type Number = i32;

    fn x0z(self) -> Self::Vec3 {
        self.x_z(0)
    }

    fn x_z(self, y: Self::Number) -> Self::Vec3 {
        IVec3::new(self.x, y, self.y)
    }
}

impl Vec2SwizzlesExt for Vec2 {
    type Vec3 = Vec3;
    type Number = f32;

    fn x0z(self) -> Self::Vec3 {
        self.x_z(0.0)
    }

    fn x_z(self, y: Self::Number) -> Self::Vec3 {
        Vec3::new(self.x, y, self.y)
    }
}

impl Vec3SwizzlesExt for IVec3 {
    type Vec2 = IVec2;
    type Number = i32;

    fn xz(self) -> Self::Vec2 {
        IVec2::new(self.x, self.z)
    }

    fn x0z(self) -> Self {
        self.x_z(0)
    }

    fn x_z(self, y: Self::Number) -> Self {
        Self::new(self.x, y, self.z)
    }
}

impl Vec3SwizzlesExt for Vec3 {
    type Vec2 = Vec2;
    type Number = f32;

    fn xz(self) -> Self::Vec2 {
        Vec2::new(self.x, self.z)
    }

    fn x0z(self) -> Self {
        self.x_z(0.0)
    }

    fn x_z(self, y: Self::Number) -> Self {
        Self::new(self.x, y, self.z)
    }
}
