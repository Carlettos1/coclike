use bevy::prelude::*;

// Rectangle sizes
pub const DEFAULT_SIZE: Vec2 = Vec2::new(1.0, 1.0);
pub const TOWNHALL_SIZE: Vec2 = Vec2::new(4.0, 4.0);
pub const COLLECTOR_SIZE: Vec2 = Vec2::new(3.0, 3.0);
pub const STORAGE_SIZE: Vec2 = Vec2::new(4.0, 4.0);
pub const DEFENSE_SIZE: Vec2 = Vec2::new(3.0, 3.0);
pub const WALL_SIZE: Vec2 = Vec2::new(1.0, 1.0);

// Colors
pub const TOWNHALL_COLOR: Color = Color::linear_rgb(0.0, 1.0, 0.0);
pub const ELIXIR_COLOR: Color = Color::linear_rgb(1.0, 0.0, 1.0);
pub const GOLD_COLOR: Color = Color::linear_rgb(1.0, 0.84, 0.0);
pub const DEFENSE_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);
pub const WALL_COLOR: Color = Color::linear_rgb(0.6, 0.3, 0.2);

pub const BROWN: Color = Color::linear_rgb(0.65, 0.33, 0.16);
pub const BROWN_SATURATED: Color = Color::linear_rgb(0.75, 0.30, 0.10);
pub const BROWN_DESATURATED: Color = Color::linear_rgb(0.55, 0.40, 0.25);

pub const BLACK: Color = Color::linear_rgb(0.0, 0.0, 0.0);
pub const DARK_GRAY: Color = Color::linear_rgb(0.25, 0.25, 0.25);
pub const GRAY: Color = Color::linear_rgb(0.5, 0.5, 0.5);
pub const LIGHT_GRAY: Color = Color::linear_rgb(0.75, 0.75, 0.75);
pub const WHITE: Color = Color::linear_rgb(1.0, 1.0, 1.0);

// Constants for building stats by level
pub const TOWNHALL_HEALTH: [f32; 10] = [
    1000.0, 1200.0, 1500.0, 1800.0, 2200.0, 2600.0, 3000.0, 3500.0, 4000.0, 5000.0,
];
pub const COLLECTOR_HEALTH: [f32; 10] = [
    400.0, 450.0, 500.0, 550.0, 600.0, 650.0, 700.0, 750.0, 800.0, 900.0,
];
pub const COLLECTOR_RATE: [f32; 10] = [5.0, 7.0, 9.0, 12.0, 15.0, 18.0, 22.0, 26.0, 30.0, 35.0];
pub const STORAGE_HEALTH: [f32; 10] = [
    600.0, 700.0, 800.0, 900.0, 1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0,
];
pub const STORAGE_CAPACITY: [u32; 10] = [
    5000, 10000, 15000, 20000, 25000, 30000, 40000, 50000, 75000, 100000,
];
pub const DEFENSE_HEALTH: [f32; 10] = [
    800.0, 900.0, 1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0, 1750.0, 2000.0,
];
pub const DEFENSE_DAMAGE: [f32; 10] = [20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 70.0];
pub const DEFENSE_RANGE: [f32; 10] = [5.0, 5.5, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5, 9.0, 10.0];
pub const DEFENSE_SPEED: [f32; 10] = [1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 2.0];
pub const WALL_HEALTH: [f32; 10] = [
    300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0, 1200.0, 1500.0,
];
