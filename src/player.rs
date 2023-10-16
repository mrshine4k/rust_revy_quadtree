use bevy::prelude::{Component, Vec2, Vec3};

use crate::quadtree::QuadTree;

#[derive(Debug, Clone, Component)]
pub struct Player {
    pub position: Vec3,
    pub current_bounds: Option<QuadTree>,
}

// Player movement speed
pub const PLAYER_SPEED: f32 = 0.025;

impl Player {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            current_bounds: None,
        }
    }

    pub fn move_player(&mut self, direction: Vec2) {
        self.position += Vec3::new(direction.x, 0., direction.y * -1.) * PLAYER_SPEED;
    }

    pub fn reset_pos(&mut self) {
        self.position = Vec3::new(0., 0., 0.);
        self.current_bounds = None;
    }

    pub fn set_bounds(&mut self, bounds: &QuadTree) {
        self.current_bounds = Some(bounds.clone());
    }

    pub fn get_bounds(&self) -> Option<QuadTree> {
        self.current_bounds.clone()
    }

    pub fn is_in_bounds(&self) -> bool {
        //check if the current position is in the bounds of the current quadtree
        if let Some(bound) = &self.current_bounds {
            return bound.check_bounds([self.position.x, self.position.z]);
        }
        false
    }
}
