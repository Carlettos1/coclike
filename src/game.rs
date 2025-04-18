use crate::prelude::*;
use bevy::{prelude::*, utils::HashMap};

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Setup game core");
    let map = TileMap::new(100, 100);
    commands.spawn(map);

    let mut resources = HashMap::new();
    resources.insert(ResourceType::Gold, 1000.0);
    resources.insert(ResourceType::Elixir, 1000.0);

    commands.insert_resource(PlayerResources { resources });

    let townhall_square = meshes.add(Rectangle::new(TOWNHALL_SIZE.x, TOWNHALL_SIZE.y));
    let townhall_color = materials.add(TOWNHALL_COLOR);
    let resource_collector_square = meshes.add(Rectangle::new(COLLECTOR_SIZE.x, COLLECTOR_SIZE.y));
    let elixir_color = materials.add(ELIXIR_COLOR);
    let gold_color = materials.add(GOLD_COLOR);
    let storage_square = meshes.add(Rectangle::new(STORAGE_SIZE.x, STORAGE_SIZE.y));
    let defense_square = meshes.add(Rectangle::new(DEFENSE_SIZE.x, DEFENSE_SIZE.y));
    let defense_color = materials.add(DEFENSE_COLOR);
    let wall_square = meshes.add(Rectangle::new(WALL_SIZE.x, WALL_SIZE.y));
    let wall_color = materials.add(WALL_COLOR);
    let default_color = materials.add(WHITE);

    let townhall = Handles::new(&townhall_square, &townhall_color);
    let elixir_collector = Handles::new(&resource_collector_square, &elixir_color);
    let gold_collector = Handles::new(&resource_collector_square, &gold_color);
    let elixir_storage = Handles::new(&storage_square, &elixir_color);
    let gold_storage = Handles::new(&storage_square, &gold_color);
    let defense = Handles::new(&defense_square, &defense_color);
    let wall = Handles::new(&wall_square, &wall_color);

    let mut assets = BuildingAssets::new(Handles::new(&wall_square, &default_color));
    assets.insert(BuildingType::TownHall, townhall);
    assets.insert(
        BuildingType::Collector(ResourceType::Elixir),
        elixir_collector,
    );
    assets.insert(BuildingType::Collector(ResourceType::Gold), gold_collector);
    assets.insert(BuildingType::Storage(ResourceType::Elixir), elixir_storage);
    assets.insert(BuildingType::Storage(ResourceType::Gold), gold_storage);
    assets.insert(BuildingType::Defense, defense);
    assets.insert(BuildingType::Wall, wall);

    commands.insert_resource(assets);
}

pub fn spawn_initial_buildings(mut commands: Commands, assets: Res<BuildingAssets>) {
    info!("Spawning buildings");

    town_hall(&mut commands, &assets, 1, 45, 45);
    gold_collector(&mut commands, &assets, 1, 40, 40);
    elixir_collector(&mut commands, &assets, 1, 50, 40);
    gold_storage(&mut commands, &assets, 1, 40, 50);
    elixir_storage(&mut commands, &assets, 1, 50, 50);
    defense_tower(&mut commands, &assets, 1, 45, 55);

    // Spawn some walls for perimeter protection
    for i in 0..5 {
        wall(&mut commands, &assets, 1, 44 + i, 60); // North wall
        wall(&mut commands, &assets, 1, 44 + i, 40); // South wall
        wall(&mut commands, &assets, 1, 40, 44 + i); // West wall
        wall(&mut commands, &assets, 1, 60, 44 + i); // East wall
    }
}

pub fn collect_resources(
    time: Res<Time>,
    mut resources: ResMut<PlayerResources>,
    query: Query<&ResourceCollector>,
) {
    trace!("Collecting resources {resources:?}");
    for resource_collector in query.iter() {
        let amount = resource_collector.production_rate * time.delta_secs();
        *resources
            .resources
            .entry(resource_collector.resource_type)
            .or_insert(0.0) += amount as f64;
    }
}

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid_size = 100.0;
    let line_width = 0.1;
    info!("Setup map grid with len: {grid_size}, wid: {line_width}");
    let grid_line_material = materials.add(Color::srgba(0.5, 0.5, 0.5, 0.3));

    // Create horizontal and vertical lines
    for i in 0..=100 {
        // Horizontal lines
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(grid_size, line_width)))),
            MeshMaterial2d(grid_line_material.clone()),
            Transform::from_xyz(50.0, i as f32, 0.0),
            GridVisual,
        ));

        // Vertical lines
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(line_width, grid_size)))),
            MeshMaterial2d(grid_line_material.clone()),
            Transform::from_xyz(i as f32, 50.0, 0.0),
            GridVisual,
        ));
    }
}

pub fn synchronize_buildings_with_map(
    mut map_query: Query<&mut TileMap>,
    building_query: Query<(Entity, &GridPosition, &GridSize), Added<Building>>,
) {
    if let Ok(mut tile_map) = map_query.get_single_mut() {
        for (entity, position, size) in building_query.iter() {
            debug!("Syncing {entity:?} building with map at {position:?} with size {size:?}");
            tile_map.place(position.x, position.y, entity, (size.width, size.height));
        }
    }
}
