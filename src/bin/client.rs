extern crate glium;
extern crate glutin;
extern crate vel0city;
extern crate wavefront_obj;
extern crate "nalgebra" as na;
extern crate clock_ticks;

use std::sync::Arc;
use glium::DisplayBuild;
use glium::Surface;

use vel0city::assets;
use na::{
    ToHomogeneous,
    Inv
};

pub struct Client {
    playermodel: vel0city::graphics::Model,
    input: vel0city::input::Input,
}
impl Client {
    fn new(display: &glium::Display) -> Client {
        let tex = vec![
            vec![(0u8, 0u8, 0u8), (0u8, 255u8, 0u8)],
            vec![(255u8, 0u8, 0u8), (0, 255u8, 127u8)]
        ];
        let tex = glium::Texture2d::new(display, tex);
        let program = glium::Program::from_source(
            &display,
            &assets::load_str_asset("vertex.glsl").unwrap(),
            &assets::load_str_asset("fragment.glsl").unwrap(),
            None
            ).unwrap();

        let s = assets::load_str_asset("player.obj").unwrap();
        let playerobj = &wavefront_obj::obj::parse(s).unwrap().objects[0];

        let playermodel = vel0city::graphics::wavefront::obj_to_model(playerobj,
                                                                      Arc::new(program),
                                                                      tex,
                                                                      display);
        let input = vel0city::input::Input::new();
        Client {
            playermodel: playermodel,
            input: input,
        }
    }
}

#[cfg(not(test))]
fn main() {
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();
    let mut client = Client::new(&display);
    let (x, y) = display.get_framebuffer_dimensions();
    let mut drawparams: glium::DrawParameters = std::default::Default::default();
    drawparams.depth_test = glium::DepthTest::IfLess;
    drawparams.depth_write = true;

    let proj = na::Persp3::new(x as f32 / y as f32, 90.0, 0.001, 4096.0).to_mat();

    let mut game = vel0city::Game {
        movesettings: std::default::Default::default(),
        players: vec![vel0city::player::Player {
            pos: na::Pnt3::new(0.0, 10.0, 7.),
            eyeheight: 0.0,
            eyeang: na::UnitQuat::new_with_euler_angles(0.,0.,0.,),
            halfextents: vel0city::player::PLAYER_HALFEXTENTS,
            vel: na::zero(),
            flags: vel0city::player::PlayerFlags::empty(),
        }],
        map: vel0city::map::single_plane_map()
    };

    let asset = assets::load_bin_asset("test.bsp").unwrap();
    let mapmodel = vel0city::qbsp_import::import_graphics_model(&asset, &display).unwrap();
    
    //display.get_window().unwrap().set_cursor(glutin::MouseCursor::NoneCursor);
    let mut lasttime = clock_ticks::precise_time_s();
    while !display.is_closed() {
        let curtime = clock_ticks::precise_time_s();
        let frametime = curtime - lasttime;
        lasttime = curtime;
        
        let win = display.get_window().unwrap();
        for ev in win.poll_events() {
            client.input.handle_event(&win, &ev);
        }

        let l = na::Iso3::new_with_rotmat(na::zero(), client.input.get_ang().to_rot()).inv().unwrap().to_homogeneous();
        let v = na::Iso3::new(game.players[0].pos.to_vec() * -1.0, na::zero()).to_homogeneous();
        //l.inv();
        let view = vel0city::graphics::View {
            w2s: proj * l * v,
            drawparams: drawparams, 
        };

        let mi = client.input.make_moveinput(&game.movesettings);

        vel0city::player::movement::move_player(&mut game, 0, &mi, frametime as f32);

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.1, 0.0), 1.0);
        vel0city::graphics::draw_view(&game,
                                      &view,
                                      &client.playermodel,
                                      &mapmodel,
                                      &mut target);
        target.finish();
    }
        
}
