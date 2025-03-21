use std::time::Instant;

use super::{
    roads::{ Direction, RoadDirection, RoadIntersection },
    vehicles_management::{ NORMAL_VELOCITY, SAFE_DISTANCE },
};

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: i32,
    pub road_direction: RoadDirection,
    pub direction: Direction,
    pub x: i32,
    pub y: i32,
    pub velocity: i32,
    pub intersection_entry_time: Option<Instant>,
    pub has_stop: bool,
}

impl Vehicle {
    pub fn new(
        id: i32,
        road_direction: RoadDirection,
        direction: Direction,
        x: i32,
        y: i32
    ) -> Self {
        Vehicle {
            id,
            road_direction,
            direction,
            x,
            y,
            velocity: NORMAL_VELOCITY,
            intersection_entry_time: None,
            has_stop: false,
        }
    }

    pub fn will_collide(&self, other_vehicle: &Vehicle) -> bool {
        let (next_x, next_y) = match self.road_direction {
            RoadDirection::North => (self.x, self.y - self.velocity),
            RoadDirection::South => (self.x, self.y + self.velocity),
            RoadDirection::West => (self.x - self.velocity, self.y),
            RoadDirection::East => (self.x + self.velocity, self.y),
        };

        let y_diff = (next_y - other_vehicle.y).abs();
        let x_diff = (next_x - other_vehicle.x).abs();

        match self.road_direction {
            RoadDirection::North | RoadDirection::South => {
                y_diff < 48 + SAFE_DISTANCE && x_diff < 40
            }
            RoadDirection::East | RoadDirection::West => {
                x_diff < 48 + SAFE_DISTANCE && y_diff < 40
            }
        }
    }

    pub fn has_to_slow(&self, other_vehicle: &Vehicle) -> bool {
        let (next_x, next_y) = match self.road_direction {
            RoadDirection::North => (self.x, self.y),
            RoadDirection::South => (self.x, self.y),
            RoadDirection::West => (self.x, self.y),
            RoadDirection::East => (self.x, self.y),
        };

        let y_diff = (next_y - other_vehicle.y).abs();
        let x_diff = (next_x - other_vehicle.x).abs();

        match self.road_direction {
            RoadDirection::North => {
                y_diff < 0 && y_diff.abs() < 48 + SAFE_DISTANCE * 3 && x_diff.abs() < 40
            }
            RoadDirection::South => {
                y_diff > 0 && y_diff.abs() < 48 + SAFE_DISTANCE * 3 && x_diff.abs() < 40
            }
            RoadDirection::West => {
                x_diff < 0 && x_diff.abs() < 48 + SAFE_DISTANCE * 3 && y_diff.abs() < 40
            }
            RoadDirection::East => {
                x_diff > 0 && x_diff.abs() < 48 + SAFE_DISTANCE * 3 && y_diff.abs() < 40
            }
        }
    }

    pub fn is_in_intersection(&self, intersection: &RoadIntersection) -> bool {
        if
            self.x >= intersection.x &&
            self.x <= intersection.x + intersection.width &&
            self.y >= intersection.y &&
            self.y <= intersection.y + intersection.height
        {
            return true;
        }
        false
    }

    pub fn is_ahead_clear(&self, vehicles: &[Vehicle]) -> bool {
        for vehicle in vehicles {
            if vehicle.id == self.id {
                continue;
            }

            let x_diff = (self.x - vehicle.x).abs();
            let y_diff = (self.y - vehicle.y).abs();

            match self.road_direction {
                RoadDirection::North => {
                    if vehicle.y > self.y && x_diff < 40 {
                        return false;
                    }
                }
                RoadDirection::South => {
                    if vehicle.y < self.y && x_diff < 40 {
                        return false;
                    }
                }
                RoadDirection::West => {
                    if vehicle.x < self.x && y_diff < 40 {
                        return false;
                    }
                }
                RoadDirection::East => {
                    if vehicle.x > self.x && y_diff < 40 {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn is_front_clear(&self, vehicles: &[Vehicle]) -> bool {
        for vehicle in vehicles {
            if vehicle.id == self.id {
                continue;
            }

            let x_diff = (self.x - vehicle.x).abs();
            let y_diff = (self.y - vehicle.y).abs();

            match self.road_direction {
                RoadDirection::North => {
                    if vehicle.y > self.y && y_diff < 48 * 2 && x_diff < 40 {
                        return false;
                    }
                }
                RoadDirection::South => {
                    if vehicle.y < self.y && y_diff < 48 * 2 && x_diff < 40 {
                        return false;
                    }
                }
                RoadDirection::West => {
                    if vehicle.x < self.x && x_diff < 48 * 2 && y_diff < 40 {
                        return false;
                    }
                }
                RoadDirection::East => {
                    if vehicle.x > self.x && x_diff < 48 * 2 && y_diff < 40 {
                        return false;
                    }
                }
            }
        }
        true
    }
}
