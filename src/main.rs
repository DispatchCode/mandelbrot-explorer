use num_complex::Complex;
use std::fs::File;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    display_fractal();
}

fn mandelbrot_fractal(width : usize, height : usize) -> Vec<u8> {
    let mut buffer = vec![0; width * height * 3];
    let mut c = Complex::new(0.0, 0.0);
    let mut z = Complex::new(0.0, 0.0);
    let max_iterations = 1000;
    let escape_radius = 2.0;
    let (x_min, x_max, y_min, y_max) = (-2.0, 1.0, -1.0, 1.0);

    let x_step = (x_max - x_min) / width as f64;
    let y_step = (y_max - y_min) / height as f64;

    for y in 0..height {
        for x in 0..width {
            c.re = x_min + x as f64 * x_step;
            c.im = y_min + y as f64 * y_step;
            z.re = 0.0;
            z.im = 0.0;

            let mut i = 0;
            while i < max_iterations && z.norm_sqr() < escape_radius {
                z = z * z + c;
                i += 1;
            }

            let (blu_a, blu_b)  = (0.0, 255.0);
            let (red_a, red_b)  = (0.0, 255.0);

            let rstepx = (red_b - red_a) / width as f32;
            let bstepy = (blu_b - blu_a) / height as f32;

            let r = (red_a + rstepx*x as f32) as u8;
            let b = (blu_a + bstepy*y as f32) as u8;

            let val_in = 32;
            let val_out = 0;
            let g= if i >= 48 { (255.0 - (((i + val_in) as f32 / (max_iterations + val_in) as f32) * (255 - val_out) as f32 )) as u8} else {0 as u8};

            let atten =  i as f32 / max_iterations as f32;
            let atten = 1.0 - atten*atten;

            let r = (r as f32 * atten) as u8;
            let b = (b as f32 * atten) as u8;

            let index = (y * width + x) * 3;
            buffer[index + 0] = r;
            buffer[index + 1] = g;
            buffer[index + 2] = b;
        }
    }

    let mut file = File::create("../images/mandelbrot.png").unwrap();
    let mut encoder = png::Encoder::new(&mut file, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buffer).unwrap();

    buffer
}

fn display_fractal() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 1920, 1080)
        .position_centered()
        .vulkan()
        .build()

        .unwrap();

    //window.set_fullscreen(sdl2::video::FullscreenType::Desktop);

    let mut canvas = window.into_canvas().build().unwrap();
   // let size = canvas.window().size();
    let (width, height) = (1920, 1080);

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            width as u32,
            height as u32,
        )
        .unwrap();

    let buffer = mandelbrot_fractal(width as usize, height as usize);
    texture.update(None, &buffer, (width * 3) as usize).unwrap();

    canvas.clear();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
}