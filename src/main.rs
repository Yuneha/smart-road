mod sim;
use sim::{
    roads::{ Direction, Line, Road, RoadDirection },
    vehicles_management::VehiclesManagement,
};
use sdl2::{
    image::{ InitFlag, LoadTexture },
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{ Texture, TextureQuery },
    video::WindowContext,
};
use std::{ path::Path, time::Duration };

pub const WINDOW_WIDTH: i32 = 1024;
pub const WINDOW_HEIGHT: i32 = 768;
pub const TILE_SIZE: u32 = 64;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let window = video_subsystem
        .window("Smart Road", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let mut event_pump = sdl_context.event_pump()?;

    // Load texture
    let texture_creator = canvas.texture_creator();
    let tile_texture = load_texture(&texture_creator, "assets/sprites/space_bg.png")?;
    let ship_north_texture = texture_creator.load_texture("assets/sprites/ship_north.png").unwrap();
    let ship_west_texture = texture_creator.load_texture("assets/sprites/ship_west.png").unwrap();
    let ship_south_texture = texture_creator.load_texture("assets/sprites/ship_south.png").unwrap();
    let ship_est_texture = texture_creator.load_texture("assets/sprites/ship_est.png").unwrap();
    // Pause game variable
    let mut paused = false;
    // Create roads
    let mut vertical_roads = Road::new();
    let mut horizontal_roads = Road::new();
    let north_line = vec![
        Line::new(RoadDirection::North, Direction::Left),
        Line::new(RoadDirection::North, Direction::Straight),
        Line::new(RoadDirection::North, Direction::Right)
    ];
    let west_line = vec![
        Line::new(RoadDirection::West, Direction::Left),
        Line::new(RoadDirection::West, Direction::Straight),
        Line::new(RoadDirection::West, Direction::Right)
    ];
    let south_line = vec![
        Line::new(RoadDirection::South, Direction::Left),
        Line::new(RoadDirection::South, Direction::Straight),
        Line::new(RoadDirection::South, Direction::Right)
    ];
    let est_line = vec![
        Line::new(RoadDirection::East, Direction::Left),
        Line::new(RoadDirection::East, Direction::Straight),
        Line::new(RoadDirection::East, Direction::Right)
    ];
    vertical_roads.lines1 = north_line.clone();
    vertical_roads.lines2 = south_line.clone();
    horizontal_roads.lines1 = west_line.clone();
    horizontal_roads.lines2 = est_line.clone();
    // Stock vehicles
    let mut vehicles = VehiclesManagement::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'running;
                }
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    paused = !paused;
                }
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Up), .. } =>
                    vehicles.spawn(&south_line),
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Down), .. } =>
                    vehicles.spawn(&north_line),
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Left), .. } =>
                    vehicles.spawn(&west_line),
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Right), .. } =>
                    vehicles.spawn(&est_line),
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::R), .. } =>
                    vehicles.spawn_random(vec![&north_line, &west_line, &south_line, &est_line]),
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    // Reset
                    vehicles = VehiclesManagement::new();
                }
                _ => {}
            }
        }

        if !paused {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            // Render background
            draw_tiled_background(&mut canvas, &tile_texture);
            // Renders roads
            Road::render(&mut canvas);
            // Render & Update Vehicles
            vehicles.update();
            vehicles.render(
                &mut canvas,
                &[&ship_north_texture, &ship_west_texture, &ship_south_texture, &ship_est_texture]
            );

            canvas.present();
        }
        std::thread::sleep(Duration::from_millis(16));
    }

    // Stats window
    let stats_window = video_subsystem
        .window("Smart Road - Stats", 400, 300)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut stats_canvas = stats_window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    stats_canvas.set_draw_color(Color::RGB(50, 50, 50));
    stats_canvas.clear();
    stats_canvas.present();

    // Load texture for new window
    let texture_creator = stats_canvas.texture_creator();
    let tile_texture = load_texture(&texture_creator, "assets/sprites/space_bg.png")?;
    // Load Font
    let font = ttf_context.load_font("assets/font/arial.ttf", 16)?;

    let stats = vec![
        format!("Number of vehicles spawned: {:?}", vehicles.number_of_vehicles),
        format!(
            "Number of vehicles that passed the intersection: {:?}",
            vehicles.number_passed_intersection
        ),
        format!("Max velocity: {:?} pixel(s)", vehicles.max_velocity),
        format!("Min velocity: {:?} pixel(s)", vehicles.min_velocity),
        format!(
            "Vehicle pass intersection Max time: {:?}s",
            round_to_tenth_second(vehicles.max_time)
        ),
        format!(
            "Vehicle pass intersection Min time: {:?}s",
            round_to_tenth_second(vehicles.min_time)
        ),
        format!("Close calls: {:?}", vehicles.close_call)
    ];

    'Stats_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'Stats_loop;
                }
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'Stats_loop;
                }
                _ => {}
            }
        }

        // Render background
        draw_tiled_background(&mut stats_canvas, &tile_texture);

        // Render stats
        let mut y_offset = 10;
        for stat in &stats {
            // Render the text to a surface
            let surface = font
                .render(stat)
                .blended(Color::RGB(255, 255, 255))
                .map_err(|e| e.to_string())?;

            let texture_creator = stats_canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            // Get the size of the rendered text
            let TextureQuery { width, height, .. } = texture.query();

            // Copy the texture to the canvas
            stats_canvas.copy(&texture, None, Rect::new(10, y_offset, width, height))?;

            // Increment the y_offset for the next line
            y_offset += (height as i32) + 5;
        }

        stats_canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn load_texture<'a>(
    texture_creator: &'a sdl2::render::TextureCreator<WindowContext>,
    file_path: &str
) -> Result<Texture<'a>, String> {
    texture_creator.load_texture(Path::new(file_path))
}

fn draw_tiled_background(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &Texture
) {
    for y in (0..WINDOW_HEIGHT).step_by(TILE_SIZE as usize) {
        for x in (0..WINDOW_WIDTH).step_by(TILE_SIZE as usize) {
            let dest_rect = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
            canvas.copy(texture, None, dest_rect).unwrap();
        }
    }
}

pub fn round_to_tenth_second(duration: Duration) -> f64 {
    (duration.as_secs_f64() * 10.0).round() / 10.0
}
