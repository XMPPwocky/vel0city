use na::{
    self,
    Rotation
};
use glutin::{
    self,
    VirtualKeyCode
};
use settings::InputSettings;
use player::movement::MoveInput;

bitflags! {
    #[derive(Debug)]
    flags Buttons: u32 {
        const BUTTON_FORWARD = 0b00_00_00_01,
        const BUTTON_BACK    = 0b00_00_00_10,
        const BUTTON_LEFT    = 0b00_00_01_00,
        const BUTTON_RIGHT   = 0b00_00_10_00,
        const BUTTON_JUMP    = 0b00_01_00_00
    }
}

pub struct Input {
    ang: na::Vec3<f32>,
    buttons: Buttons,
    cursorpos: (i32, i32),

    pub settings: InputSettings
}
impl Input {
    pub fn new() -> Input {
        use glutin::VirtualKeyCode::*;

        Input {
            ang: na::zero(),
            buttons: Buttons::empty(),
            cursorpos: (0, 0),
            settings: InputSettings {
                sensitivity: 0.01,
                forwardkey: F,
                backkey: N,
                leftkey: Y,
                rightkey: E,
                jumpkey: Space
            }
        }
    }
    pub fn handle_event(&mut self,
                        event: &glutin::Event) {
        use glutin::Event::{
            MouseMoved,
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
                use std::f32::consts::{
                    PI_2,
                    FRAC_PI_2
                };

                let (x, y) = (absx - self.cursorpos.0, absy - self.cursorpos.1);
                self.cursorpos = (absx, absy);

                self.ang.y += x as f32 * self.settings.sensitivity;
                self.ang.x += y as f32 * self.settings.sensitivity;

                self.ang.y = (self.ang.y + PI_2) % PI_2;
                self.ang.x = na::clamp(self.ang.x, -FRAC_PI_2, FRAC_PI_2);
            },
            _ => ()
        }
    }
    pub fn make_moveinput(&self, movesettings: &::settings::MoveSettings) -> MoveInput {
        let mut wvel: na::Vec3<f32> = na::zero();
        if self.buttons.contains(BUTTON_FORWARD) {
            wvel.z += movesettings.movespeed;
        }
        if self.buttons.contains(BUTTON_BACK) {
            wvel.z -= movesettings.movespeed;
        }
        if self.buttons.contains(BUTTON_LEFT) {
            wvel.x += movesettings.movespeed;
        }
        if self.buttons.contains(BUTTON_RIGHT) {
            wvel.x -= movesettings.movespeed;
        }
        let jump = self.buttons.contains(BUTTON_JUMP);

        let wvel = na::rotate(
            &na::Rot3::new(na::Vec3::new(0.0, self.ang.y, 0.0)),
            &wvel);

        MoveInput {
            wishvel: wvel,
            jump: jump,
        }
    }
    pub fn get_ang(&self) -> na::UnitQuat<f32> {
        let rot = na::UnitQuat::new(na::Vec3::new(0.0, self.ang.y, 0.0));
        rot.append_rotation(
            &na::Vec3::new(self.ang.x, 0.0, 0.0)
            )
    }
}
