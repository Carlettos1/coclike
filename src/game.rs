use crate::prelude::*;
use bevy::{prelude::*, utils::HashMap};

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let map = TileMap::new(100, 100);
    commands.spawn(map);

    let mut resources = HashMap::new();
    resources.insert(ResourceType::Gold, 1000);
    resources.insert(ResourceType::Elixir, 1000);

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

    spawn_initial_buildings(&mut commands, &assets);
    commands.insert_resource(assets);
}

fn spawn_initial_buildings(commands: &mut Commands, assets: &BuildingAssets) {
    let town_hall = commands
        .spawn((
            Building::new(1000.0),
            BuildingType::TownHall(TownHall),
            GridPosition { x: 45, y: 45 },
            GridSize {
                width: 4,
                height: 4,
            },
            Mesh2d(assets.town_hall.0.clone()),
            MeshMaterial2d(assets.town_hall.1.clone()),
        ))
        .id();

    let gold_mine = commands
        .spawn((
            Building::new(500.0),
            BuildingType::ResourceCollector(ResourceCollector::new(ResourceType::Gold, 10.0)),
            GridPosition { x: 40, y: 40 },
            GridSize {
                width: 3,
                height: 3,
            },
            Mesh2d(assets.resource_collector.0.clone()),
            MeshMaterial2d(assets.resource_collector.1.clone()),
        ))
        .id();

    // Add buildings to the map
    // Note: In a real implementation, you'd query for the map entity and get
    // the TileMap component to place these correctly
}

pub fn game_systems(mut set: ParamSet<(Query<()>, Res<PlayerResources>)>) {}

pub fn collect_resources(
    time: Res<Time>,
    mut resources: ResMut<PlayerResources>,
    query: Query<&BuildingType>,
) {
    for building_type in query.iter() {
        if let BuildingType::ResourceCollector(ResourceCollector {
            resource_type,
            production_rate,
        }) = building_type
        {
            let amount = (production_rate * time.delta_secs()) as u32;
            *resources
                .resources
                .entry(resource_type.clone())
                .or_insert(0) += amount;
        }
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
    // This would implement mouse-based building placement
    // 1. Convert mouse position to world coordinates
    // 2. Convert world coordinates to grid position
    // 3. Check resource requirements
    // 4. Place building if possible
}
