use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player::{CharacterAnimation, AnimationTimer, Direction, Player};

use crate::dialogue::logic::DialogueState;
use crate::dialogue::ui::{spawn_dialogue_box, DialogueBox};

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct Waypoints {
    pub points: Vec<Vec3>,
    pub current: usize,
}

#[derive(Component)]
pub struct NpcInteraction;

#[derive(Component)]
pub struct NpcState {
    pub stopped: bool,
}

pub fn spawn_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("Sprites/Character/character2.png");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 3, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture: texture_handle,
            atlas: TextureAtlas { layout: layout_handle, index: 1 },
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 1.0),
                ..default()
            },
            ..default()
        },
        Npc,
        NpcInteraction,
        CharacterAnimation {
            direction: Direction::Down,
            moving: false,
            frame: 0,
        },
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        Waypoints {
            points: vec![
                Vec3::new(100.0, 100.0, 1.0),
                Vec3::new(150.0, 100.0, 1.0),
                Vec3::new(150.0, 150.0, 1.0),
                Vec3::new(100.0, 150.0, 1.0),
            ],
            current: 0,
        },
        NpcState { stopped: false },

        RigidBody::KinematicPositionBased,
        Collider::cuboid(8.0, 8.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
    ));
}

pub fn npc_patrol(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Waypoints,
        &mut CharacterAnimation,
        &NpcState,
        &mut KinematicCharacterController,
    ), With<Npc>>,
    controller_output: Query<&KinematicCharacterControllerOutput>,
    transforms: Query<&Transform>,
) {
    for (entity, mut path, mut anim, state, mut controller) in query.iter_mut() {
        if state.stopped {
            anim.moving = false;
            controller.translation = Some(Vec2::ZERO);
            continue;
        }

        let transform = transforms.get(entity).unwrap();
        let target = path.points[path.current];
        let direction = (target - transform.translation).truncate();

        if direction.length() < 1.0 {
            path.current = (path.current + 1) % path.points.len();
            anim.moving = false;
            controller.translation = Some(Vec2::ZERO);
        } else {
            let step = direction.normalize() * 50.0 * time.delta_seconds();
            controller.translation = Some(step);

            if let Ok(output) = controller_output.get(entity) {
                if output.effective_translation.length_squared() < 0.01 {
                    anim.moving = false;
                } else {
                    anim.moving = true;

                    anim.direction = if direction.x.abs() > direction.y.abs() {
                        if direction.x > 0.0 { Direction::Right } else { Direction::Left }
                    } else {
                        if direction.y > 0.0 { Direction::Up } else { Direction::Down }
                    };
                }
            }
        }
    }
}

pub fn npc_interact(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut npc_query: Query<(&Transform, &mut CharacterAnimation, &mut NpcState), With<Npc>>,

    mut dialogue: ResMut<DialogueState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialogue_box_query: Query<Entity, With<DialogueBox>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };

    for (npc_transform, mut anim, mut state) in npc_query.iter_mut() {
        let distance = player_transform.translation.distance(npc_transform.translation);

        if distance > 48.0 {
            state.stopped = false;
            continue;
        }

        if keyboard_input.just_pressed(KeyCode::KeyE) && distance < 24.0 {
            // Don't trigger again if dialogue is already active
            if dialogue.active {
                return;
            }

            let diff = (npc_transform.translation - player_transform.translation).truncate();
            anim.direction = if diff.x.abs() > diff.y.abs() {
                if diff.x > 0.0 { Direction::Left } else { Direction::Right }
            } else {
                if diff.y > 0.0 { Direction::Down } else { Direction::Up }
            };

            anim.moving = false;
            state.stopped = true;

            //println!("NPC: Hello there! How's your day going?");
            //dialogue.set_line("NPC: Hello there! How's your day going?");

            // Load new dialogue
            dialogue.load_file("dialogue/npcs/nora.ron");

            // Only spawn a dialogue box if one isn't already on screen
            if dialogue_box_query.get_single().is_err() {
                spawn_dialogue_box(&mut commands, &asset_server);
            }
        }
    }
}

