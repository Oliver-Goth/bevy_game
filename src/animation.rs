fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer)>,
) {
    for (mut sprite, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = (sprite.index + 1) % 9;
        }
    }
}

#[derive(Component)]
struct AnimationTimer(Timer);
