use std::{ collections::HashMap, time::{ Duration, Instant } };
use sdl2::{ rect::Rect, render::{ Canvas, Texture }, video::Window };
use rand::Rng;
use crate::{ WINDOW_HEIGHT, WINDOW_WIDTH };
use super::{ roads::{ Direction, Line, RoadDirection, RoadIntersection }, vehicle::Vehicle };

pub const NORMAL_VELOCITY: i32 = 2;
pub const SLOW_VELOCITY: i32 = 1;
pub const STOP_VELOCITY: i32 = 0;
pub const FAST_VELOCITY: i32 = 3;
pub const SAFE_DISTANCE: i32 = 10;

const SPAWN_COOLDOWN: Duration = Duration::from_millis(500);

#[derive(Debug)]
pub struct VehiclesManagement {
    pub list: Vec<Vehicle>,
    last_spawn_time: Option<Instant>,
    pub intersection_list: Vec<i32>,
    pub intersection: RoadIntersection,
    pub number_of_vehicles: i32,
    pub number_passed_intersection: i32,
    pub max_velocity: i32,
    pub min_velocity: i32,
    pub max_time: Duration,
    pub min_time: Duration,
    pub close_call: usize,
}

impl VehiclesManagement {
    pub fn new() -> Self {
        VehiclesManagement {
            list: vec![],
            last_spawn_time: None,
            intersection_list: Vec::new(),
            intersection: RoadIntersection::new(362, 234, 300, 300),
            number_of_vehicles: 0,
            number_passed_intersection: 0,
            max_velocity: 0,
            min_velocity: 0,
            max_time: Duration::from_secs(0),
            min_time: Duration::from_secs(0),
            close_call: 0,
        }
    }

    pub fn spawn(&mut self, lines: &Vec<Line>) {
        if let Some(last_spawn) = self.last_spawn_time {
            if last_spawn.elapsed() < SPAWN_COOLDOWN {
                return; // Cooldown not yet complete, don't spawn
            }
        }

        let random_road = self.random_roads(&lines);
        // Create hashmap with all spawn
        let position_map: HashMap<(RoadDirection, Direction), (i32, i32)> = [
            // North
            ((RoadDirection::North, Direction::Left), (462, 0)),
            ((RoadDirection::North, Direction::Straight), (412, 0)),
            ((RoadDirection::North, Direction::Right), (362, 0)),
            // West
            ((RoadDirection::West, Direction::Left), (WINDOW_WIDTH, 334)),
            ((RoadDirection::West, Direction::Straight), (WINDOW_WIDTH, 284)),
            ((RoadDirection::West, Direction::Right), (WINDOW_WIDTH, 234)),
            // South
            ((RoadDirection::South, Direction::Left), (512, WINDOW_HEIGHT)),
            ((RoadDirection::South, Direction::Straight), (562, WINDOW_HEIGHT)),
            ((RoadDirection::South, Direction::Right), (612, WINDOW_HEIGHT)),
            // Est
            ((RoadDirection::East, Direction::Left), (0, 384)),
            ((RoadDirection::East, Direction::Straight), (0, 434)),
            ((RoadDirection::East, Direction::Right), (0, 484)),
        ]
            .iter()
            .cloned()
            .collect();
        // Get the vehicle spawn
        if
            let Some(&(x, y)) = position_map.get(
                &(random_road.road_direction, random_road.direction)
            )
        {
            if !self.check_spawn(x, y) {
                self.list.push(
                    Vehicle::new(
                        self.number_of_vehicles,
                        random_road.road_direction,
                        random_road.direction,
                        x,
                        y
                    )
                );
                self.number_of_vehicles += 1;
                self.last_spawn_time = Some(Instant::now());
                return;
            }
        }
    }

    pub fn spawn_random(&mut self, lines: Vec<&Vec<Line>>) {
        let mut rng = rand::thread_rng();
        let rand_road_direction = rng.gen_range(0..lines.len());
        self.spawn(lines[rand_road_direction]);
    }

    fn random_roads(&self, lines: &Vec<Line>) -> Line {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0..lines.len());
        lines[rand]
    }

    pub fn update(&mut self) {
        // Update list of vehicle in intersection
        for vehicle in self.list.iter_mut() {
            if vehicle.is_in_intersection(&self.intersection) {
                if !self.intersection_list.contains(&vehicle.id) {
                    vehicle.intersection_entry_time = Some(Instant::now());
                    self.intersection_list.push(vehicle.id);
                }
            } else {
                if let Some(entry_time) = vehicle.intersection_entry_time.take() {
                    let mut time_in_intersection = entry_time.elapsed();
                    if time_in_intersection < Duration::from_millis(50) {
                        time_in_intersection = Duration::from_millis(400);
                    }
                    if self.max_time < time_in_intersection {
                        self.max_time = time_in_intersection;
                    }
                    if self.min_time == Duration::from_secs(0) {
                        self.min_time = time_in_intersection;
                    } else if
                        self.min_time > time_in_intersection &&
                        time_in_intersection > Duration::from_millis(50)
                    {
                        self.min_time = time_in_intersection;
                    }
                    self.number_passed_intersection += 1;
                }
                self.intersection_list.retain(|id| *id != vehicle.id);
            }
        }

        self.check_collision();

        self.update_position();

        // Remove vehicles if outside the screen
        self.list.retain(|vehicle| {
            vehicle.x >= -50 &&
                vehicle.x <= WINDOW_WIDTH &&
                vehicle.y >= -50 &&
                vehicle.y <= WINDOW_HEIGHT
        });
    }

    fn update_position(&mut self) {
        for vehicle in &mut self.list {
            // Update Position
            match vehicle.road_direction {
                RoadDirection::North => {
                    vehicle.y += vehicle.velocity;
                }
                RoadDirection::South => {
                    vehicle.y -= vehicle.velocity;
                }
                RoadDirection::West => {
                    vehicle.x -= vehicle.velocity;
                }
                RoadDirection::East => {
                    vehicle.x += vehicle.velocity;
                }
            }

            match (vehicle.road_direction, vehicle.direction) {
                (RoadDirection::North, Direction::Left) if vehicle.y >= 384 => {
                    vehicle.road_direction = RoadDirection::East;
                    vehicle.direction = Direction::Straight;
                }
                (RoadDirection::North, Direction::Right) if vehicle.y >= 234 => {
                    vehicle.road_direction = RoadDirection::West;
                    vehicle.direction = Direction::Straight;
                }
                (RoadDirection::West, Direction::Right) if vehicle.x <= 612 => {
                    vehicle.road_direction = RoadDirection::South;
                    vehicle.direction = Direction::Straight;
                }
                (RoadDirection::West, Direction::Left) if vehicle.x <= 462 => {
                    vehicle.road_direction = RoadDirection::North;
                    vehicle.direction = Direction::Straight;
                }
                (RoadDirection::South, Direction::Right) if vehicle.y <= 484 => {
                    vehicle.road_direction = RoadDirection::East;
                    vehicle.direction = Direction::Straight;
                }
                (RoadDirection::South, Direction::Left) if vehicle.y <= 334 => {
                    vehicle.road_direction = RoadDirection::West;
                    vehicle.direction = Direction::Straight;
                }
                (RoadDirection::East, Direction::Right) if vehicle.x >= 362 => {
                    vehicle.road_direction = RoadDirection::North;
                    vehicle.direction = Direction::Straight;
                }
                (RoadDirection::East, Direction::Left) if vehicle.x >= 512 => {
                    vehicle.road_direction = RoadDirection::South;
                    vehicle.direction = Direction::Straight;
                }
                _ => {}
            }
        }
    }

    fn check_spawn(&self, x: i32, y: i32) -> bool {
        for vehicle in &self.list {
            // Check vertical spawn
            if
                (x - vehicle.x).abs() < 25 &&
                (y - vehicle.y).abs() < 50 + SAFE_DISTANCE * 2 &&
                (vehicle.road_direction == RoadDirection::North ||
                    vehicle.road_direction == RoadDirection::South)
            {
                return true;
            }
            // Check horizontal spawn
            if
                (x - vehicle.x).abs() < 50 + SAFE_DISTANCE * 2 &&
                (y - vehicle.y).abs() < 25 &&
                (vehicle.road_direction == RoadDirection::West ||
                    vehicle.road_direction == RoadDirection::East)
            {
                return true;
            }
        }
        false
    }

    fn check_collision(&mut self) {
        let len = self.list.len();

        for i in 0..len {
            let mut should_stop = false;
            let mut should_slow = false;
            let mut is_ahead_clear = false;

            for j in 0..len {
                if i != j && self.list[i].has_to_slow(&self.list[j]) {
                    should_slow = true;
                    should_stop = false;
                    is_ahead_clear = false;
                    break;
                }
            }

            for j in 0..len {
                if i != j && self.list[i].will_collide(&self.list[j]) {
                    should_stop = true;
                    if !self.list[i].has_stop {
                        self.close_call += 1;
                    }
                    self.list[i].has_stop = true;
                    should_slow = false;
                    is_ahead_clear = false;
                    break;
                }
            }

            if self.list[i].is_ahead_clear(&self.list) {
                should_stop = false;
                should_slow = false;
                is_ahead_clear = true;
            }

            if self.list[i].is_front_clear(&self.list) {
                should_stop = false;
            }

            if should_stop {
                self.list[i].velocity = STOP_VELOCITY;
                self.list[i].has_stop = true;
            } else if should_slow {
                self.list[i].velocity = SLOW_VELOCITY;
            } else if is_ahead_clear {
                self.list[i].velocity = FAST_VELOCITY;
            } else {
                self.list[i].velocity = NORMAL_VELOCITY;
            }

            if self.max_velocity < self.list[i].velocity {
                self.max_velocity = self.list[i].velocity;
            }
            if self.min_velocity >= self.list[i].velocity {
                self.min_velocity = self.list[i].velocity;
            }
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, texture: &[&Texture]) {
        for vehicle in &self.list {
            if
                (vehicle.x >= -50 || vehicle.x <= WINDOW_WIDTH) &&
                (vehicle.y >= -50 || vehicle.y <= WINDOW_HEIGHT)
            {
                let dest_rect = Rect::new(vehicle.x, vehicle.y, 50, 50);
                match vehicle.road_direction {
                    RoadDirection::North => {
                        canvas.copy(texture[0], None, dest_rect).unwrap();
                    }
                    RoadDirection::West => {
                        canvas.copy(texture[1], None, dest_rect).unwrap();
                    }
                    RoadDirection::South => {
                        canvas.copy(texture[2], None, dest_rect).unwrap();
                    }
                    RoadDirection::East => {
                        canvas.copy(texture[3], None, dest_rect).unwrap();
                    }
                }
            }
        }
    }
}
