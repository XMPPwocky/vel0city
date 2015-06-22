extern crate glium;
extern crate glutin;
extern crate vel0city;
extern crate nalgebra as na;
extern crate clock_ticks;
extern crate image;

use std::borrow::ToOwned;
use glium::DisplayBuild;
use glium::Surface;

use vel0city::assets;
use vel0city::graphics::hud;
use na::{
    Diag,
    Rotation,
    Rotate,
    ToHomogeneous,
    Inv
};
use std::f32::consts::{
    PI,
};

pub struct Client {
    input: vel0city::input::Input,
    hudmanager: vel0city::graphics::hud::HudManager,
    hudelements: Vec<hud::Element>,
    scene: Option<vel0city::graphics::Scene>,
}
impl Client {
    fn new(display: &glium::Display) -> Client {
        let input = vel0city::input::Input::new();
        let hudmanager = hud::HudManager::new(display);

        let tex = assets::load_bin_asset("textures/arrow.png").unwrap();
        let tex = image::load(std::io::Cursor::new(tex), image::PNG).unwrap();
        let tex = glium::Texture2d::new(display, tex);

        fn id(context: &hud::Context) -> Option<na::Mat4<f32>> {
            let eyedir = context.eyeang.rotate(&na::Vec3::new(0.0, 0.0, -1.0));
            let eye_heading = f32::atan2(eyedir.x, eyedir.z);
            let vel_heading = f32::atan2(context.player_vel.x, context.player_vel.z);
            let ang = eye_heading - vel_heading; 
            let scale = na::norm(&na::Vec2::new(context.player_vel.x, context.player_vel.z)) / 1500.0;
            if scale > 0.03 {
                let scalemat = na::Mat4::from_diag(&na::Vec4::new(0.15, scale, 1.0, 1.0));
                let rotmat = na::Rot3::new(na::Vec3::new(0.0, 0.0, ang)).to_homogeneous();
                Some(rotmat * scalemat)
            } else {
                None
            }
        }

        Client {
            input: input,
            hudmanager: hudmanager,
            hudelements: vec![hud::Element {
                transform: na::Iso2::new(na::zero(), na::zero()),
                element_type: hud::ElementType::TransformedBlit {
                    texture: tex,
                    f: id 
                }
            }],
            scene: None,
        }
    }
}

#[cfg(not(test))]
fn main() {
    let display = glutin::WindowBuilder::new()
        // .with_vsync()
        .with_title("vel0city".to_owned())
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();
    let mut client = Client::new(&display);
    let (x, y) = display.get_framebuffer_dimensions();

    let proj = na::Persp3::new(x as f32 / y as f32, 90.0, 1.0, 4096.0).to_mat();

    let asset = assets::load_bin_asset("maps/test.bsp").unwrap();
    let mut game = vel0city::Game {
        movesettings: std::default::Default::default(),
        players: vec![Default::default()],
        map: vel0city::map::q3_import::import(&asset).unwrap(),
        timescale: 1.0,
        time: 0.0,
    };

    let mapmodel = vel0city::map::q3_import::import_graphics_model(&asset, &display).unwrap();
    let ents = vel0city::map::q3_import::import_entities(&asset).unwrap();
    println!("{}", ents);
    client.scene = Some(vel0city::graphics::Scene {
        map: mapmodel,
        lights: vec![ vel0city::graphics::Light { position: na::zero(), intensity: 0.0, radius: 0.5, color: na::Vec3::new(0.0, 1.0, 1.0) }] 
    });
    
    let mut winsize;
    {
        let window = display.get_window().unwrap();
        winsize = window.get_inner_size().unwrap();
        window.set_cursor_state(glutin::CursorState::Grab).unwrap();
    }
    //client.input.cursorpos = (winsize.0 as i32 / 2, winsize.1 as i32 / 2);

    let psystem = vel0city::graphics::passes::PassSystem::new(&display);
    let cel_program = glium::Program::from_source(
        &display,
        &assets::load_str_asset("shaders/post/vertex.glsl").unwrap(),
        &assets::load_str_asset("shaders/post/cel_fragment.glsl").unwrap(),
        None
        ).unwrap();
    let cel_technique = vel0city::graphics::passes::Technique {
        shader: cel_program,
        drawparams: glium::DrawParameters {
            ..::std::default::Default::default()
        }
    };
    let light_program = glium::Program::from_source(
        &display,
        &assets::load_str_asset("shaders/light/vertex.glsl").unwrap(),
        &assets::load_str_asset("shaders/light/dlight_fragment.glsl").unwrap(),
        None
        ).unwrap();
    let light_technique = vel0city::graphics::passes::Technique {
        shader: light_program,
        drawparams: glium::DrawParameters {
            blending_function: Some(glium::BlendingFunction::Addition {
                source: glium::LinearBlendingFactor::One,
                destination: glium::LinearBlendingFactor::One,
            }),
            ..::std::default::Default::default()
        }
    };

    let mut pass_data = vel0city::graphics::passes::PassData::new(&display, (winsize.0, winsize.1)); 
    
    let tick = 1.0/120.0;
    let mut lasttime = clock_ticks::precise_time_s();
    let mut accumtime = 0.0;
    let mut smoothtime = 0.0;
    'mainloop: loop { 
        let curtime = clock_ticks::precise_time_s();
        let frametime = curtime - lasttime;
        accumtime += frametime;
        smoothtime = (smoothtime + frametime) / 2.0;
        lasttime = curtime;
        //println!("frametime: {}us", smoothtime * 1000.0 * 1000.0);

        let win = display.get_window().unwrap();
        for ev in win.poll_events() {
            match &ev {
                &glutin::Event::Resized(width, height) => {
                    winsize = (width, height);
                    if winsize.0 < 2 { winsize.0 = 2; }
                    if winsize.1 < 2 { winsize.1 = 2; }
                    pass_data = vel0city::graphics::passes::PassData::new(&display, (winsize.0, winsize.1)); 
                    client.input.cursorpos = (winsize.0 as i32 / 2, winsize.1 as i32 / 2);
                },
                &glutin::Event::Closed => {
                    break 'mainloop
                },
                _ => ()
            }

            client.input.handle_event(&win, &ev);
        }


        if accumtime >= tick {
            // handle dropped frames more gracefully
            accumtime = f64::min(tick * 3.0, accumtime);
            while accumtime >= tick {
                let mi = client.input.make_moveinput(&game.movesettings);
                accumtime -= tick;
                let timescale = game.timescale; // borrow checker hack
                let time = tick as f32 * timescale;
                game.time += time;
                vel0city::player::movement::move_player(&mut game, 0, &mi, time);
                // FIXME: hack 
                //client.input.ang = game.players[0].eyeang;
            }
        }

        let pv = game.players[0].vel;
        let rot = game.players[0].eyeang
            .append_rotation(&na::Vec3::new(PI, 0.0, 0.0)).to_rot();

        let l = na::Iso3::new_with_rotmat(na::zero(), rot).inv().unwrap().to_homogeneous();
        let v = na::Iso3::new(game.players[0].get_eyepos().to_vec() * -1.0, na::zero()).to_homogeneous(); 
        let lv = l * v;
        //l.inv();
        let view = vel0city::graphics::View {
            cam: lv,
            w2s: proj * lv,
        };

        let mut target = display.draw();
        pass_data.get_framebuffer_for_prepass(&display).clear_depth(1.0);
        if let Some(ref mut scene) = client.scene {
            scene.lights[0].position = game.players[0].pos.to_vec() + na::Vec3::new(0.0, vel0city::player::PLAYER_HALFEXTENTS.y * 0.1, 0.0);
            scene.lights[0].intensity = na::clamp(na::norm(&na::Vec2::new(pv.x, pv.z)) / 5.0, 2.0, 50.0);


            vel0city::graphics::draw_scene(&mut pass_data.get_framebuffer_for_prepass(&display),
                                           &scene,
                                           &view);
            psystem.light_passes(&display, &mut pass_data, &scene.lights, &view, &light_technique);

            psystem.postprocess(&pass_data, &mut target, &cel_technique);
        };
        let hudcontext = hud::Context {
            eyeang: game.players[0].eyeang,
            player_vel: game.players[0].vel
        };

        client.hudmanager.draw_elements(&mut target, &hudcontext, &client.hudelements);

        target.finish().unwrap();
    }
        
}
