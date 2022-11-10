use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputAction::default())
            .insert_resource(InputMovement::default())
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new().with_system(action).with_system(movement),
            );
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum InputAction {
    #[default]
    None,
    Jump,
}

fn action(keyboard: Res<Input<KeyCode>>, mut input_action: ResMut<InputAction>) {
    let mut action = InputAction::None;

    if keyboard.just_pressed(KeyCode::Space) {
        action = InputAction::Jump;
    }

    if *input_action != action {
        *input_action = action;
    }
}

#[derive(Default)]
pub struct InputMovement(Vec2);

impl InputMovement {
    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn xy(&self) -> Vec2 {
        self.0
    }

    pub fn x0z(&self) -> Vec3 {
        Vec3::new(self.0.x, 0.0, self.0.y)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == Vec2::ZERO
    }
}

fn movement(keyboard: Res<Input<KeyCode>>, mut input_movement: ResMut<InputMovement>) {
    let mut input = Vec2::ZERO;

    if keyboard.pressed(KeyCode::Left) {
        input.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::Right) {
        input.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Up) {
        input.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::Down) {
        input.y += 1.0;
    }

    input_movement.0 = input.normalize_or_zero();
}
