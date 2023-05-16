use bevy::{prelude::{*}, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.85, 1.00)))
        .init_resource::<ShelfStructure>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_startup_system(setup_shelves)
        .add_startup_system(setup_player)
        .add_system(player_movement)
        .run();
}
#[derive(Resource)]
struct ShelfStructure {
    level_count: u32,
    width: u32, 
    shelves: Vec<ShelfPosition>
}

#[derive(Clone, Copy, Debug)]
struct ShelfPosition {
    level: u32,
    start: i32,
    end: i32,
}

impl Default for ShelfStructure {
    fn default() -> ShelfStructure {
        ShelfStructure { 
            level_count: 6,
            width: 32, 
            shelves: vec![
                ShelfPosition {
                    level: 0,
                    start: -8,
                    end: 8,
                },
                ShelfPosition {
                    level: 1,
                    start: -12,
                    end: 1,
                },
                ShelfPosition {
                    level: 1,
                    start: 3,
                    end: 7,
                },
                ShelfPosition {
                    level: 2,
                    start: -3,
                    end: 9,
                },
                ShelfPosition {
                    level: 3,
                    start: -7,
                    end: 3,
                },
                ShelfPosition {
                    level: 3,
                    start: 5,
                    end: 9,
                },
                ShelfPosition {
                    level: 4,
                    start: 1,
                    end: 4,
                },
                ShelfPosition {
                    level: 5,
                    start: -7,
                    end: -3,
                },
                ShelfPosition {
                    level: 5,
                    start: -1,
                    end: 2,
                },
                ShelfPosition {
                    level: 5,
                    start: 5,
                    end: 8,
                },
            ],
        }
    }
}

#[derive(Component)]
struct Shelf{
    
}

#[derive(Component)]
struct Player {
    shelf: ShelfPosition,
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    shelf_structure: Res<ShelfStructure>
) {
    let window = window_query.get_single().unwrap();
    let smallest_width = window.width() / 32.0;
    let smallest_height = window.height() / (shelf_structure.level_count + 1) as f32;

    let initial_shelf = shelf_structure.shelves.get(0).unwrap();

    commands.spawn((
        Player{ 
            shelf: initial_shelf.clone(),
        },
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2 { x: smallest_width, y: smallest_width },
                origin: shapes::RectangleOrigin::CustomCenter(Vec2 { 
                    x: smallest_width * ((initial_shelf.start + initial_shelf.end) / 2) as f32, 
                    y: smallest_width * 1.5 + smallest_height * initial_shelf.level as f32 - window.height() / 2.0 
                }),
            }),
            ..default()
        },
        Fill::color(Color::ALICE_BLUE),
        Stroke::new(Color::ANTIQUE_WHITE, 2.0),
    ));
}

fn setup_shelves(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    shelf_structure: Res<ShelfStructure>,
) {
    let window = window_query.get_single().unwrap();

    let smallest_width = window.width() / shelf_structure.width as f32;
    let smallest_height = window.height() / (shelf_structure.level_count + 1) as f32;

    for shelf in shelf_structure.shelves.iter() {
        commands.spawn((
            Shelf{},
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RoundedPolygon {
                    points: vec![
                        Vec2{
                            x: shelf.start as f32 * smallest_width,
                            y: smallest_height * shelf.level as f32 - window.height() / 2.0,
                        },
                        Vec2{
                            x: shelf.start as f32 * smallest_width,
                            y: smallest_height * shelf.level as f32 + smallest_width - window.height() / 2.0,
                        },
                        Vec2{
                            x: shelf.end as f32 * smallest_width,
                            y: smallest_height * shelf.level as f32 + smallest_width - window.height() / 2.0,
                        },
                        Vec2{
                            x: shelf.end as f32 * smallest_width,
                            y: smallest_height * shelf.level as f32 - window.height() / 2.0,
                        },
                    ],
                    radius: 10.0,
                    closed: true,
                }),
                ..default()
            },
            Fill::color(Color::MIDNIGHT_BLUE),
            Stroke::new(Color::ANTIQUE_WHITE, 2.0),
        ));
    }
}

fn player_movement(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(Entity, &Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    shelf_structure: Res<ShelfStructure>,
) {
    let window = window_query.get_single().unwrap();

    let smallest_width = window.width() / shelf_structure.width as f32;
    let smallest_height = window.height() / (shelf_structure.level_count + 1) as f32;

    if let Ok((entity, player)) = player_query.get_single_mut() {
        let new_shelf_optional: Option<ShelfPosition> = 
            if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                let shelves_in_the_same_level = shelf_structure.shelves.iter().filter(|shelf| shelf.level == player.shelf.level);
                let mut candidate: Option<ShelfPosition> = Option::None; 
                for shelf in shelves_in_the_same_level {
                    if shelf.end < player.shelf.start {
                        if candidate.is_none() || candidate.unwrap().end < shelf.end  {
                            candidate = Option::from(shelf.clone());
                        }
                    }
                }
                candidate
            } else if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                let shelves_in_the_same_level = shelf_structure.shelves.iter().filter(|shelf| shelf.level == player.shelf.level);
                let mut candidate: Option<ShelfPosition> = Option::None; 
                println!("player.shelf: {:?}", player.shelf);
                for shelf in shelves_in_the_same_level {
                    println!("{:?}", shelf);
                    if player.shelf.end < shelf.start {
                        println!("player.shelf: {:?}", player.shelf);
                        println!("shelf: {:?}", shelf);
                        if candidate.is_none() || shelf.end < candidate.unwrap().end  {
                            println!("candidate: {:?}", candidate);
                            candidate = Option::from(shelf.clone());
                        }
                    }
                }
                candidate
            } else if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                if player.shelf.level == shelf_structure.level_count - 1 {
                    None
                } else {
                    let mut candidate: Option<ShelfPosition> = Option::None; 
                    let current_center = player.shelf.start + player.shelf.end;
                    shelf_structure.shelves.iter().filter(|shelf| shelf.level == player.shelf.level + 1).for_each(|shelf| {
                        if candidate.is_none() 
                            || (shelf.start + shelf.end - current_center).abs() < (candidate.unwrap().start + candidate.unwrap().end - current_center).abs()  {
                            candidate = Option::from(shelf.clone());
                        }
                    });
                    candidate
                }
            } else if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                if player.shelf.level == 0 {
                    None
                } else {
                    let mut candidate: Option<ShelfPosition> = Option::None; 
                    let current_center = player.shelf.start + player.shelf.end;
                    shelf_structure.shelves.iter().filter(|shelf| shelf.level == player.shelf.level - 1).for_each(|shelf| {
                        if candidate.is_none() 
                            || (shelf.start + shelf.end - current_center).abs() < (candidate.unwrap().start + candidate.unwrap().end - current_center).abs()  {
                            candidate = Option::from(shelf.clone());
                        }
                    });
                    candidate
                }
            } else {
                Option::None
            };
        
        if new_shelf_optional.is_some() {
            let new_shelf = new_shelf_optional.unwrap();
            print!("new_shelf: {:?}", new_shelf);
            commands.entity(entity).despawn();
            commands.spawn((
                Player{ 
                    shelf: new_shelf,
                },
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: Vec2 { x: smallest_width, y: smallest_width },
                        origin: shapes::RectangleOrigin::CustomCenter(Vec2 { 
                            x: smallest_width * ((new_shelf.start + new_shelf.end) / 2) as f32, 
                            y: smallest_width * 1.5 + smallest_height * new_shelf.level as f32 - window.height() / 2.0 
                        }),
                    }),
                    ..default()
                },
                Fill::color(Color::ALICE_BLUE),
                Stroke::new(Color::ANTIQUE_WHITE, 2.0),
            ));
        }
        
    }

    

}