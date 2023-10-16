mod player;
mod quadtree;

use bevy::ecs::component::Component;
use bevy::{
    prelude::{
        default, shape, App, Assets, Camera3dBundle, Color, Commands, DirectionalLight,
        DirectionalLightBundle, Handle, ImagePlugin, Input, KeyCode, Mesh, PbrBundle, PluginGroup,
        Query, Res, ResMut, StandardMaterial, Startup, Transform, Update, Vec2, Vec3, With,
    },
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};
use player::Player;
use quadtree::QuadTree;
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
        .add_systems(
            Update,
            (
                simple_player_movement,
                update_quadtree_mesh,
                check_quadtree_data,
            ),
        )
        .add_plugins(WireframePlugin)
        .run();
}

//player movement
fn simple_player_movement(
    keyboard_input: Res<'_, Input<KeyCode>>,
    mut query: Query<(&mut player::Player, &mut Transform)>,
) {
    //move the player on the x and z axis
    for (mut player, mut transform) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec2::new(0., -1.);
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec2::new(1., 0.);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec2::new(0., 1.);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec2::new(-1., 0.);
        }

        player.move_player(direction);
        transform.translation.x = player.position.x;
        transform.translation.z = player.position.z;
    }
}

#[derive(Component)]
struct QuadTreeMeshes;

fn update_quadtree_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut qt_meshes: Query<(&mut Handle<Mesh>, &mut Handle<StandardMaterial>), With<QuadTreeMeshes>>,
    mut quadtree: ResMut<QuadTree>,
    mut player_query: Query<&mut Player>,
) {
    for mut player in player_query.iter_mut() {
        if !player.is_in_bounds() || player.get_bounds().is_none() {
            //clear the quadtree meshes and materials
            qt_meshes.iter_mut().for_each(|(mesh, material)| {
                meshes.remove(mesh.clone());
                materials.remove(material.clone());
            });
            quadtree.clear_children();

            //subdivide the quadtree to the player's current position
            quadtree.subdivide_until_depth([player.position.x, player.position.z], 5);

            //search for the quadtree node that contains the player
            for child in quadtree.get_all_children().unwrap().iter() {
                let mut max_depth = 0;
                if child.check_bounds([player.position.x, player.position.z]) {
                    child.get_depth();
                    max_depth = max_depth.max(child.get_depth());
                    if child.get_depth() == max_depth {
                        player.set_bounds(child);
                    }
                }
            }

            //spawn new meshes
            let mut qt_meshes = Vec::with_capacity(4);
            let mut qt_materials = Vec::with_capacity(4);
            let mut qt_transforms = Vec::with_capacity(4);
            // create meshes for each child node
            for (i, child) in quadtree.get_all_children().unwrap().iter().enumerate() {
                if child.children.is_some() {
                    continue;
                }

                qt_meshes.push(meshes.add(Mesh::from(Plane {
                    size: child.get_half_length(),
                    subdivisions: 0,
                })));
                qt_materials.push(materials.add(get_color(child.get_depth()).into()));

                qt_transforms.push(Transform::from_xyz(child.get_x(), 1.0, child.get_z()));

                spawn_qt_mesh(&mut commands, &qt_meshes, i, &qt_materials, &qt_transforms);
            }
        } else {
            //if the player is in bounds, do nothing
            return;
        }
    }
}

//press space to print the quadtree data
fn check_quadtree_data(
    keyboard_input: Res<'_, Input<KeyCode>>,
    quadtree: ResMut<QuadTree>,
    player_query: Query<&Player>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("{:#?}", quadtree);
        for player in player_query.iter() {
            println!("{:#?}", player);
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //spawn the quadtree
    let mut quadtree = quadtree::QuadTree::new([0., 0.], 16., 0);
    quadtree.subdivide();

    quadtree.get_children().unwrap()[0].subdivide();

    println!("{:#?}", quadtree);

    let mut qt_meshes = Vec::with_capacity(4);
    let mut qt_materials = Vec::with_capacity(4);
    let mut qt_transforms = Vec::with_capacity(4);

    //loop through every quadrant and spawn a plane mesh with a random color, same size as the quadrant
    for (i, child) in quadtree.get_all_children().unwrap().iter().enumerate() {
        qt_meshes.push(meshes.add(Mesh::from(Plane {
            size: child.get_half_length(),
            subdivisions: 0,
        })));
        qt_materials.push(materials.add(get_color(child.get_depth()).into()));

        qt_transforms.push(Transform::from_xyz(child.get_x(), 1.0, child.get_z()));

        spawn_qt_mesh(&mut commands, &qt_meshes, i, &qt_materials, &qt_transforms);
    }

    //spawn player
    //assign capsule mesh to player, similar to default player in unity
    let player_mesh = meshes.add(Mesh::from(shape::Capsule {
        radius: 0.5,
        depth: 1.,
        ..Default::default()
    }));
    commands
        .spawn((player::Player::new(Vec3::new(0., 0., 0.)),))
        .insert(PbrBundle {
            mesh: player_mesh,
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..Default::default()
        });

    //ground plane
    let plane = meshes.add(Mesh::from(Plane {
        size: 16.0,
        subdivisions: 1,
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
        transform: Transform::from_xyz(0.0, 16.0, -32.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 15000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(-10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(quadtree);
}

fn spawn_qt_mesh(
    commands: &mut Commands<'_, '_>,
    qt_meshes: &[Handle<Mesh>],
    i: usize,
    qt_materials: &[Handle<StandardMaterial>],
    qt_transforms: &[Transform],
) {
    commands.spawn((
        PbrBundle {
            mesh: qt_meshes.get(i).unwrap().clone(),
            material: qt_materials.get(i).unwrap().clone(),
            transform: *qt_transforms.get(i).unwrap(),
            ..Default::default()
        },
        Wireframe,
        QuadTreeMeshes,
    ));
}

fn get_color(depth: usize) -> Color {
    let color = [
        Color::AQUAMARINE,
        Color::BEIGE,
        Color::FUCHSIA,
        Color::TEAL,
        Color::NAVY,
    ];

    if depth > color.len() {
        return Color::WHITE;
    }

    color[depth % color.len()]
}
