mod quadtree;

use bevy::prelude::*;
use bevy::window::PresentMode;
use shape::Plane;
//wireframe debug
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_systems(Startup, setup)
        .add_plugins(WireframePlugin)
        .run();
}

//setup basic plane and camera
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //ground plane
    let plane = meshes.add(Mesh::from(Plane {
        size: 16.0,
        subdivisions: 0,
    }));
    let material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());
    commands.spawn(PbrBundle {
        mesh: plane,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    //camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(16.0, 16.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    //point light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..Default::default()
    });

    //quadtree
    let mut quadtree = quadtree::QuadTree::new(-16, -16, 32, None, 0);
    quadtree.subdivide();
    quadtree.get_children().unwrap()[0].subdivide();
    quadtree.get_children().unwrap()[0].get_children().unwrap()[1].subdivide();
    quadtree.get_children().unwrap()[0].get_children().unwrap()[1].get_children().unwrap()[2].subdivide();
    quadtree.get_children().unwrap()[0].get_children().unwrap()[1].get_children().unwrap()[2].get_children().unwrap()[3].subdivide();
    
    
    println!("{:#?}", quadtree);

    let height = 1.00001;
    let color = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::ORANGE,
        Color::PINK,
        Color::PURPLE,
        Color::TEAL,
    ];
    //add meshes for each quadrant
    for child in quadtree.get_all_children().unwrap() {
        if child.children.is_some() {
            println!("branch");
            continue;
        }

        println!("leaf");
        let plane = meshes.add(Mesh::from(Plane {
            size: child.dimension as f32,
            subdivisions: 0,
        }));
        let material = materials.add(color[child.depth % color.len()].into());
        commands.spawn((
            PbrBundle {
                mesh: plane,
                material,
                transform: Transform::from_xyz(
                    child.x as f32 + child.dimension as f32 / 2.0,
                    height,
                    child.y as f32 + child.dimension as f32 / 2.0,
                ),
                ..Default::default()
            },
            Wireframe,
        ));
    }
}

//recursively go through children again and again but not create any meshes
