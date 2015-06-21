use na::{
    self,
    Rotation
};
use glutin;
use settings::InputSettings;
use player::movement::MoveInput;
use std::f32::consts::{
    PI_2,
    FRAC_PI_2
};

bitflags! {
    flags Buttons: u32 {
        const BUTTON_FORWARD = 0b00_00_00_01,
        const BUTTON_BACK    = 0b00_00_00_10,
        const BUTTON_LEFT    = 0b00_00_01_00,
        const BUTTON_RIGHT   = 0b00_00_10_00,
        const BUTTON_JUMP    = 0b00_01_00_00,
        const BUTTON_RESET   = 0b01_00_00_00,
        const BUTTON_SPECIAL = 0b10_00_00_00,
    }
}

fn wrap_yaw(yaw: f32) -> f32 {
    yaw % PI_2
}
fn clamp_pitch(pitch: f32) -> f32 {
    const MAXPITCH: f32 = FRAC_PI_2 - 0.05; 
    na::clamp(pitch % PI_2, -MAXPITCH, MAXPITCH)
}
pub struct Input {
    pitch: f32,
    yaw: f32,
    buttons: Buttons,
    pub cursorpos: (i32, i32),

    hack: bool,

    pub settings: InputSettings
}
impl Input {
    pub fn new() -> Input {
        use glutin::VirtualKeyCode::*;

        Input {
            pitch: 0.0,
            yaw: 0.0,
            buttons: Buttons::empty(),
            cursorpos: (400, 300),
            hack: false,
            settings: InputSettings {
                sensitivity: 0.0008,
                /*
                forwardkey: W,
                backkey: A,
                leftkey: S,
                rightkey: D,
                */
                forwardkey: F,
                backkey: N,
                leftkey: Y,
                rightkey: E,
                jumpkey: Space
            }
        }
    }
    pub fn handle_event(&mut self,
                        window: &glutin::Window,
                        event: &glutin::Event) {
        use glutin::Event::{
            MouseMoved,
            MouseInput,
            KeyboardInput
        };

        match event {
            &KeyboardInput(state, _, Some(vkcode)) => {
                let action = |b: &mut Buttons, o| {
                    match state {
                        glutin::ElementState::Pressed => b.insert(o), 
                        glutin::ElementState::Released => b.remove(o) 
                    }
                };

                if vkcode == self.settings.forwardkey { 
                    action(&mut self.buttons, BUTTON_FORWARD);
                }
                if vkcode == self.settings.backkey { 
                    action(&mut self.buttons, BUTTON_BACK);
                }
                if vkcode == self.settings.leftkey { 
                    action(&mut self.buttons, BUTTON_LEFT);
                }
                if vkcode == self.settings.rightkey { 
                    action(&mut self.buttons, BUTTON_RIGHT);
                }
                if vkcode == self.settings.jumpkey { 
                    action(&mut self.buttons, BUTTON_JUMP);
                }
            },
            &MouseMoved((absx, absy)) => {
                if !self.hack { 
                    let (x, y) = (absx - self.cursorpos.0, absy - self.cursorpos.1);
                    let _ = window.set_cursor_position(self.cursorpos.0, self.cursorpos.1);
                    self.yaw = wrap_yaw(
                        self.yaw + (x as f32 * self.settings.sensitivity)
                        );
                    self.pitch = clamp_pitch(
                        self.pitch + (y as f32 * self.settings.sensitivity)
                        );

                    self.hack = true;
                } else {
                    self.hack = false;
                }
            },
            &MouseInput(state, glutin::MouseButton::Left) => {
                if state == glutin::ElementState::Pressed {
                    self.buttons.insert(BUTTON_SPECIAL);
                } else {
                    self.buttons.remove(BUTTON_SPECIAL);
                }
            },
            _ => ()
        }
    }

    pub fn get_ang(&self) -> na::UnitQuat<f32> {
        na::UnitQuat::new(na::Vec3::new(0.0, self.yaw, 0.0))
            .append_rotation(&na::Vec3::new(self.pitch, 0.0, 0.0))
    }

    pub fn make_moveinput(&self, movesettings: &::settings::MoveSettings) -> MoveInput {
        let mut wvel: na::Vec3<f32> = na::zero();
        if self.buttons.contains(BUTTON_FORWARD) {
            wvel.z -= movesettings.movespeed;
        }
        if self.buttons.contains(BUTTON_BACK) {
            wvel.z += movesettings.movespeed;
        }
        if self.buttons.contains(BUTTON_LEFT) {
            wvel.x += movesettings.movespeed;
        }
        if self.buttons.contains(BUTTON_RIGHT) {
            wvel.x -= movesettings.movespeed;
        }
        let jump = self.buttons.contains(BUTTON_JUMP);
        let special = self.buttons.contains(BUTTON_SPECIAL);

        MoveInput {
            wishvel: wvel,
            eyeang: self.get_ang(),
            jump: jump,
            special: special,
        }
    }
}
