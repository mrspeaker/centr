use bevy::{
    prelude::*,
    color::palettes::basic::PURPLE,
    window::{CursorGrabMode, PrimaryWindow},
    input::mouse::MouseMotion,
};

use std::{fs::{self, File}, io, collections::HashMap, path::Path};
use serde::{Serialize, Deserialize};

use serde_json;

#[derive(Component, Serialize, Deserialize, Debug)]
struct Icon {
    name: String,
    x: f32,
    y: f32
}

#[derive(Debug, Event)]
struct SaveMeta;

fn main() {
    let person = Icon {
        name: String::from("My folder3"),
        x: 100.0,
        y: 100.0
    };
    let serialized = serde_json::to_string(&person).unwrap();
    let deserialized: Icon = serde_json::from_str(&serialized).unwrap();
    println!("{:?} {:?}", serialized, deserialized);

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, cursor_grab))
        .add_systems(Update, mouse_track)
        .add_observer(write_meta)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d);

    let font = TextFont {
        font_size: 10.0,
        ..default()
    };

    for ico in read_meta_n_files() {
        commands.spawn((
            Name::new("Folder"),
            Sprite::sized(Vec2::new(64., 64.)),
            Transform::from_xyz(ico.x, ico.y, 0.0),
            Visibility::default()
        )).with_children(|f| {
            /*f.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(PURPLE))),
                Transform::default()
                    .with_scale(Vec3::splat(64.))
            ));*/
            f.spawn((
                Text2d::new(ico.name.clone()),
                font.clone(),
                Transform::from_xyz(0.0, -40.0, 0.0),

            ));
        })
            .observe(recolor_on::<Pointer<Over>>(Srgba::hex("#990088").unwrap().into()))
            .observe(recolor_on::<Pointer<Out>>(Srgba::hex("#ff00ff").unwrap().into()));
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
            BackgroundColor(Srgba::hex("#ff00ff").unwrap().into()),
        ));
        parent.spawn((
            Node {
                width: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Srgba::hex("#000000").unwrap().into()),
        ));
    });

    commands.trigger(SaveMeta);

}


fn cursor_grab(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = windows.single_mut();
    let w = window.resolution.width();
    let h = window.resolution.height();
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    window.set_cursor_position(Some(Vec2::new(w/2., h/2.0)));
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

fn write_meta(
    _trigger: Trigger<SaveMeta>,
    icons: Query<&Icon>
) {
    let vec: Vec<&Icon> = icons.iter().collect();
    let serialized = serde_json::to_string(&vec).unwrap();
    match fs::write("./_meta.json", serialized) {
        Ok(_) => println!("yup"),
        Err(e) => println!("fail {}", e)
    }
}

fn read_meta_n_files() -> Vec<Icon> {
    let metadata_path = Path::new("./entries.json");
    let entries: Vec<Icon> = if metadata_path.exists() {
        let contents = fs::read_to_string(metadata_path).unwrap();
        serde_json::from_str(&contents).unwrap_or(Vec::new())
    } else {
        Vec::new()
    };

    let icon_map: HashMap<String, (f32, f32)> = entries.into_iter()
        .map(|icon| (icon.name, (icon.x, icon.y)))
        .collect();

    let mut icons: Vec<Icon> = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let entry_name = String::from(entry.file_name().to_str().unwrap());
        let ico = if icon_map.contains_key(&entry_name) {
            let pos = icon_map.get(&entry_name).unwrap();
            Icon { name: entry_name, x: pos.0, y: pos.1 }
        } else {
            Icon{ name: entry_name, x: 0.0, y: 0.0 }
        };
        icons.push(ico)
    };
    icons
}

// An observer listener that changes the target entity's color.
fn recolor_on<E: Clone + Reflect>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.entity()) else {
            return;
        };
        sprite.color = color;
    }
}
