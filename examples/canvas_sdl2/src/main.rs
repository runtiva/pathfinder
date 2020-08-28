// pathfinder/examples/canvas_minimal/src/main.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#[macro_use]
extern crate log;

use euclid::default::Size2D;
use pathfinder_canvas::{Canvas, CanvasFontContext, Path2D, CompositeOperation};

use pathfinder_color::{ColorF, rgbau, rgbaf};

use pathfinder_simd::default::F32x2;
use pathfinder_content::gradient::Gradient;
use pathfinder_content::fill::FillRule;
use pathfinder_content::outline::ArcDirection;
use pathfinder_content::stroke::LineCap;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_geometry::vector::Vector2F;
use pathfinder_geometry::line_segment::LineSegment2F;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::transform2d::Matrix2x2F;
use pathfinder_simd::default::F32x4;

use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::executor::SequentialExecutor;
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererMode, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use surfman::{Connection, ContextAttributeFlags, ContextAttributes, GLVersion as SurfmanGLVersion};
use surfman::{SurfaceAccess, SurfaceType};
//use winit::dpi::LogicalSize;
//use winit::{ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use std::time::Instant;

use log::LevelFilter;
use env_logger::{Builder, Target};

mod pathfinder_device;

fn main() {
    //test_pdf_transform();
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Error)
        .init();

    let test1 = Vector2F::new(1.23, 4.56);
    let test2 = test1;
    // println!("log_enabled!(error): {}", log_enabled!(log::Level::Error));
    // println!("log_enabled!(Debug): {}", log_enabled!(log::Level::Debug));
    // println!("log_enabled!(Debug): {}", log_enabled!(log::Level::Info));
    // debug!("Line1");
    // if log_enabled!(log::Level::Info) {
    //     let x = 3 * 4; // expensive computation
    //     info!("the answer was: {:?}", x);
    // }

    let start = Instant::now();

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let duration = start.elapsed();
    println!("1: Time elapsed is: {:?}", duration);

    // Make sure we have at least a GL 3.0 context. Pathfinder requires this.
    let gl_attributes = video.gl_attr();
    gl_attributes.set_context_profile(GLProfile::Core);
    gl_attributes.set_context_version(3, 3);

    let duration = start.elapsed();
    println!("2: Time elapsed is: {:?}", duration);

    // Open a window.
    let viewport_width = 2289.706;// / 2.0; //800.0;
    let viewport_height = 909.151;// / 2.0; // 800.0;
    let viewport_scale = 1.0; //0.5;
    let window_size = vec2i((viewport_width * viewport_scale) as i32, (viewport_height * viewport_scale) as i32);
    let window = video.window("Text example", window_size.x() as u32, window_size.y() as u32)
                      .opengl()
                      .build()
                      .unwrap();

    let duration = start.elapsed();
    println!("3: Time elapsed is: {:?}", duration);

    // // Create a `surfman` device. On a multi-GPU system, we'll request the low-power integrated
    // // GPU.
    // let connection = Connection::from_winit_window(&window).unwrap();
    // let native_widget = connection.create_native_widget_from_winit_window(&window).unwrap();
    // let adapter = connection.create_low_power_adapter().unwrap();
    // let mut device = connection.create_device(&adapter).unwrap();

    // // Request an OpenGL 3.x context. Pathfinder requires this.
    // let context_attributes = ContextAttributes {
    //     version: SurfmanGLVersion::new(3, 0),
    //     flags: ContextAttributeFlags::ALPHA,
    // };
    // let context_descriptor = device.create_context_descriptor(&context_attributes).unwrap();

    // // Make the OpenGL context via `surfman`, and load OpenGL functions.
    // let surface_type = SurfaceType::Widget { native_widget };
    // let mut context = device.create_context(&context_descriptor).unwrap();
    // let surface = device.create_surface(&context, SurfaceAccess::GPUOnly, surface_type)
    //                     .unwrap();
    // device.bind_surface_to_context(&mut context, surface).unwrap();
    // device.make_context_current(&context).unwrap();
    // gl::load_with(|symbol_name| device.get_proc_address(&context, symbol_name));

    // Create the GL context, and make it current.
    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video.gl_get_proc_address(name) as *const _);
    window.gl_make_current(&gl_context).unwrap();

    let duration = start.elapsed();
    println!("4: Time elapsed is: {:?}", duration);

    // // Create a Pathfinder renderer.
    // let mode = RendererMode::default_for_device(&pathfinder_device);
    // let resource_loader = EmbeddedResourceLoader;
    // let mut renderer = Renderer::new(GLDevice::new(GLVersion::GL3, 0),
    //                                  &resource_loader,
    //                                  DestFramebuffer::full_window(window_size),
    //                                  RendererOptions {
    //                                      background_color: Some(ColorF::white()),
    //                                      ..RendererOptions::default()
    //                                  });

    // // Get the real size of the window, taking HiDPI into account.
    // let hidpi_factor = window.get_current_monitor().get_hidpi_factor();
    // let physical_size = logical_size.to_physical(hidpi_factor);
    // let framebuffer_size = vec2i(physical_size.width as i32, physical_size.height as i32);

    // // Create a Pathfinder GL device.
    // let default_framebuffer = device.context_surface_info(&context)
    //                                 .unwrap()
    //                                 .unwrap()
    //                                 .framebuffer_object;
    // let pathfinder_device = GLDevice::new(GLVersion::GL3, default_framebuffer);

    // Create a Pathfinder renderer.
    let pathfinder_device = GLDevice::new(GLVersion::GL3, 0);
    let mode = RendererMode::default_for_device(&pathfinder_device);
    let options = RendererOptions {
        dest: DestFramebuffer::full_window(window_size), // framebuffer_size),
        background_color: Some(ColorF::white()),
        show_debug_ui: false, // ..RendererOptions::default()
    };

    let duration = start.elapsed();
    println!("5: Time elapsed is: {:?}", duration);

    let resource_loader = EmbeddedResourceLoader::new();
    let mut renderer = Renderer::new(pathfinder_device, &resource_loader, mode, options);

    let duration = start.elapsed();
    println!("6: Time elapsed is: {:?}", duration);

    // Make a canvas. We're going to draw a house.
    let font_context = CanvasFontContext::from_system_source();
    let mut canvas = Canvas::new(/*framebuffer_size*/window_size.to_f32()).get_context_2d(font_context);

    let duration = start.elapsed();
    println!("7: Time elapsed is: {:?}", duration);

    // Build POC here
    //viewport_height

    canvas.set_fill_style(rgbau((0.38 * 255.) as u8, (0.776 * 255.) as u8, (0.898 * 255.) as u8, (1.0 * 255.) as u8));
    let mut path = Path2D::new();

    let base_transform = Transform2F::row_major(1., 0., 341.0606, 0., 1., 532.2056);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    canvas.set_transform(&pdf_transform);

    let v = vec2f(0., 0.);
    path.move_to(v);
    let v = vec2f(0., -227.586);
    path.line_to(v);
    let v = vec2f(-153.374, -227.586);
    path.line_to(v);
    path.line_to(vec2f(-153.373, 0.0));
    path.bezier_curve_to(vec2f(-153.374, 46.105), vec2f(-119.034, 83.485), vec2f(-76.682, 83.485));
    path.bezier_curve_to(vec2f(-34.33, 83.485), vec2f(0.0, 46.105), vec2f(0.0, 0.0));
    path.close_path();
    canvas.fill_path(path, FillRule::Winding);

    let mut path = Path2D::new();
    canvas.set_fill_style(rgbau(255, 255, 255, 255));
    let base_transform = Transform2F::row_major(1., 0., 1403.9215, 0., 1., 322.5688);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    canvas.set_transform(&pdf_transform);
    path.move_to(vec2f(0., 0.));
    path.bezier_curve_to(vec2f(0., -113.901), vec2f(-92.335, -206.236), vec2f(-206.236, -206.236));
    path.bezier_curve_to(vec2f(-320.137, -206.236), vec2f(-412.472, -113.901), vec2f(-412.472, 0.0));
    path.bezier_curve_to(vec2f(-412.472, 113.9), vec2f(-320.137, 206.235), vec2f(-206.236, 206.235));
    path.bezier_curve_to(vec2f(-92.335, 206.235), vec2f(0.0, 113.8), vec2f(0., 0.));
    canvas.fill_path(path, FillRule::Winding);

    canvas.set_stroke_style(rgbau((0.137 * 255.) as u8, (0.122 * 255.) as u8, (0.125 * 255.) as u8, 255));
    canvas.set_line_width(14.0 * viewport_scale);

    // let base_transform = Transform2F::row_major(1., 0., 1403.9215, 0., 1., 322.5688);
    // let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let mut path = Path2D::new();
    path.move_to(vec2f(0., 0.));
    path.bezier_curve_to(vec2f(0., -113.901), vec2f(-92.335, -206.236), vec2f(-206.236, -206.236));
    path.bezier_curve_to(vec2f(-320.137, -206.236), vec2f(-412.472, -113.901), vec2f(-412.472, 0.0));
    path.bezier_curve_to(vec2f(-412.472, 113.9), vec2f(-320.137, 206.235), vec2f(-206.236, 206.235));
    path.bezier_curve_to(vec2f(-92.335, 206.235), vec2f(0.0, 113.8), vec2f(0., 0.));
    path.close_path();
    canvas.stroke_path(path);



    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 1403.9215, 0., 1., 584.731);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    canvas.set_transform(&pdf_transform);

    path.move_to(vec2f(0., 0.));
    path.bezier_curve_to(vec2f(0., -113.901), vec2f(-92.335, -206.236), vec2f(-206.236, -206.236));
    path.bezier_curve_to(vec2f(-320.137, -206.236), vec2f(-412.472, -113.901), vec2f(-412.472, 0.0));
    path.bezier_curve_to(vec2f(-412.472, 113.9), vec2f(-320.137, 206.235), vec2f(-206.236, 206.235));
    path.bezier_curve_to(vec2f(-92.335, 206.235), vec2f(0.0, 113.8), vec2f(0., 0.));
    canvas.fill_path(path, FillRule::Winding);

    let base_transform = Transform2F::row_major(1., 0., 1403.9215, 0., 1., 584.731);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    canvas.set_transform(&pdf_transform);
    let mut path = Path2D::new();
    path.move_to(vec2f(0., 0.));
    path.bezier_curve_to(vec2f(0., -113.901), vec2f(-92.335, -206.236), vec2f(-206.236, -206.236));
    path.bezier_curve_to(vec2f(-320.137, -206.236), vec2f(-412.472, -113.901), vec2f(-412.472, 0.0));
    path.bezier_curve_to(vec2f(-412.472, 113.9), vec2f(-320.137, 206.235), vec2f(-206.236, 206.235));
    path.bezier_curve_to(vec2f(-92.335, 206.235), vec2f(0.0, 113.8), vec2f(0., 0.));
    path.close_path();
    canvas.stroke_path(path);

    canvas.reset_transform();

/*
//    0.38 0.776 0.898 rg

    // Line 227 - rectangle
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, Transform2F::default());
    let mut path = Path2D::new();
    let x1 = 613.787;
    let y1 = 530.183;
    let x2 = 613.787-178.328;
    let y2 = 530.183-243.077;
    let v1 = vec2f(x1, y1);
    let v2 = vec2f(x1, y2);
    let v3 = vec2f(x2, y2);
    let v4 = vec2f(x2, y1);

    path.move_to(transform(pdf_transform, v1));
    path.line_to(transform(pdf_transform, v2));
    path.line_to(transform(pdf_transform, v3));
    path.line_to(transform(pdf_transform, v4));
    path.close_path();
    canvas.stroke_path(path);

    // Line 229
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 613.7867, 0., 1., 530.1831);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(0., -49.243)), transform(pdf_transform, vec2f(-39.92, -89.163)), transform(pdf_transform, vec2f( -89.164, -89.163)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-138.408, -89.163)), transform(pdf_transform, vec2f( -178.328, -49.243)), transform(pdf_transform, vec2f( -178.328, 0.)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-178.328, 49.244 )), transform(pdf_transform, vec2f(-138.408, 89.164 )), transform(pdf_transform, vec2f(-89.164, 89.164)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-39.92, 89.164)), transform(pdf_transform, vec2f( 0., 49.244)), transform(pdf_transform, vec2f( 0., 0.)));
    canvas.stroke_path(path);

    // Line 238
    canvas.set_fill_style(rgbau((0.137 * 255.) as u8, (0.123 * 255.) as u8, (0.126 * 255.) as u8, 255));
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 842.6124, 0., 1., 499.0679);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(-10.437, 10.437)), transform(pdf_transform, vec2f(-24.839, 16.909)), transform(pdf_transform, vec2f(-40.792, 16.909)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-72.74, 16.909)), transform(pdf_transform, vec2f(-98.63, -9.025)), transform(pdf_transform, vec2f(-98.63, -40.975)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-98.63, -72.88)), transform(pdf_transform, vec2f(-72.742, -98.813)), transform(pdf_transform, vec2f(-40.792, -98.813)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-24.839, -98.813)), transform(pdf_transform, vec2f(-10.437, -92.341)), transform(pdf_transform, vec2f(0., -81.903)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(10.528, -71.466)), transform(pdf_transform, vec2f( 17.046, -56.973)), transform(pdf_transform, vec2f(17.046, -40.975)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(17.046, -24.932)), transform(pdf_transform, vec2f( 10.528, -10.438)), transform(pdf_transform, vec2f(0., 0.)));
    canvas.fill_path(path, FillRule::Winding);

    // Line 249
    canvas.set_stroke_style(rgbau((0.902 * 255.) as u8, (0.906 * 255.) as u8, (0.91 * 255.) as u8, 255));
    canvas.set_line_width(2.0 * viewport_scale);
    let base_transform = Transform2F::row_major(1., 0., 1676.2135, 0., 1., 817.5728);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let mut path = Path2D::new();
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.line_to(transform(pdf_transform, vec2f(0., -688.108)));
    canvas.stroke_path(path);

    // Line 256
    let base_transform = Transform2F::row_major(1., 0., 2070.4858, 0., 1., 319.7251);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let mut path = Path2D::new();
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.line_to(transform(pdf_transform, vec2f(-167.418, 0.)));
    path.line_to(transform(pdf_transform, vec2f(-167.418, 242.291)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-167.418, 275.271)), transform(pdf_transform, vec2f(-148.235, 303.871)), transform(pdf_transform, vec2f(-120.44, 317.466)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-109.364, 322.954)), transform(pdf_transform, vec2f(-96.876, 326.025)), transform(pdf_transform, vec2f(-83.684, 326.025)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-79.001, 326.025)), transform(pdf_transform, vec2f(-74.47, 325.623)), transform(pdf_transform, vec2f(-69.988, 324.918)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-30.362, 318.321)), transform(pdf_transform, vec2f(0., 283.78)), transform(pdf_transform, vec2f(0., 242.291)));
    path.close_path();

    let v = transform(pdf_transform, vec2f(-19.637, 317.517));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(-33.584, 329.399)), transform(pdf_transform, vec2f( -50.955, 337.556)), transform(pdf_transform, vec2f(-69.988, 340.175)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-74.47, 340.828)), transform(pdf_transform, vec2f(-79.052, 341.131)), transform(pdf_transform, vec2f(-83.684, 341.131)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-138.215, 341.131)), transform(pdf_transform, vec2f( -182.523, 296.821)), transform(pdf_transform, vec2f(-182.523, 242.291)));
    path.line_to(transform(pdf_transform, vec2f(-182.523, -15.105)));
    path.line_to(transform(pdf_transform, vec2f(15.105, -15.105)));
    path.line_to(transform(pdf_transform, vec2f(15.105, 242.291)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(15.105, 272.352)), transform(pdf_transform, vec2f( 1.611, 299.339)), transform(pdf_transform, vec2f(-19.637, 317.517)));
    canvas.fill_path(path, FillRule::Winding);

    // Line 275
    canvas.set_fill_style(rgbau((0.38 * 255.) as u8, (0.776 * 255.) as u8, (0.898 * 255.) as u8, 255));
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 2070.4858, 0., 1., 401.2437);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);

    path.line_to(transform(pdf_transform, vec2f(0., -66.563)));
    path.line_to(transform(pdf_transform, vec2f(-155.133, -66.563)));
    path.line_to(transform(pdf_transform, vec2f(-155.133, -4.632)));

    path.bezier_curve_to(transform(pdf_transform, vec2f(-150.147, -6.344)), transform(pdf_transform, vec2f(-145.112, -7.854)), transform(pdf_transform, vec2f( -140.027, -9.163)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-121.599, -14.047)), transform(pdf_transform, vec2f(-102.616, -16.464)), transform(pdf_transform, vec2f(-83.684, -16.464)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-55.236, -16.464)), transform(pdf_transform, vec2f( -26.737, -10.976)), transform(pdf_transform, vec2f( 0., 0.)));
    canvas.fill_path(path, FillRule::Winding);

    // Line 286
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 2070.4858, 0., 1., 562.0161);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);

    path.line_to(transform(pdf_transform, vec2f(0., -5.337)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-44.46, 12.94)), transform(pdf_transform, vec2f( -93.805, 16.012)), transform(pdf_transform, vec2f(-139.927, 3.877)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-145.062, 2.568)), transform(pdf_transform, vec2f( -150.098, 1.008)), transform(pdf_transform, vec2f( -155.133, -0.705)));
    path.line_to(transform(pdf_transform, vec2f(-155.133, 0.)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-155.133, 30.061)), transform(pdf_transform, vec2f( -141.638, 57.048)), transform(pdf_transform, vec2f( -120.44, 75.175)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-109.364, 80.663)), transform(pdf_transform, vec2f( -96.876, 83.734 )), transform(pdf_transform, vec2f(-83.684, 83.734)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-79.001, 83.734)), transform(pdf_transform, vec2f( -74.47, 83.332)), transform(pdf_transform, vec2f( -69.988, 82.627)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-30.362, 76.03)), transform(pdf_transform, vec2f( 0., 41.489)), transform(pdf_transform, vec2f( 0., 0.)));
    canvas.fill_path(path, FillRule::Winding);

    // Line 298
    canvas.set_fill_style(rgbau((0.137 * 255.) as u8, (0.123 * 255.) as u8, (0.126 * 255.) as u8, 255));
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 2097.8765, 0., 1., 543.1343);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.line_to(transform(pdf_transform, vec2f(0., -17.723)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(5.187, -21.047)), transform(pdf_transform, vec2f( 10.222, -24.622)), transform(pdf_transform, vec2f( 15.105, -28.499)));
    path.line_to(transform(pdf_transform, vec2f(15.105, -9.668)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(10.171, -6.243)), transform(pdf_transform, vec2f( 5.136, -2.971)), transform(pdf_transform, vec2f( 0., 0.)));
    path.close_path();

    // not currently displaying anything ???
    path.move_to(transform(pdf_transform, vec2f(-97.379, 116.766)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-116.462, 114.146)), transform(pdf_transform, vec2f( -133.833, 106.04)), transform(pdf_transform, vec2f( -147.831, 94.057)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-136.754, 99.545)), transform(pdf_transform, vec2f( -124.267, 102.616)), transform(pdf_transform, vec2f( -111.075, 102.616)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-106.392, 102.616)), transform(pdf_transform, vec2f( -101.86, 102.214)), transform(pdf_transform, vec2f( -97.379, 101.509)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-92.948, 102.214)), transform(pdf_transform, vec2f( -88.366, 102.616)), transform(pdf_transform, vec2f( -83.734, 102.616)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-70.592, 102.616)), transform(pdf_transform, vec2f( -58.105, 99.545)), transform(pdf_transform, vec2f( -47.028, 94.107)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-60.975, 105.99)), transform(pdf_transform, vec2f( -78.346, 114.146)), transform(pdf_transform, vec2f( -97.379, 116.766)));
    path.close_path();

    path.move_to(transform(pdf_transform, vec2f(-182.523, 18.177)));
    path.line_to(transform(pdf_transform, vec2f(-182.523, 2.115)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-177.538, 4.028)), transform(pdf_transform, vec2f( -172.503, 5.64)), transform(pdf_transform, vec2f( -167.418, 7.1)));
    path.line_to(transform(pdf_transform, vec2f(-167.418, 18.882)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-167.418, 20.191)), transform(pdf_transform, vec2f( -167.368, 21.5)), transform(pdf_transform, vec2f( -167.317, 22.759)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-172.453, 21.45)), transform(pdf_transform, vec2f( -177.488, 19.89)), transform(pdf_transform, vec2f( -182.523, 18.177)));
    path.close_path();

    path.move_to(transform(pdf_transform, vec2f(-182.523, -146.522)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-177.538, -148.234)), transform(pdf_transform, vec2f( -172.503, -149.745)), transform(pdf_transform, vec2f( -167.418, -151.054)));
    path.line_to(transform(pdf_transform, vec2f(-167.418, -135.395)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-172.503, -133.984)), transform(pdf_transform, vec2f( -177.538, -132.322)), transform(pdf_transform, vec2f( -182.523, -130.46)));
    path.close_path();

    //  re
    let x1 = -27.391;
    let y1 = -208.454;
    let x2 = -27.391 + 15.105;
    let y2 = -208.454 + 15.104;
    let v1 = vec2f(x1, y1);
    let v2 = vec2f(x1, y2);
    let v3 = vec2f(x2, y2);
    let v4 = vec2f(x2, y1);
    path.move_to(transform(pdf_transform, v1));
    path.line_to(transform(pdf_transform, v2));
    path.line_to(transform(pdf_transform, v3));
    path.line_to(transform(pdf_transform, v4));

    path.move_to(transform(pdf_transform, vec2f(-27.391, -193.35)));
    path.move_to(transform(pdf_transform, vec2f(0., -110.622)));
    path.line_to(transform(pdf_transform, vec2f(0., -128.346)));

    path.bezier_curve_to(transform(pdf_transform, vec2f(5.136, -125.375)), transform(pdf_transform, vec2f( 10.171, -122.102)), transform(pdf_transform, vec2f( 15.105, -118.678)));

    path.line_to(transform(pdf_transform, vec2f(15.105, -99.847)));

    path.bezier_curve_to(transform(pdf_transform, vec2f(10.222, -103.724)), transform(pdf_transform, vec2f( 5.187, -107.298)), transform(pdf_transform, vec2f( 0., -110.622)));
    canvas.fill_path(path, FillRule::Winding);

    // Line 332
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 2085.5913, 0., 1., 549.7808);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);

    // 11
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(-4.935, 2.518)), transform(pdf_transform, vec2f( -9.97, 4.834)), transform(pdf_transform, vec2f(-15.105, 6.898)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-59.565, 25.176)), transform(pdf_transform, vec2f(-108.91, 28.247)), transform(pdf_transform, vec2f(-155.032, 16.112)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-155.082, 14.854)), transform(pdf_transform, vec2f(-155.133, 13.545)), transform(pdf_transform, vec2f(-155.133, 12.235)));
    path.line_to(transform(pdf_transform, vec2f(-155.133, 0.453)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-109.061, 13.545)), transform(pdf_transform, vec2f( -59.264, 10.222)), transform(pdf_transform, vec2f(-15.105, -9.516)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-9.97, -11.781)), transform(pdf_transform, vec2f( -4.935, -14.299)), transform(pdf_transform, vec2f(0., -17.019)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(4.179, -19.284)), transform(pdf_transform, vec2f(8.258, -21.752)), transform(pdf_transform, vec2f(12.285, -24.369)));
    path.line_to(transform(pdf_transform, vec2f(12.285, -6.646)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(8.258, -4.279)), transform(pdf_transform, vec2f( 4.179, -2.064)), transform(pdf_transform, vec2f(0., 0.)));
    path.close_path();

    //canvas.fill_path(path, FillRule::Winding); // TEMP
    //canvas.set_fill_style(rgbau(255, 255, 255, 255)); // WHERE IS THIS IN PDF
    //let mut path = Path2D::new();
    path.move_to(transform(pdf_transform, vec2f(45.971, -90.028)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(40.029, -95.97)), transform(pdf_transform, vec2f(33.836, -101.458)), transform(pdf_transform, vec2f( 27.391, -106.493)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(22.507, -110.37)), transform(pdf_transform, vec2f(17.472, -113.944)), transform(pdf_transform, vec2f(12.285, -117.269)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(8.258, -119.887)), transform(pdf_transform, vec2f( 4.179, -122.304)), transform(pdf_transform, vec2f(0., -124.62)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-4.935, -127.339)), transform(pdf_transform, vec2f(-9.97, -129.855)), transform(pdf_transform, vec2f(-15.105, -132.122)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-59.264, -151.859)), transform(pdf_transform, vec2f(-109.061, -155.183)), transform(pdf_transform, vec2f(-155.133, -142.041)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-160.218, -140.631)), transform(pdf_transform, vec2f( -165.253, -138.969 )), transform(pdf_transform, vec2f(-170.238, -137.106)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-174.367, -135.596)), transform(pdf_transform, vec2f( -178.496, -133.935)), transform(pdf_transform, vec2f(-182.524, -132.122)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-187.66, -129.806)), transform(pdf_transform, vec2f( -192.695, -127.339 )), transform(pdf_transform, vec2f(-197.629, -124.569)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-214.144, -115.506)), transform(pdf_transform, vec2f( -229.652, -103.976)), transform(pdf_transform, vec2f( -243.6, -90.028)));
    path.line_to(transform(pdf_transform, vec2f(-262.834, -70.845)));
    path.line_to(transform(pdf_transform, vec2f(-243.6, -51.61)));

    path.bezier_curve_to(transform(pdf_transform, vec2f(-229.652, -37.663)), transform(pdf_transform, vec2f( -214.144, -26.132)), transform(pdf_transform, vec2f(-197.629, -17.068)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-192.695, -14.299)), transform(pdf_transform, vec2f( -187.66, -11.781)), transform(pdf_transform, vec2f( -182.524, -9.516)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-178.496, -7.703)), transform(pdf_transform, vec2f( -174.367, -6.042)), transform(pdf_transform, vec2f( -170.238, -4.531)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-165.253, -2.618)), transform(pdf_transform, vec2f( -160.218, -1.007)), transform(pdf_transform, vec2f( -155.133, 0.453)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-109.061, 13.545)), transform(pdf_transform, vec2f( -59.264, 10.222)), transform(pdf_transform, vec2f( -15.105, -9.516)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-9.97, -11.781)), transform(pdf_transform, vec2f( -4.935, -14.299)), transform(pdf_transform, vec2f( 0., -17.019)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(4.179, -19.284)), transform(pdf_transform, vec2f( 8.258, -21.752)), transform(pdf_transform, vec2f( 12.285, -24.369)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(17.472, -27.693)), transform(pdf_transform, vec2f( 22.507, -31.269)), transform(pdf_transform, vec2f(27.391, -35.146)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(33.836, -40.181)), transform(pdf_transform, vec2f( 40.029, -45.669 )), transform(pdf_transform, vec2f(45.971, -51.61)));
    path.line_to(transform(pdf_transform, vec2f(65.205, -70.845)));
    path.close_path();

    // line 366
    path.move_to(transform(pdf_transform, vec2f(56.645, -40.936)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(47.481, -31.771)), transform(pdf_transform, vec2f( 37.663, -23.564 )), transform(pdf_transform, vec2f(27.391, -16.314)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(22.457, -12.89)), transform(pdf_transform, vec2f( 17.421, -9.617 )), transform(pdf_transform, vec2f(12.285, -6.646)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(8.258, -4.279)), transform(pdf_transform, vec2f(4.179, -2.064)), transform(pdf_transform, vec2f(0., 0.)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-4.935, 2.518)), transform(pdf_transform, vec2f(-9.97, 4.834)), transform(pdf_transform, vec2f(-15.105, 6.898)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-59.565, 25.176)), transform(pdf_transform, vec2f( -108.91, 28.247)), transform(pdf_transform, vec2f(-155.032, 16.112)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-160.168, 14.804)), transform(pdf_transform, vec2f(-165.203, 13.243)), transform(pdf_transform, vec2f(-170.238, 11.53)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-174.367, 10.12)), transform(pdf_transform, vec2f(-178.445, 8.61)), transform(pdf_transform, vec2f(-182.524, 6.898)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-187.609, 4.834)), transform(pdf_transform, vec2f(-192.644, 2.567)), transform(pdf_transform, vec2f(-197.629, 0.05)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-218.072, -10.221)), transform(pdf_transform, vec2f(-237.255, -23.867)), transform(pdf_transform, vec2f(-254.274, -40.936)));
    path.line_to(transform(pdf_transform, vec2f(-284.183, -70.845)));
    path.line_to(transform(pdf_transform, vec2f(-254.274, -100.703)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-237.255, -117.771)), transform(pdf_transform, vec2f(-218.072, -131.417)), transform(pdf_transform, vec2f(-197.629, -141.688)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-192.644, -144.155)), transform(pdf_transform, vec2f(-187.609, -146.472)), transform(pdf_transform, vec2f(-182.524, -148.537)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-178.445, -150.248)), transform(pdf_transform, vec2f(-174.367, -151.759)), transform(pdf_transform, vec2f(-170.238, -153.169)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-165.253, -154.881)), transform(pdf_transform, vec2f(-160.218, -156.392)), transform(pdf_transform, vec2f(-155.133, -157.7)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-136.704, -162.584)), transform(pdf_transform, vec2f(-117.722, -165.001)), transform(pdf_transform, vec2f(-98.789, -165.001)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-70.341, -165.001)), transform(pdf_transform, vec2f(-41.842, -159.513)), transform(pdf_transform, vec2f(-15.105, -148.537)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-10.021, -146.472 )), transform(pdf_transform, vec2f(-4.985, -144.155)), transform(pdf_transform, vec2f(0., -141.688)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(4.128, -139.574)), transform(pdf_transform, vec2f(8.258, -137.358)), transform(pdf_transform, vec2f( 12.285, -134.992)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(17.421, -132.021)), transform(pdf_transform, vec2f(22.457, -128.748)), transform(pdf_transform, vec2f( 27.391, -125.324)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(37.663, -118.074)), transform(pdf_transform, vec2f(47.481, -109.866 )), transform(pdf_transform, vec2f(56.645, -100.703)));
    path.line_to(transform(pdf_transform, vec2f(86.554, -70.845)));
    path.close_path();
    canvas.fill_path(path, FillRule::Winding);

    // Line 392
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 2050.647, 0., 1., 482.7124);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(0., -17.673)), transform(pdf_transform, vec2f(-7.149, -33.685)), transform(pdf_transform, vec2f(-18.781, -45.215)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-30.311, -56.745)), transform(pdf_transform, vec2f(-46.272, -63.896)), transform(pdf_transform, vec2f(-63.845, -63.896)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-99.141, -63.896)), transform(pdf_transform, vec2f(-127.741, -35.245)), transform(pdf_transform, vec2f(-127.741, 0.)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-127.741, 35.297)), transform(pdf_transform, vec2f(-99.141, 63.947)), transform(pdf_transform, vec2f(-63.845, 63.947)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-46.272, 63.947)), transform(pdf_transform, vec2f( -30.311, 56.797)), transform(pdf_transform, vec2f(-18.781, 45.267)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-7.149, 33.736)), transform(pdf_transform, vec2f(0., 17.725)), transform(pdf_transform, vec2f(0., 0.)));
    path.close_path();
    canvas.fill_path(path, FillRule::Winding);

    // Line 402
    canvas.set_fill_style(rgbau(255, 255, 255, 255));
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 2001.4078, 0., 1., 453.3931);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(-3.16, -1.053)), transform(pdf_transform, vec2f(-6.55, -1.646)), transform(pdf_transform, vec2f(-10.072, -1.646)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-27.221, -1.646)), transform(pdf_transform, vec2f(-41.079, 12.213)), transform(pdf_transform, vec2f(-41.079, 29.328)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-41.079, 46.478)), transform(pdf_transform, vec2f(-27.221, 60.335)), transform(pdf_transform, vec2f(-10.072, 60.335)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-6.55, 60.335)), transform(pdf_transform, vec2f(-3.16, 59.775)), transform(pdf_transform, vec2f( 0., 58.689)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-12.212, 54.509)), transform(pdf_transform, vec2f(-21., 42.956)), transform(pdf_transform, vec2f(-21., 29.328)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-21., 15.734)), transform(pdf_transform, vec2f(-12.212, 4.181)), transform(pdf_transform, vec2f(0., 0.)));
    path.close_path();
    canvas.fill_path(path, FillRule::Winding);

    // Line 413
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 822.9351, 0., 1., 624.1802);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(0., -19.728)), transform(pdf_transform, vec2f(-15.992, -35.72)), transform(pdf_transform, vec2f(-35.72, -35.72)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-55.447, -35.72)), transform(pdf_transform, vec2f(-71.439, -19.728)), transform(pdf_transform, vec2f(-71.439, 0.)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-71.439, 19.728)), transform(pdf_transform, vec2f(-55.447, 35.72)), transform(pdf_transform, vec2f(-35.72, 35.72)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-15.992, 35.72)), transform(pdf_transform, vec2f(0., 19.728)), transform(pdf_transform, vec2f(0., 0.)));
    path.close_path();
    canvas.fill_path(path, FillRule::Winding);

    // Line 421
    canvas.set_stroke_style(rgbau((0.137 * 255.) as u8, (0.122 * 255.) as u8, (0.125 * 255.) as u8, 255));
    canvas.set_line_width(6. * viewport_scale);
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 822.9351, 0., 1., 624.1802);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(0., -19.728)), transform(pdf_transform, vec2f(-15.992, -35.72)), transform(pdf_transform, vec2f(-35.72, -35.72)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-55.447, -35.72)), transform(pdf_transform, vec2f(-71.439, -19.728)), transform(pdf_transform, vec2f(-71.439, 0.)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-71.439, 19.728)), transform(pdf_transform, vec2f(-55.447, 35.72)), transform(pdf_transform, vec2f(-35.72, 35.72)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-15.992, 35.72)), transform(pdf_transform, vec2f(0., 19.728)), transform(pdf_transform, vec2f(0., 0.)));
    path.close_path();
    canvas.stroke_path(path);

   // Line 432
   let mut path = Path2D::new();
   let base_transform = Transform2F::row_major(1., 0., 849.9351, 0., 1., 624.1802);
   let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
   let v = transform(pdf_transform, vec2f(0., 0.));
   path.move_to(v);
   path.bezier_curve_to(transform(pdf_transform, vec2f(0., -19.728)), transform(pdf_transform, vec2f(-15.992, -35.72)), transform(pdf_transform, vec2f(-35.72, -35.72)));
   path.bezier_curve_to(transform(pdf_transform, vec2f(-55.447, -35.72)), transform(pdf_transform, vec2f(-71.439, -19.728)), transform(pdf_transform, vec2f(-71.439, 0.)));
   path.bezier_curve_to(transform(pdf_transform, vec2f(-71.439, 19.728)), transform(pdf_transform, vec2f(-55.447, 35.72)), transform(pdf_transform, vec2f(-35.72, 35.72)));
   path.bezier_curve_to(transform(pdf_transform, vec2f(-15.992, 35.72)), transform(pdf_transform, vec2f(0., 19.728)), transform(pdf_transform, vec2f(0., 0.)));
   path.close_path();
   canvas.fill_path(path, FillRule::Winding);

    // Line 440
    let mut path = Path2D::new();
    let base_transform = Transform2F::row_major(1., 0., 849.9351, 0., 1., 624.1802);
    let pdf_transform = create_pdf_transform(viewport_height, viewport_scale, base_transform);
    let v = transform(pdf_transform, vec2f(0., 0.));
    path.move_to(v);
    path.bezier_curve_to(transform(pdf_transform, vec2f(0., -19.728)), transform(pdf_transform, vec2f(-15.992, -35.72)), transform(pdf_transform, vec2f(-35.72, -35.72)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-55.447, -35.72)), transform(pdf_transform, vec2f(-71.439, -19.728)), transform(pdf_transform, vec2f(-71.439, 0.)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-71.439, 19.728)), transform(pdf_transform, vec2f(-55.447, 35.72)), transform(pdf_transform, vec2f(-35.72, 35.72)));
    path.bezier_curve_to(transform(pdf_transform, vec2f(-15.992, 35.72)), transform(pdf_transform, vec2f(0., 19.728)), transform(pdf_transform, vec2f(0., 0.)));
    path.close_path();
    canvas.stroke_path(path);
*/

//    path.bezier_curve_to(transform(pdf_transform, vec2f()), transform(pdf_transform, vec2f()), transform(pdf_transform, vec2f()));
//    path.line_to(transform(pdf_transform, vec2f()));


    // let v1 = transform(pdf_transform, vec2f(613.787-178.328, 530.183-243.077));
    // println!("v1: {:?}", v1);
    // //let v2 = transform(pdf_transform, vec2f(-178.328, -243.077));
    // let v2 = transform(pdf_transform, vec2f(613.787, 530.183));
    // println!("v2: {:?}", v2);
    // path.rect(RectF::new(v1, v2));
    //panic!("ST");
    //canvas.stroke_path(path);
/*
    // // Set line width.
    canvas.set_line_width(10.0);
    canvas.set_line_cap(LineCap::Round);
    canvas.set_stroke_style(rgbau(200, 0, 0, 200));
    let mut path = Path2D::new();
    path.move_to(vec2f(50.0, 100.0));
    path.line_to(vec2f(150.0, 750.0));
    //path.line_to(vec2f(150.0, 250.0));
    //path.line_to(vec2f(150.0, 140.0));
    //path.close_path();
    canvas.set_line_cap(LineCap::Round);
    canvas.stroke_path(path);

    canvas.set_fill_style(rgbau(0, 250, 0, 250));
    let mut clip_path = Path2D::new();
    clip_path.rect(RectF::new(vec2f(0.0, 300.0), vec2f(150.0, 450.0)));
    //canvas.clip_path(clip_path, FillRule::Winding);
    canvas.fill_path(clip_path, FillRule::Winding);
 */

    // let mut path2 = Path2D::new();
    // path2.rect(RectF::new(vec2f(50.0, 175.0), vec2f(150.0, 200.0)));
    // path2.move_to(vec2f(150.0, 100.0));
    // path2.line_to(vec2f(50.0, 250.0));
    // //path.line_to(vec2f(150.0, 140.0));
    // //path.close_path();
    // //canvas.set_line_cap(LineCap::Round);
    // canvas.clip_path(path2, FillRule::Winding);

    // // Draw walls.
    //canvas.stroke_rect(RectF::new(vec2f(75.0, 140.0), vec2f(150.0, 110.0)));

    // // Draw door.
    // canvas.fill_rect(RectF::new(vec2f(130.0, 190.0), vec2f(40.0, 60.0)));

    // // Draw roof.
    // let mut path = Path2D::new();
    // path.arc(vec2f(180., 180.), 160., start_angle: f32, end_angle: f32, direction: ArcDirection)
    // path.move_to(vec2f(50.0, 140.0));
    // path.line_to(vec2f(150.0, 60.0));
    // path.line_to(vec2f(250.0, 140.0));
    // path.close_path();
    // canvas.stroke_path(path);


    // canvas.set_global_composite_operation(CompositeOperation::SourceAtop);

    // // background
    // canvas.set_fill_style(rgbau(0, 0, 0, 255));
    // canvas.fill_rect(RectF::new(vec2f(0., 0.), vec2f(640.0, 480.0)));

    // // Draw circle
    // let from = vec2f(180., 180.);
    // let radii = F32x2::new(0.0, 180.);
    // let mut gradient = Gradient::radial(from, radii);
    // gradient.add_color_stop(rgbau(255, 255, 255, 255), 0.);
    // gradient.add_color_stop(rgbau(255, 111, 63, 255), 1.);
    // canvas.set_fill_style(gradient);

    // pub const PI: f32 = 3.14159265358979323846264338327950288_f32;
    // const PI_2: f32 = PI * 2.0;

    // let mut path = Path2D::new();
    // path.arc(vec2f(180., 180.), 160., 0., PI_2, ArcDirection::CW);
    // path.close_path();
    // canvas.fill_path(path, FillRule::Winding);

    // // Draw rounded-corner rect
    // canvas.set_global_composite_operation(CompositeOperation::DestinationAtop);
    // let line = LineSegment2F::new(vec2f(195., 195.), vec2f(470., 470.));
    // let mut gradient = Gradient::linear(line);
    // gradient.add_color_stop(rgbau(255, 255, 255, 255), 0.);
    // gradient.add_color_stop(rgbau(63, 159, 255, 255), 1.);
    // canvas.set_fill_style(gradient);

    // let path = create_rounded_rect_path(RectF::new(vec2f(195., 195.), vec2f(270., 270.)), 25.);
    // canvas.set_global_composite_operation(CompositeOperation::DestinationAtop);
    // canvas.fill_path(path, FillRule::Winding);


    let duration = start.elapsed();
    println!("8: Time elapsed is: {:?}", duration);

    // Render the canvas to screen.
    let mut scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(),
                                           renderer.mode().level,
                                           SequentialExecutor); //RayonExecutor);
    
    scene.build_and_render(&mut renderer, BuildOptions::default());

    window.gl_swap_window();

    let duration = start.elapsed();
    println!("9: Time elapsed is: {:?}", duration);

    // Wait for a keypress.
    let mut pump_run_once = false;
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        match event_pump.wait_event() {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
            _ => {}
        }

    if pump_run_once == false {
            let duration = start.elapsed();
            println!("9: Time elapsed is: {:?}", duration);
            pump_run_once = true;
        }
    }

    // // Present the surface.
    // let mut surface = device.unbind_surface_from_context(&mut context).unwrap().unwrap();
    // device.present_surface(&mut context, &mut surface).unwrap();
    // device.bind_surface_to_context(&mut context, surface).unwrap();

    // // Wait for a keypress.
    // event_loop.run_forever(|event| {
    //     match event {
    //         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } |
    //         Event::WindowEvent { event: WindowEvent::KeyboardInput { .. }, .. } => {
    //             ControlFlow::Break
    //         }
    //         _ => ControlFlow::Continue,
    //     }
    // });

    // // Clean up.
    // drop(device.destroy_context(&mut context));
}

fn create_rounded_rect_path(rect: RectF, radius: f32) -> Path2D {
    let mut path = Path2D::new();
    path.move_to(rect.origin() + vec2f(radius, 0.0));
    path.arc_to(rect.upper_right(), rect.upper_right() + vec2f(0.0,  radius), radius);
    path.arc_to(rect.lower_right(), rect.lower_right() + vec2f(-radius, 0.0), radius);
    path.arc_to(rect.lower_left(),  rect.lower_left()  + vec2f(0.0, -radius), radius);
    path.arc_to(rect.origin(),      rect.origin()      + vec2f(radius,  0.0), radius);
    path.close_path();
    path
}

fn flip_y_axis(viewport_height: f32, vector: Vector2F) -> Vector2F {
    let transform = Transform2F::from_y_axis_flip(viewport_height);
    transform * vector
}

fn transform(transform: Transform2F, vector: Vector2F) -> Vector2F {
    transform * vector
}

fn create_pdf_transform(viewport_height: f32, scale: f32, base_transform: Transform2F) -> Transform2F {
    
    // let scale_transform = Transform2F {
    //     matrix: Matrix2x2F(F32x4::new(scale / 3.0, 0.0, 0.0, scale)),
    //     vector: Vector2F::zero(),
    // };

    let scale_transform = Transform2F::from_scale(scale);
    let flip_y_axis_transform = Transform2F::from_y_axis_flip(viewport_height);
    //let base_transform = Transform2F::row_major(1., 0., 341.0606, 0., 1., 532.2056);

    println!("s: {:?}", scale_transform);
    println!("f: {:?}", flip_y_axis_transform);
    println!("b: {:?}", base_transform);

    scale_transform * flip_y_axis_transform * base_transform
}

fn test_pdf_transform() {
    let viewport_width = 2289.706;
    let viewport_height = 909.151;
    let viewport_scale = 0.5;
    let base_transform = Transform2F::row_major(1., 0., 341.0606, 0., 1., 532.2056);
    let pdf_t = create_pdf_transform(viewport_height, viewport_scale, base_transform);

    println!("pdf_transform: {:?}", pdf_t);

    let vec_in = vec2f(0., 0.);
    let result = transform(pdf_t, vec_in);
    println!("vec_in: {:?}", vec_in);
    println!("result: {:?}", result);

    println!("-----------------------------------------------------------");

    let vec_in = vec2f(0., -227.586);
    let result = transform(pdf_t, vec_in);
    println!("vec_in: {:?}", vec_in);
    println!("result: {:?}", result);
    println!("-----------------------------------------------------------");

    let vec_in = vec2f(-153.374, -227.586);
    let result = transform(pdf_t, vec_in);
    println!("vec_in: {:?}", vec_in);
    println!("result: {:?}", result);
    println!("-----------------------------------------------------------");


    let v = transform(pdf_t, vec2f(0., 0.));
    println!("v: {:?}", v);
    //path.move_to(v);
    let v = transform(pdf_t, vec2f(0., -227.586));
    println!("v: {:?}", v);
    //path.line_to(v);
    let v = transform(pdf_t, vec2f(-153.374, -227.586));
    println!("v: {:?}", v);


    panic!("STOPPING");

}

fn test_transform() {
    let viewport_height = 909.151/2.0;
    let viewport_width = 2289.706/2.0;
    let base_transform = Transform2F::row_major(1., 0., 341.0606, 0., 1., 532.2056);
    let scale_tranform = Transform2F::from_scale(0.5);
    let flip_y_axis_transform = Transform2F::from_y_axis_flip(viewport_height);

    let t1 = vec2f(100., 808.);
    let t1 = vec2f(0., 0.);
    //let t1 = vec2f(-153.374, -227.586);
    let result = (scale_tranform * flip_y_axis_transform * base_transform) * t1;

    println!("b: {:?}", base_transform);
    println!("s: {:?}", scale_tranform);
    println!("f: {:?}", flip_y_axis_transform);
    println!("resulting t: {:?}", scale_tranform * flip_y_axis_transform * base_transform);
    println!("in: {:?}", t1);
    println!("out: {:?}", result);

    println!("---------------------------------------------------");

    let viewport_height = 500.0;
    let viewport_width = 800.0;
    let base_transform = Transform2F::row_major(1., 0., 200.0, 0., 1., 300.0);
    let scale_tranform = Transform2F::from_scale(0.5);
    let flip_y_axis_transform = Transform2F::from_y_axis_flip(viewport_height);

    let t1 = vec2f(100., 100.);
    let result = (scale_tranform * flip_y_axis_transform * base_transform) * t1;

    println!("b: {:?}", base_transform);
    println!("s: {:?}", scale_tranform);
    println!("f: {:?}", flip_y_axis_transform);
    println!("resulting t: {:?}", scale_tranform * flip_y_axis_transform * base_transform);
    println!("in: {:?}", t1);
    println!("out: {:?}", result);

    panic!("STOPPING");
}