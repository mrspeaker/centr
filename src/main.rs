use bevy::prelude::*;
use bevy::color::palettes::basic::PURPLE;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::input::mouse::MouseMotion;

use std::fs;
use std::io;

fn read_dir(path: &str) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(path)? {
        entries.push(entry?);
    }
    Ok(entries)
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, cursor_grab))
        .add_systems(Update, mouse_track)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let Ok(paths) = read_dir("./") else { return };

    commands.spawn(Camera2d);

    let font = TextFont {
        font_size: 10.0,
        ..default()
    };

    let mut i = 0.0;
    for p in paths {
        let n = p.path().to_str().unwrap_or("").to_string();
        commands.spawn((
            Name::new("Folder"),
            Transform::from_xyz(i * 72.0, 0.0, 0.0)
        )).with_children(|f| {
            f.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(PURPLE))),
                Transform::default()
                    .with_scale(Vec3::splat(64.))
            ));
            f.spawn((
                Text2d::new(n.clone()),
                font.clone()
            ));
        });
        i += 1.0;
    }

    commands.spawn((
        Name::new("Crosshair"),
        Node {
            width: Val::Px(4.0),
            height: Val::Px(4.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            margin: UiRect {
                left: Val::Px(-5.0), // Offset to center
                top: Val::Px(-5.0),
                ..default()
            },
            ..default()
        })).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Srgba::hex("#ff0000").unwrap().into()),
        ));
        parent.spawn((
            Node {
                width: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Srgba::hex("#000000").unwrap().into()),
        ));
    });

}


fn cursor_grab(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = windows.single_mut();
    //window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
}

fn cursor_ungrab(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = windows.single_mut();
    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;
}

fn mouse_track(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>
) {
    let Ok(mut camera) = camera.get_single_mut() else { return; };

    let dt = time.delta_secs();
    let x_speed = 10.0;
    let y_speed = x_speed * 2.0;

    for event in mouse_motion_events.read() {
        let offset = event.delta;
        camera.translation.x += offset.x * x_speed * dt;
        camera.translation.y -= offset.y * y_speed * dt;
    }
}
