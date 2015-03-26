use na;
use glutin::{
    self,
};
use settings::InputSettings;
use player::movement::MoveInput;
use std::f32::consts::{
    PI_2,
    FRAC_PI_2
};


bitflags! {
    #[derive(Debug)]
    flags Buttons: u32 {
        const BUTTON_FORWARD = 0b00_00_00_01,
        const BUTTON_BACK    = 0b00_00_00_10,
        const BUTTON_LEFT    = 0b00_00_01_00,
        const BUTTON_RIGHT   = 0b00_00_10_00,
        const BUTTON_JUMP    = 0b00_01_00_00,
        const BUTTON_RESET   = 0b00_10_00_00,
    }
}

pub struct Input {
    ang: na::Vec3<f32>,
    buttons: Buttons,
    pub cursorpos: (i32, i32),

    hack: bool,

    pub settings: InputSettings
}
impl Input {
    pub fn new() -> Input {
        use glutin::VirtualKeyCode::*;

        Input {
            ang: na::zero(),
            buttons: Buttons::empty(),
            cursorpos: (400, 300),
            hack: false,
            settings: InputSettings {
                sensitivity: 0.0033,
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
                resetkey: Escape,
                jumpkey: Space
            }
        }
    }
    pub fn handle_event(&mut self,
                        window: &glutin::Window,
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
                if vkcode == self.settings.resetkey { 
                    action(&mut self.buttons, BUTTON_RESET);
                }
            },
            &MouseMoved((absx, absy)) => {
                if !self.hack { 
                    let (x, y) = (absx - self.cursorpos.0, absy - self.cursorpos.1);
                    let _ = window.set_cursor_position(self.cursorpos.0, self.cursorpos.1);

                    self.ang.y += x as f32 * self.settings.sensitivity;
                    self.ang.x += y as f32 * self.settings.sensitivity;

                    self.ang.y = (self.ang.y + PI_2) % PI_2;
                    self.ang.x = na::clamp(self.ang.x, -FRAC_PI_2, FRAC_PI_2);

                    self.hack = true;
                } else {
                    self.hack = false;
                }
            },
            _ => ()
        }
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
        let reset = self.buttons.contains(BUTTON_RESET);

        let wvel = na::rotate(
            &na::Rot3::new(na::Vec3::new(0.0, self.ang.y, 0.0)),
            &wvel);

        MoveInput {
            wishvel: wvel,
            eyeang: self.ang,
            jump: jump,
            reset: reset,
        }
    }
    pub fn get_ang(&self) -> na::Vec3<f32> { 
        self.ang
    }
}
