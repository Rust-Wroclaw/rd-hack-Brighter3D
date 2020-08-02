#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate env_logger;
extern crate serde_json;
extern crate notify;
extern crate reqwest;
extern crate failure;
#[macro_use] extern crate failure_derive;

mod argvalues;
mod runner;
mod loader;
mod error;

use std::string::String;
use std::thread;
use std::str;
use argvalues::ArgValues;

fn main() {

    thread::spawn(|| {
        main_shader();
    });    

    main_gui();
}

fn main_shader() {
    let _res = runner::run(&ArgValues::new());
} 

extern crate nuklear;
extern crate nuklear_backend_gfx;

use nuklear::*;
use nuklear_backend_gfx::{Drawer, GfxBackend};

use glutin::GlRequest;
use glutin::dpi::{LogicalSize, LogicalPosition};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const MAX_VERTEX_MEMORY: usize = 512 * 1024;
const MAX_ELEMENT_MEMORY: usize = 128 * 1024;
const MAX_COMMANDS_MEMORY: usize = 64 * 1024;


#[derive(Clone)]
struct NuclearFloat {
    text: [u8; 64],
    text_len: i32,
    val : f32,
}

impl NuclearFloat {
    fn from_val(v:f32)->Self {
        let     text_s = format!("{}",v);
        let mut chars  = [0u8;64];
        let mut len    = 0i32;

        for c in text_s.chars()
        {
            chars[len as usize] = c as u8;
            len+=1;
        }
        NuclearFloat{
            text: chars,
            text_len: len,
            val: v,
        }
    }

    fn update(&mut self)->f32
    {        
        let st   = str::from_utf8(&self.text).unwrap_or("");                    
        self.val = st[..self.text_len as usize].parse::<f32>().unwrap();
        self.val
    }

    fn val(&self)->f32 {
        self.val
    }
}

#[derive(Clone)]
struct SphereState {
    red  : i32,
    green: i32,
    blue : i32,
    id : i32,
    x: NuclearFloat,
    y: NuclearFloat,
    z: NuclearFloat,
    radius: NuclearFloat,
}

impl SphereState {
    fn new(id: &mut usize)->Self {
        *id+=1;
        SphereState {
            red  : 255,
            green: 255,
            blue : 255,
            id : *id as i32,
            x: NuclearFloat::from_val(0.0),
            y: NuclearFloat::from_val(2.0),
            z: NuclearFloat::from_val(0.0),
            radius: NuclearFloat::from_val(1.0),
        }   
    }

    fn update(&mut self){
        self.x.update();
        self.y.update();
        self.z.update();
        self.radius.update();
    }

    fn get_shader(&self)->String {

        let code = 256.0*256.0*self.red as f32 + 256.0*self.green as f32 + self.blue as f32;
        format!(" res = opU( res, vec2( sdSphere(pos-vec3({},{},{}), {} ), {} ) );\n",
                self.x.val()*0.1,
                self.y.val()*0.1,
                self.z.val()*0.1,
                self.radius.val()*0.1*2.0,
                code as f32+100.0)
    
    }
}

enum ActionState {
    None,
    AddSphere,
    //CopyShader,
    CompileShader
}

#[allow(dead_code)]
struct Media {
    font_atlas: FontAtlas,
    font_14: FontID,
    font_18: FontID,
    font_20: FontID,
    font_22: FontID,

    font_tex: Handle,
}

impl Drop for Media {
    fn drop(&mut self) {
        unsafe {
            self.font_tex = ::std::mem::zeroed();
        }
    }
}

fn generate_shader(spheres:&mut Vec<SphereState>)->String {
    for s in spheres.iter_mut() {
        s.update();
    }
    
    let mut res = vec![];
    
    for s in spheres {
        res.push(s.get_shader());
    }
    res.join("")
}


fn main_gui() {
    let gl_version = GlRequest::GlThenGles {
        opengles_version: (2, 0),
        opengl_version: (3, 3),
    };


    let builder = glutin::WindowBuilder::new().with_title("Distance Field Editor").
            with_dimensions(LogicalSize { width: 920., height: 1000. });

    let context = glutin::ContextBuilder::new().with_gl(gl_version).with_vsync(true).with_srgb(false).with_depth_buffer(24);
    let mut event_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, mut main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &event_loop).unwrap();
    window.set_position(LogicalPosition{x:0.0,y:0.0});
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut cfg = FontConfig::with_size(0.0);
    cfg.set_oversample_h(3);
    cfg.set_oversample_v(2);
    cfg.set_glyph_range(font_cyrillic_glyph_ranges());
    cfg.set_ttf(include_bytes!("../res/fonts/Roboto-Regular.ttf"));

    let mut allo = Allocator::new_vec();

    let mut drawer = Drawer::new(&mut factory, main_color, 36, MAX_VERTEX_MEMORY, MAX_ELEMENT_MEMORY, Buffer::with_size(&mut allo, MAX_COMMANDS_MEMORY), GfxBackend::OpenGlsl150);

    let mut atlas = FontAtlas::new(&mut allo);

    cfg.set_ttf_data_owned_by_atlas(false);
    cfg.set_size(14f32);
    let font_14 = atlas.add_font_with_config(&cfg).unwrap();

    cfg.set_ttf_data_owned_by_atlas(false);
    cfg.set_size(18f32);
    let font_18 = atlas.add_font_with_config(&cfg).unwrap();

    cfg.set_ttf_data_owned_by_atlas(false);
    cfg.set_size(20f32);
    let font_20 = atlas.add_font_with_config(&cfg).unwrap();

    cfg.set_ttf_data_owned_by_atlas(false);
    cfg.set_size(22f32);
    let font_22 = atlas.add_font_with_config(&cfg).unwrap();

    let font_tex = {
        let (b, w, h) = atlas.bake(FontAtlasFormat::Rgba32);
        drawer.add_texture(&mut factory, b, w, h)
    };

    let mut null = DrawNullTexture::default();

    atlas.end(font_tex, Some(&mut null));
    //atlas.cleanup();

    let mut ctx = Context::new(&mut allo, atlas.font(font_14).unwrap().handle());

    let mut media = Media {
        font_atlas: atlas,
        font_14: font_14,
        font_18: font_18,
        font_20: font_20,
        font_22: font_22,

        font_tex: font_tex,
    };

    let mut spheres = vec![];
    let mut sphere_uid = 0;
    
    spheres.push(SphereState::new(&mut sphere_uid));
    
    let mut mx = 0;
    let mut my = 0;

    let mut config = ConvertConfig::default();
    config.set_null(null.clone());
    config.set_circle_segment_count(22);
    config.set_curve_segment_count(22);
    config.set_arc_segment_count(22);
    config.set_global_alpha(1.0f32);
    config.set_shape_aa(AntiAliasing::On);
    config.set_line_aa(AntiAliasing::On);

    let mut closed = false;
    while !closed {
        ctx.input_begin();
        event_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::ReceivedCharacter(c) => {
                        ctx.input_unicode(c);
                    }
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput { state, virtual_keycode, .. },
                        ..
                    } => {
                        if let Some(k) = virtual_keycode {
                            let key = match k {
                                glutin::VirtualKeyCode::Back => Key::Backspace,
                                glutin::VirtualKeyCode::Delete => Key::Del,
                                glutin::VirtualKeyCode::Up => Key::Up,
                                glutin::VirtualKeyCode::Down => Key::Down,
                                glutin::VirtualKeyCode::Left => Key::Left,
                                glutin::VirtualKeyCode::Right => Key::Right,
                                _ => Key::None,
                            };

                            ctx.input_key(key, state == glutin::ElementState::Pressed);
                        }
                    }
                    glutin::WindowEvent::CursorMoved { position: LogicalPosition { x, y }, .. } => {
                        mx = x as i32;
                        my = y as i32;
                        ctx.input_motion(x as i32, y as i32);
                    }
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        let button = match button {
                            glutin::MouseButton::Left => Button::Left,
                            glutin::MouseButton::Middle => Button::Middle,
                            glutin::MouseButton::Right => Button::Right,
                            _ => Button::Max,
                        };

                        ctx.input_button(button, mx, my, state == glutin::ElementState::Pressed)
                    }
                    glutin::WindowEvent::MouseWheel { delta, .. } => {
                        if let glutin::MouseScrollDelta::LineDelta(x, y) = delta {
                            ctx.input_scroll(Vec2 { x: x * 22f32, y: y * 22f32 });
                        }
                    }
                    glutin::WindowEvent::Resized(_) => {
                        let mut main_color = drawer.col.clone().unwrap();
                        gfx_window_glutin::update_views(&window, &mut main_color, &mut main_depth);
                        drawer.col = Some(main_color);
                    }
                    _ => (),
                }
            }
        });
        ctx.input_end();

        if closed {
            break;
        }

        // println!("{:?}", event);
        let LogicalSize { width, height } = window.get_inner_size().unwrap();
        let scale = Vec2 { x: 1., y: 1. };

        match control_panel(&mut ctx, &mut media) {            
            ActionState::AddSphere     => { spheres.push(SphereState::new(&mut sphere_uid)); },
            ActionState::CompileShader => { let inner_shader = generate_shader(&mut spheres); 
                let s = loader::generate_shader_from_template(inner_shader);    
                let _res = loader::save_to_file("./shaders/default.frag", s);                                    
              },
            /*ActionState::CopyShader    => { 
                let inner_shader = generate_shader(&mut spheres); 
                let text = loader::generate_shader_from_template(inner_shader);                   
                set_clipboard(formats::Unicode, text).expect("To set clipboard");            
                },  */
            ActionState::None          => {},
        }
      
        let mut remove_id = -1;

        for i in 0..spheres.len() {            
            if sphere_demo(&mut ctx, &mut media, &mut spheres[i]) {
                remove_id = i as i32;
            }
        }

        if remove_id!=-1 {
            spheres.remove(remove_id as usize);
        }
        

        encoder.clear(drawer.col.as_ref().unwrap(), [0.1f32, 0.2f32, 0.3f32, 1.0f32]);
        drawer.draw(&mut ctx, &mut config, &mut encoder, &mut factory, width as u32, height as u32, scale);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        
        ::std::thread::sleep(::std::time::Duration::from_millis(20));

        ctx.clear();
    }
}

fn ui_header(ctx: &mut Context, media: &mut Media, title: &str) {
    ctx.style_set_font(media.font_atlas.font(media.font_18).unwrap().handle());
    ctx.layout_row_dynamic(20f32, 1);
    ctx.text(title, TextAlignment::Left as Flags);
}

const RATIO_W: [f32; 2] = [0.15f32, 0.85f32];
fn ui_widget(ctx: &mut Context, media: &mut Media, height: f32) {
    ctx.style_set_font(media.font_atlas.font(media.font_22).unwrap().handle());
    ctx.layout_row(LayoutFormat::Dynamic, height, &RATIO_W);
    // ctx.layout_row_dynamic(height, 1);
    ctx.spacing(1);
}

fn control_panel(ctx: &mut Context, media: &mut Media)->ActionState {
    ctx.style_set_font(media.font_atlas.font(media.font_20).unwrap().handle());
    ctx.begin(
        nk_string!("Control Panel"),
        Rect { x: 320f32, y: 50f32, w: 275f32, h: 610f32 },
        PanelFlags::Border as Flags | PanelFlags::Movable as Flags | PanelFlags::Title as Flags,
    );


    ui_header(ctx, media, "Create");
    ui_widget(ctx, media, 35f32);

    let mut action = ActionState::None;

    if ctx.button_text("Sphere") {
        action = ActionState::AddSphere;
    }

    ui_header(ctx, media, "Shader");

    /*
    ui_widget(ctx, media, 35f32);
    if ctx.button_text("Copy to Clipboard") {
        action = ActionState::CopyShader;
    }
    */

    ui_widget(ctx, media, 35f32);
    if ctx.button_text("Compile") {
        action = ActionState::CompileShader;
    }
    ctx.end();

    action
}

fn sphere_demo(ctx: &mut Context, media: &mut Media, state: &mut SphereState)->bool {
    ctx.style_set_font(media.font_atlas.font(media.font_20).unwrap().handle());

    let off = (state.id-1) as f32*10.0;
    ctx.begin(        
        nk_string!("Sphere{}",state.id),
        Rect { x: 50f32+off, y: 50f32+off, w: 255f32, h: 410f32 },
        PanelFlags::Border as Flags | PanelFlags::Movable as Flags | PanelFlags::Title as Flags,   
    );

    // ------------------------------------------------
    ctx.style_set_font(media.font_atlas.font(media.font_20).unwrap().handle());
    ui_header(ctx, media, "Color");

   

    ctx.layout_row_dynamic(30f32, 2);

    ctx.style_set_font(media.font_atlas.font(media.font_14).unwrap().handle());
    ctx.label(nk_string!("Red:"), TextAlign::Right as Flags);
    
    ctx.slider_int(0,&mut state.red,255,1);
    ctx.label(nk_string!("Green:"), TextAlign::Right as Flags);
    
    ctx.slider_int(0,&mut state.green,255,1);
    ctx.label(nk_string!("Blue:"), TextAlign::Right as Flags);
    
    ctx.slider_int(0,&mut state.blue,255,1);

    ctx.style_set_font(media.font_atlas.font(media.font_20).unwrap().handle());
    ui_header(ctx, media, "Position");
      
    ctx.style_set_font(media.font_atlas.font(media.font_14).unwrap().handle());
    ctx.layout_row_dynamic(30f32, 2);
    ctx.text("X:", TextAlignment::Right as Flags);
    
    ctx.edit_string(EditType::Field as Flags, &mut state.x.text, &mut state.x.text_len, NK_FILTER_FLOAT);
    ctx.text("Y:", TextAlignment::Right as Flags);
    ctx.edit_string(EditType::Field as Flags, &mut state.y.text, &mut state.y.text_len, NK_FILTER_FLOAT);
    ctx.text("Z:", TextAlignment::Right as Flags);
    ctx.edit_string(EditType::Field as Flags, &mut state.z.text, &mut state.z.text_len, NK_FILTER_FLOAT);
    ctx.text("Radius:", TextAlignment::Right as Flags);
    ctx.edit_string(EditType::Field as Flags, &mut state.radius.text, &mut state.radius.text_len, NK_FILTER_FLOAT);

    ui_header(ctx, media, "Remove");
    ui_widget(ctx, media, 35f32);

    let mut remove = false;

    if ctx.button_text("Remove") {
        remove = true;
    }

    ctx.style_set_font(media.font_atlas.font(media.font_14).unwrap().handle());
    ctx.end();

    remove
}
