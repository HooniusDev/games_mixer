use bevy::prelude::*;
use bevy::color::palettes::css::*;
use crate::my_app::AppState::Gameplay;
use crate::my_app::{Game};

pub fn plugin(app: &mut App) {

    app.add_systems(
        Update,
        check_collisions
            .run_if(in_state(Gameplay(Game::Flappy)))
            .run_if(is_alive),
    );
}

// Bird Collided Event
#[derive(Event)]
pub struct BirdCollidedEvent;

const PIPE_SIZE: Vec2 = Vec2::new(101., 603.0);

use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use crate::flappy::bird::is_alive;

fn check_collisions(
    mut commands: Commands,
    mut gizmos: Gizmos,
    bird_query: Single<&Transform, With<super::bird::Bird>>,
    pipe_query: Query<&Transform, With<super::pipes::Pipe>>,
) {
    let bird = *bird_query;
    for pipe_transform in pipe_query.iter() {
        gizmos.circle_2d(bird.translation.truncate() + Vec2::new(5., 3.), 32., RED);
        gizmos.rect_2d(pipe_transform.translation.truncate(), PIPE_SIZE, PINK );
        let collision = ball_collision(
            BoundingCircle::new(bird.translation.truncate() + Vec2::new(5., 3.), 32.),
            Aabb2d::new(
                pipe_transform.translation.truncate(),
                PIPE_SIZE / 2.0,
            )
        );
        if collision {
            //println!("Collision detected! {:?}", i);
            commands.trigger(BirdCollidedEvent)
        } else {
            //println!("No collision.");
        }
    }
}

fn ball_collision(bird: BoundingCircle, bounding_box: Aabb2d) -> bool {
    bird.intersects(&bounding_box)
}