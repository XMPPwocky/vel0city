use na;
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
    pub ang: na::UnitQuat<f32>,
    buttons: Buttons,

    pub settings: InputSettings
}
impl Input {
    pub fn new() -> Input {
        use glutin::VirtualKeyCode::*;

        Input {
            ang: na::UnitQuat::new_with_euler_angles(0.0, 0.0, 0.0),
            buttons: Buttons::empty(),
            settings: InputSettings {
                sensitivity: 1.0,
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
            //MouseMoved,
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
        let wvel = na::rotate(&self.ang, &wvel);
        MoveInput {
            wishvel: wvel
        }
    }
}
