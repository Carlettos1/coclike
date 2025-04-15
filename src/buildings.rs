//! Here are the building constructors
use crate::prelude::*;
use bevy::prelude::*;

// Constants for building stats by level
const TOWNHALL_HEALTH: [f32; 10] = [
    1000.0, 1200.0, 1500.0, 1800.0, 2200.0, 2600.0, 3000.0, 3500.0, 4000.0, 5000.0,
];
const COLLECTOR_HEALTH: [f32; 10] = [
    400.0, 450.0, 500.0, 550.0, 600.0, 650.0, 700.0, 750.0, 800.0, 900.0,
];
const COLLECTOR_RATE: [f32; 10] = [5.0, 7.0, 9.0, 12.0, 15.0, 18.0, 22.0, 26.0, 30.0, 35.0];
const STORAGE_HEALTH: [f32; 10] = [
    600.0, 700.0, 800.0, 900.0, 1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0,
];
const STORAGE_CAPACITY: [u32; 10] = [
    5000, 10000, 15000, 20000, 25000, 30000, 40000, 50000, 75000, 100000,
];
const DEFENSE_HEALTH: [f32; 10] = [
    800.0, 900.0, 1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0, 1750.0, 2000.0,
];
const DEFENSE_DAMAGE: [f32; 10] = [20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 70.0];
const DEFENSE_RANGE: [f32; 10] = [5.0, 5.5, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5, 9.0, 10.0];
const DEFENSE_SPEED: [f32; 10] = [1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 2.0];
const WALL_HEALTH: [f32; 10] = [
    300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0, 1200.0, 1500.0,
];

const TOWNHALL_SIZE: (usize, usize) = (4, 4);
const COLLECTOR_SIZE: (usize, usize) = (3, 3);
const STORAGE_SIZE: (usize, usize) = (4, 4);
const DEFENSE_SIZE: (usize, usize) = (3, 3);
const WALL_SIZE: (usize, usize) = (1, 1);

// Level check and adjustment
fn validate_level(level: u32) -> usize {
    let idx = (level as usize).saturating_sub(1);
    idx.min(9) // Max level is 10 (index 9)
}

pub fn town_hall(
    commands: &mut Commands,
    assets: &BuildingAssets,
    level: u32,
    x: usize,
    y: usize,
) -> Entity {
    let idx = validate_level(level);
    let health = TOWNHALL_HEALTH[idx];

    commands
        .spawn((
            TownHall,
            Building::new_with_level(level, health),
            GridPosition::new_full(x, y, TOWNHALL_SIZE.0, TOWNHALL_SIZE.1),
            Mesh2d(assets.town_hall.0.clone()),
            MeshMaterial2d(assets.town_hall.1.clone()),
        ))
        .id()
}

pub fn gold_collector(
    commands: &mut Commands,
    assets: &BuildingAssets,
    level: u32,
    x: usize,
    y: usize,
) -> Entity {
    let idx = validate_level(level);
    let health = COLLECTOR_HEALTH[idx];
    let rate = COLLECTOR_RATE[idx];

    commands
        .spawn((
            ResourceCollector::new(ResourceType::Gold, rate),
            Building::new_with_level(level, health),
            GridPosition::new_full(x, y, COLLECTOR_SIZE.0, COLLECTOR_SIZE.1),
            Mesh2d(assets.resource_collector.0.clone()),
            MeshMaterial2d(assets.resource_collector.1.clone()),
        ))
        .id()
}

pub fn elixir_collector(
    commands: &mut Commands,
    assets: &BuildingAssets,
    level: u32,
    x: usize,
    y: usize,
) -> Entity {
    let idx = validate_level(level);
    let health = COLLECTOR_HEALTH[idx];
    let rate = COLLECTOR_RATE[idx];

    commands
        .spawn((
            ResourceCollector::new(ResourceType::Elixir, rate),
            Building::new_with_level(level, health),
            GridPosition::new_full(x, y, COLLECTOR_SIZE.0, COLLECTOR_SIZE.1),
            Mesh2d(assets.resource_collector.0.clone()),
            MeshMaterial2d(assets.resource_collector.1.clone()),
        ))
        .id()
}

pub fn gold_storage(
    commands: &mut Commands,
    assets: &BuildingAssets,
    level: u32,
    x: usize,
    y: usize,
) -> Entity {
    let idx = validate_level(level);
    let health = STORAGE_HEALTH[idx];
    let capacity = STORAGE_CAPACITY[idx];

    commands
        .spawn((
            Storage::new(ResourceType::Gold, capacity),
            Building::new_with_level(level, health),
            GridPosition::new_full(x, y, STORAGE_SIZE.0, STORAGE_SIZE.1),
            Mesh2d(assets.storage.0.clone()),
            MeshMaterial2d(assets.storage.1.clone()),
        ))
        .id()
}

pub fn elixir_storage(
    commands: &mut Commands,
    assets: &BuildingAssets,
    level: u32,
    x: usize,
    y: usize,
) -> Entity {
    let idx = validate_level(level);
    let health = STORAGE_HEALTH[idx];
    let capacity = STORAGE_CAPACITY[idx];

    commands
        .spawn((
            Storage::new(ResourceType::Elixir, capacity),
            Building::new_with_level(level, health),
            GridPosition::new_full(x, y, STORAGE_SIZE.0, STORAGE_SIZE.1),
            Mesh2d(assets.storage.0.clone()),
            MeshMaterial2d(assets.storage.1.clone()),
        ))
        .id()
}

pub fn defense_tower(
    commands: &mut Commands,
    assets: &BuildingAssets,
    level: u32,
    x: usize,
    y: usize,
) -> Entity {
    let idx = validate_level(level);
    let health = DEFENSE_HEALTH[idx];
    let damage = DEFENSE_DAMAGE[idx];
    let range = DEFENSE_RANGE[idx];
    let speed = DEFENSE_SPEED[idx];

    commands
        .spawn((
            Defense::new(damage, range, speed),
            Building::new_with_level(level, health),
            GridPosition::new_full(x, y, DEFENSE_SIZE.0, DEFENSE_SIZE.1),
            Mesh2d(assets.defense.0.clone()),
            MeshMaterial2d(assets.defense.1.clone()),
        ))
        .id()
}

pub fn wall(
    commands: &mut Commands,
    assets: &BuildingAssets,
    level: u32,
    x: usize,
    y: usize,
) -> Entity {
    let idx = validate_level(level);
    let health = WALL_HEALTH[idx];

    commands
        .spawn((
            Building::new_with_level(level, health),
            Wall::new(health),
            GridPosition::new_full(x, y, WALL_SIZE.0, WALL_SIZE.1),
            Mesh2d(assets.wall.0.clone()),
            MeshMaterial2d(assets.wall.1.clone()),
        ))
        .id()
}

// Function for custom positioning of any building
pub fn place_building(
    commands: &mut Commands,
    assets: &BuildingAssets,
    building_type: &str,
    level: u32,
    position: (usize, usize),
) -> Option<Entity> {
    match building_type {
        "townhall" => {
            let idx = validate_level(level);
            let health = TOWNHALL_HEALTH[idx];

            Some(
                commands
                    .spawn((
                        TownHall,
                        Building::new_with_level(level, health),
                        GridPosition::new_full(
                            position.0,
                            position.1,
                            TOWNHALL_SIZE.0,
                            TOWNHALL_SIZE.1,
                        ),
                        Mesh2d(assets.town_hall.0.clone()),
                        MeshMaterial2d(assets.town_hall.1.clone()),
                    ))
                    .id(),
            )
        }
        "gold_collector" => {
            let idx = validate_level(level);
            let health = COLLECTOR_HEALTH[idx];
            let rate = COLLECTOR_RATE[idx];

            Some(
                commands
                    .spawn((
                        ResourceCollector::new(ResourceType::Gold, rate),
                        Building::new_with_level(level, health),
                        GridPosition::new_full(
                            position.0,
                            position.1,
                            COLLECTOR_SIZE.0,
                            COLLECTOR_SIZE.1,
                        ),
                        Mesh2d(assets.resource_collector.0.clone()),
                        MeshMaterial2d(assets.resource_collector.1.clone()),
                    ))
                    .id(),
            )
        }
        "elixir_collector" => {
            let idx = validate_level(level);
            let health = COLLECTOR_HEALTH[idx];
            let rate = COLLECTOR_RATE[idx];

            Some(
                commands
                    .spawn((
                        ResourceCollector::new(ResourceType::Elixir, rate),
                        Building::new_with_level(level, health),
                        GridPosition::new_full(
                            position.0,
                            position.1,
                            COLLECTOR_SIZE.0,
                            COLLECTOR_SIZE.1,
                        ),
                        Mesh2d(assets.resource_collector.0.clone()),
                        MeshMaterial2d(assets.resource_collector.1.clone()),
                    ))
                    .id(),
            )
        }
        "gold_storage" => {
            let idx = validate_level(level);
            let health = STORAGE_HEALTH[idx];
            let capacity = STORAGE_CAPACITY[idx];

            Some(
                commands
                    .spawn((
                        Storage::new(ResourceType::Gold, capacity),
                        Building::new_with_level(level, health),
                        GridPosition::new_full(
                            position.0,
                            position.1,
                            STORAGE_SIZE.0,
                            STORAGE_SIZE.1,
                        ),
                        Mesh2d(assets.storage.0.clone()),
                        MeshMaterial2d(assets.storage.1.clone()),
                    ))
                    .id(),
            )
        }
        "elixir_storage" => {
            let idx = validate_level(level);
            let health = STORAGE_HEALTH[idx];
            let capacity = STORAGE_CAPACITY[idx];

            Some(
                commands
                    .spawn((
                        Storage::new(ResourceType::Elixir, capacity),
                        Building::new_with_level(level, health),
                        GridPosition::new_full(
                            position.0,
                            position.1,
                            STORAGE_SIZE.0,
                            STORAGE_SIZE.1,
                        ),
                        Mesh2d(assets.storage.0.clone()),
                        MeshMaterial2d(assets.storage.1.clone()),
                    ))
                    .id(),
            )
        }
        "defense" => {
            let idx = validate_level(level);
            let health = DEFENSE_HEALTH[idx];
            let damage = DEFENSE_DAMAGE[idx];
            let range = DEFENSE_RANGE[idx];
            let speed = DEFENSE_SPEED[idx];

            Some(
                commands
                    .spawn((
                        Defense::new(damage, range, speed),
                        Building::new_with_level(level, health),
                        GridPosition::new_full(
                            position.0,
                            position.1,
                            DEFENSE_SIZE.0,
                            DEFENSE_SIZE.1,
                        ),
                        Mesh2d(assets.defense.0.clone()),
                        MeshMaterial2d(assets.defense.1.clone()),
                    ))
                    .id(),
            )
        }
        "wall" => {
            let idx = validate_level(level);
            let health = WALL_HEALTH[idx];

            Some(
                commands
                    .spawn((
                        Wall::new(health),
                        Building::new_with_level(level, health),
                        GridPosition::new_full(position.0, position.1, WALL_SIZE.0, WALL_SIZE.1),
                        Mesh2d(assets.wall.0.clone()),
                        MeshMaterial2d(assets.wall.1.clone()),
                    ))
                    .id(),
            )
        }
        _ => None,
    }
}
