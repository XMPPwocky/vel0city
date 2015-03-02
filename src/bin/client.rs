#![feature(io, fs)]

extern crate glium;
extern crate glutin;
extern crate vel0city;
extern crate wavefront_obj;
extern crate time;
extern crate "nalgebra" as na;

use std::io::Read;
use std::sync::Arc;
use glium::DisplayBuild;
use glium::Surface;

pub struct Client {
    playermodel: vel0city::graphics::Model
}
impl Client {
    fn new(display: &glium::Display) -> Client {
        let tex = vec![
            vec![(0u8, 0u8, 0u8), (0u8, 255u8, 0u8)],
            vec![(255u8, 0u8, 0u8), (255u8, 255u8, 0u8)]
        ];
        let tex = glium::Texture2d::new(display, tex);
        let program = glium::Program::from_source(&display,
        // vertex shader
        "
            #version 110
            uniform mat4 transform;
            attribute vec3 position;
            attribute vec2 texcoords;
            varying vec2 v_texcoords;
            void main() {
                gl_Position = transform * vec4(position, 1.0); 
                v_texcoords = texcoords;
            }
        ",

        // fragment shader
        "
            #version 110
            uniform sampler2D color;
            varying vec2 v_texcoords;
            void main() {
                gl_FragColor = texture2D(color, v_texcoords);
            }
        ",

        // geometry shader
        None)
        .unwrap();
        let mut playerobjfile = std::fs::File::open("assets/player.obj").unwrap();
        let mut s = String::new();
        playerobjfile.read_to_string(&mut s).unwrap();
        let playerobj = &wavefront_obj::obj::parse(s).unwrap().objects[0];

        let playermodel = vel0city::graphics::wavefront::obj_to_model(playerobj,
                                                                      Arc::new(program),
                                                                      tex,
                                                                      display);

        Client {
            playermodel: playermodel,
        }
    }
}

fn main() {
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();
    let client = Client::new(&display);
    let view = vel0city::graphics::View {
        w2s: na::Persp3::new(1024.0/768.0, 90.0, 0.1, 4096.0).to_mat(),
        drawparams: std::default::Default::default(),
    };

    let mut game = vel0city::Game {
        settings: std::default::Default::default(),
        players: vec![vel0city::player::Player {
            pos: na::Pnt3::new(0.0, 10.0, 5.),
            eyeheight: 0.0,
            eyeang: na::UnitQuat::new_with_euler_angles(0.,0.,0.,),
            halfextents: vel0city::player::PLAYER_HALFEXTENTS,
            vel: na::zero()
        }],
        map: vel0city::map::single_plane_map()
    };
    game.settings.gravity = 9.8;
    
    let mut lasttime = time::precise_time_ns(); 
    loop {
        let now = time::precise_time_ns();
        let frametime_ns = now - lasttime;
        lasttime = now;

        vel0city::player::movement::move_player(&mut game, 0, &vel0city::player::movement::MoveInput { wishvel: na::Vec3::new(0.0, -1.0, 0.0) }, frametime_ns as f32 / 1.0E9);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        vel0city::graphics::draw_view(&game,
                                      &view,
                                      &client.playermodel,
                                      &mut target);
        target.finish();
    }
        
}
