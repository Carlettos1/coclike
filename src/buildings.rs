//! Here are the building constructors
use crate::prelude::*;
use bevy::prelude::*;

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
            GridPosition::new_full(x, y, TOWNHALL_SIZE.x as usize, TOWNHALL_SIZE.y as usize),
            assets.get(&BuildingType::TownHall).to_component(),
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
            GridPosition::new_full(x, y, COLLECTOR_SIZE.x as usize, COLLECTOR_SIZE.y as usize),
            assets
                .get(&BuildingType::Collector(ResourceType::Gold))
                .to_component(),
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
            GridPosition::new_full(x, y, COLLECTOR_SIZE.x as usize, COLLECTOR_SIZE.y as usize),
            assets
                .get(&BuildingType::Collector(ResourceType::Elixir))
                .to_component(),
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
            GridPosition::new_full(x, y, STORAGE_SIZE.x as usize, STORAGE_SIZE.y as usize),
            assets
                .get(&BuildingType::Storage(ResourceType::Gold))
                .to_component(),
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
            GridPosition::new_full(x, y, STORAGE_SIZE.x as usize, STORAGE_SIZE.y as usize),
            assets
                .get(&BuildingType::Storage(ResourceType::Elixir))
                .to_component(),
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
            GridPosition::new_full(x, y, DEFENSE_SIZE.x as usize, DEFENSE_SIZE.y as usize),
            assets.get(&BuildingType::Defense).to_component(),
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
            GridPosition::new_full(x, y, WALL_SIZE.x as usize, WALL_SIZE.y as usize),
            assets.get(&BuildingType::Wall).to_component(),
        ))
        .id()
}

// Function for custom positioning of any building
pub fn place_building(
    commands: &mut Commands,
    assets: &BuildingAssets,
    building_type: BuildingType,
    level: u32,
    x: usize,
    y: usize,
) -> Entity {
    match building_type {
        BuildingType::Defense => defense_tower(commands, assets, level, x, y),
        BuildingType::Collector(resource) => match resource {
            ResourceType::Gold => gold_collector(commands, assets, level, x, y),
            ResourceType::Elixir => elixir_collector(commands, assets, level, x, y),
        },
        BuildingType::Storage(resource) => match resource {
            ResourceType::Gold => gold_storage(commands, assets, level, x, y),
            ResourceType::Elixir => elixir_storage(commands, assets, level, x, y),
        },
        BuildingType::Wall => wall(commands, assets, level, x, y),
        BuildingType::TownHall => town_hall(commands, assets, level, x, y),
    }
}

pub fn to_size(building_type: BuildingType) -> (usize, usize) {
    match building_type {
        BuildingType::Defense => (DEFENSE_SIZE.x as usize, DEFENSE_SIZE.y as usize),
        BuildingType::Collector(_) => (COLLECTOR_SIZE.x as usize, COLLECTOR_SIZE.y as usize),
        BuildingType::Storage(_) => (STORAGE_SIZE.x as usize, STORAGE_SIZE.y as usize),
        BuildingType::Wall => (WALL_SIZE.x as usize, WALL_SIZE.y as usize),
        BuildingType::TownHall => (TOWNHALL_SIZE.x as usize, TOWNHALL_SIZE.y as usize),
    }
}
