//FIRST REWRITE

#![warn(unreachable_code)]
use bevy::prelude::{Component, Resource};

///# Quadtree
/// A node of the quadtree, containing the dimension, x and y position, the corner it is in, the depth of the node and the children,
///which is an array of 4 quadtree nodes, each can be subdivided into 4 more nodes and so on
///
///The children are stored in a Box<> to prevent recursive type, hence all the as_() calls,
///same goes for Option<> with unwrap() and ? calls
#[derive(Debug, Clone, Component, Resource)]
pub struct QuadTree {
    //[x , z]
    position: [f32; 2],
    depth: usize,
    half_length: f32,
    pub children: Option<Box<[QuadTree; 4]>>,
}

const DEPTH_MAX: usize = 5;

impl QuadTree {
    pub fn new(position: [f32; 2], half_length: f32, depth: usize) -> Self {
        Self {
            position,
            half_length,
            depth,
            children: None,
        }
    }

    /// Returns the position of the quadtree node
    pub fn get_position(&self) -> [f32; 2] {
        self.position
    }

    /// Subdivides the quadtree into 4 children, clockwise from top right i.e.
    ///
    /// Will not subdivide if children already exist, as the child arr already populated
    ///
    /// [[0]] = top right, [[1]] = bottom right, [[2]] = bottom left, [[3]] = top left
    pub fn subdivide(&mut self) {
        let length = self.half_length;
        let half_length = self.half_length / 2.;
        let depth = self.depth + 1;
        let children = [
            QuadTree::new(
                // Top right
                [
                    self.position[0] + half_length / 2.,
                    self.position[1] + half_length / 2.,
                ],
                half_length,
                depth,
            ),
            QuadTree::new(
                // Bottom right
                [
                    self.position[0] + half_length / 2.,
                    self.position[1] - half_length / 2.,
                ],
                half_length,
                depth,
            ),
            QuadTree::new(
                // Bottom left
                [
                    self.position[0] - half_length / 2.,
                    self.position[1] - half_length / 2.,
                ],
                half_length,
                depth,
            ),
            QuadTree::new(
                // Top left
                [
                    self.position[0] - half_length / 2.,
                    self.position[1] + half_length / 2.,
                ],
                half_length,
                depth,
            ),
        ];
        self.children = Some(Box::new(children));
    }

    pub fn offset(&mut self, offset: [f32; 2]) {
        if self.get_children().is_some() {
            self.children
                .as_mut()
                .unwrap()
                .iter_mut()
                .for_each(|child| {
                    child.offset(offset);
                });
        }
        self.position[0] += offset[0];
        self.position[1] += offset[1];
    }

    /// Returns a mutable reference of the children inside the quadtree
    pub fn get_children(&mut self) -> Option<&mut [QuadTree; 4]> {
        self.children.as_ref()?;

        Some(self.children.as_mut().unwrap())
    }

    /// Returns the reference of all children inside the quadtree onto a vector
    pub fn get_all_children(&self) -> Option<Vec<&QuadTree>> {
        self.children.as_ref()?;
        let mut children = Vec::with_capacity(4);

        for child in self.children.as_ref().unwrap().iter() {
            if child.children.is_some() {
                children.append(&mut child.get_all_children().unwrap());
            } else {
                children.push(child);
            }
        }

        Some(children)
    }

    pub fn subdivide_until_depth(&mut self, position: [f32; 2], depth: usize) -> Option<QuadTree> {
        if self.depth == depth {
            return Some(self.clone());
        }

        self.subdivide();

        if self.get_children().is_some() {
            self.children
                .as_mut()
                .unwrap()
                .iter_mut()
                .for_each(|child| {
                    if child.check_bounds(position) {
                        child.subdivide_until_depth(position, depth);
                    }
                });
        }
        None
    }

    /// Checks if the given point is inside the quadtree
    // -----------
    // |    |    | o
    // |____|____|
    // |    |o   |
    // |    |    |
    // -----------
    /// 
    pub fn check_bounds(&self, point: [f32; 2]) -> bool {
        if point[0] < self.position[0] - self.half_length
            || point[0] > self.position[0] + self.half_length
            || point[1] < self.position[1] - self.half_length
            || point[1] > self.position[1] + self.half_length
        {
            return false;
        }
        true
    }

    pub fn get_children_count(&self) -> usize {
        if self.children.is_none() {
            return 0;
        }

        let mut count = 0;
        for child in self.children.as_ref().unwrap().iter() {
            if child.children.is_some() {
                count += child.get_children_count();
            } else {
                count += 1;
            }
        }

        count
    }

    pub fn clear_children(&mut self) {
        self.children = None;
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn get_half_length(&self) -> f32 {
        self.half_length
    }

    pub fn get_x(&self) -> f32 {
        self.position[0]
    }

    pub fn get_z(&self) -> f32 {
        self.position[1]
    }
}
