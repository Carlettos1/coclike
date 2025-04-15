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

    let townhall_square = meshes.add(Rectangle::new(4.0, 4.0));
    let townhall_color = materials.add(Color::linear_rgb(0.0, 1.0, 0.0));
    let resource_collector_square = meshes.add(Rectangle::new(3.0, 3.0));
    let resource_collector_color = materials.add(Color::linear_rgb(1.0, 0.0, 1.0));
    let storage_square = meshes.add(Rectangle::new(4.0, 4.0));
    let storage_color = materials.add(Color::linear_rgb(1.0, 0.0, 1.0));
    let defense_square = meshes.add(Rectangle::new(3.0, 3.0));
    let defense_color = materials.add(Color::linear_rgb(1.0, 0.0, 0.0));
    let wall_square = meshes.add(Rectangle::new(1.0, 1.0));
    let wall_color = materials.add(Color::linear_rgb(0.6, 0.5, 0.4));

    let assets = BuildingAssets {
        town_hall: (townhall_square, townhall_color),
        resource_collector: (resource_collector_square, resource_collector_color),
        storage: (storage_square, storage_color),
        defense: (defense_square, defense_color),
        wall: (wall_square, wall_color),
    };

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
    // Add buildings to the map (TileMap)
    // Note: In a real implementation, you'd query for the map entity and get
    // the TileMap component to place these correctly
}

pub fn game_systems(mut set: ParamSet<(Query<()>, Res<PlayerResources>)>) {
    trace!("Executing game systems");
}

pub fn collect_resources(
    time: Res<Time>,
    mut resources: ResMut<PlayerResources>,
    query: Query<&ResourceCollector>,
) {
    trace!("Collecting resources {resources:?}");
    for resource_collector in query.iter() {
        let amount = (resource_collector.production_rate * time.delta_secs());
        *resources
            .resources
            .entry(resource_collector.resource_type.clone())
            .or_insert(0.0) += amount as f64;
    }
}

pub fn building_placement(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut map_query: Query<&mut TileMap>,
    mut resources: ResMut<PlayerResources>,
) {
    info!("TODO: building placement");
    // This would implement mouse-based building placement
    // 1. Convert mouse position to world coordinates
    // 2. Convert world coordinates to grid position
    // 3. Check resource requirements
    // 4. Place building if possible
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
            info!("Syncing {entity:?} building with map at {position:?} with size {size:?}");
            tile_map.place_entity(position.x, position.y, entity, (size.width, size.height));
        }
    }
}
