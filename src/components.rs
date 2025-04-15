use bevy::{prelude::*, utils::HashMap};

// Button marker component
#[derive(Component)]
pub enum MenuButton {
    Play,
    Editor,
    Quit,
}

// Map and grid components
#[derive(Component)]
pub struct TileMap {
    pub width: usize,               // 100 as you mentioned
    pub height: usize,              // 100 as you mentioned
    pub tiles: Vec<Option<Entity>>, // Stores entities placed on tiles
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        TileMap {
            width,
            height,
            tiles: vec![None; width * height],
        }
    }

    pub fn get_tile_idx(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get_entity_at(&self, x: usize, y: usize) -> Option<Entity> {
        self.get_tile_idx(x, y).and_then(|idx| self.tiles[idx])
    }

    pub fn place_entity(
        &mut self,
        x: usize,
        y: usize,
        entity: Entity,
        size: (usize, usize),
    ) -> bool {
        // Check if all required tiles are free
        for dy in 0..size.1 {
            for dx in 0..size.0 {
                if let Some(idx) = self.get_tile_idx(x + dx, y + dy) {
                    if self.tiles[idx].is_some() {
                        return false; // Tile already occupied
                    }
                } else {
                    return false; // Out of bounds
                }
            }
        }

        // Place entity on all required tiles
        for dy in 0..size.1 {
            for dx in 0..size.0 {
                if let Some(idx) = self.get_tile_idx(x + dx, y + dy) {
                    self.tiles[idx] = Some(entity);
                }
            }
        }
        true
    }
}

// Position component that tracks grid position
#[derive(Component, Debug, Clone)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

impl GridPosition {
    pub fn new_full(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> (Self, Transform, GridSize) {
        (
            Self { x, y },
            Transform::from_xyz(
                x as f32 + width as f32 / 2.0,
                y as f32 + height as f32 / 2.0,
                1.0,
            ),
            GridSize { width, height },
        )
    }
}

// For entities that occupy multiple tiles
#[derive(Component, Debug, Clone)]
pub struct GridSize {
    pub width: usize,
    pub height: usize,
}

// For mobile units
#[derive(Component, Debug, Clone)]
pub struct UnitPosition {
    pub x: f32,
    pub y: f32,
}

// Resource types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Gold,
    Elixir,
}

#[derive(Component, Debug, Clone)]
pub enum BuildingType {
    TownHall,
    ResourceCollector,
    Storage,
    Defense,
    Wall,
}

#[derive(Component, Debug, Clone)]
#[require(BuildingType(|| BuildingType::TownHall))]
pub struct TownHall;

#[derive(Component, Debug, Clone)]
#[require(BuildingType(|| BuildingType::ResourceCollector))]
pub struct ResourceCollector {
    pub resource_type: ResourceType,
    pub production_rate: f32,
}

impl ResourceCollector {
    pub fn new(resource_type: ResourceType, production_rate: f32) -> Self {
        ResourceCollector {
            resource_type,
            production_rate,
        }
    }
}

#[derive(Component, Debug, Clone)]
#[require(BuildingType(|| BuildingType::Storage))]
pub struct Storage {
    pub resource_type: ResourceType,
    pub capacity: u32,
}

impl Storage {
    pub fn new(resource_type: ResourceType, capacity: u32) -> Self {
        Storage {
            resource_type,
            capacity,
        }
    }
}

#[derive(Component, Debug, Clone)]
#[require(BuildingType(|| BuildingType::Defense))]
pub struct Defense {
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
}

impl Defense {
    pub fn new(attack_damage: f32, attack_range: f32, attack_speed: f32) -> Self {
        Defense {
            attack_damage,
            attack_range,
            attack_speed,
        }
    }
}

#[derive(Component, Debug, Clone)]
#[require(BuildingType(|| BuildingType::Wall))]
pub struct Wall {
    pub durability: f32,
}

impl Wall {
    pub fn new(durability: f32) -> Self {
        Wall { durability }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Building {
    pub level: u32,
    pub health: f32,
    pub max_health: f32,
}

impl Building {
    pub fn new(health: f32) -> Self {
        Building {
            level: 1,
            health,
            max_health: health,
        }
    }

    pub fn new_with_level(level: u32, health: f32) -> Self {
        Building {
            level,
            health,
            max_health: health,
        }
    }
}

// Unit components
#[derive(Component, Debug)]
pub struct Unit {
    pub health: f32,
    pub max_health: f32,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub movement_speed: f32,
    pub target_preference: TargetPreference,
}

#[derive(Debug, Clone)]
pub enum TargetPreference {
    AnyBuilding,
    DefenseFirst,
    ResourcesFirst,
}

// Global resource for player resources
#[derive(Resource, Debug, Default)]
pub struct PlayerResources {
    pub resources: HashMap<ResourceType, f64>,
}

#[derive(Resource)]
pub struct BuildingAssets {
    // TODO: HashMap<Building Type, (Handle mesh, Handle Colormaterial)>,
    pub town_hall: (Handle<Mesh>, Handle<ColorMaterial>),
    pub resource_collector: (Handle<Mesh>, Handle<ColorMaterial>),
    pub storage: (Handle<Mesh>, Handle<ColorMaterial>),
    pub defense: (Handle<Mesh>, Handle<ColorMaterial>),
    pub wall: (Handle<Mesh>, Handle<ColorMaterial>),
}

// Marker for selected entity
#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct GridVisual;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct GameCamera {
    pub speed: f32,
    pub is_dragging: bool,
    pub last_position: Option<Vec2>,
    pub zoom: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            speed: 20.0,
            is_dragging: false,
            last_position: None,
            zoom: 6.0,
            min_zoom: 4.0,
            max_zoom: 20.0,
            zoom_speed: 1.0,
        }
    }
}

#[derive(Component)]
pub struct GameHUD;

#[derive(Component)]
pub struct ResourceDisplay;

#[derive(Component)]
pub struct DebugOverlay;

#[derive(Resource, Default)]
pub struct DebugState {
    pub visible: bool,
}
