#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuadTree {
    pub dimension: usize,
    pub x: i32,
    pub y: i32,
    pub corner: Option<CornerPosition>,
    pub depth: usize,
    pub children: Option<Box<[QuadTree; 4]>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CornerPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl QuadTree {
    pub fn new(
        x: i32,
        y: i32,
        dimension: usize,
        corner: Option<CornerPosition>,
        depth: usize,
    ) -> Self {
        Self {
            dimension,
            x,
            y,
            children: None,
            corner,
            depth,
        }
    }

    pub fn subdivide(&mut self) {
        if self.dimension <= 2 {
            return;
        }

        let dimension = self.dimension / 2;
        let child_depth = self.depth + 1;
        let children = [
            QuadTree::new(
                self.x,
                self.y,
                dimension,
                Some(CornerPosition::TopLeft),
                child_depth,
            ),
            QuadTree::new(
                self.x + dimension as i32,
                self.y,
                dimension,
                Some(CornerPosition::TopRight),
                child_depth,
            ),
            QuadTree::new(
                self.x,
                self.y + dimension as i32,
                dimension,
                Some(CornerPosition::BottomLeft),
                child_depth,
            ),
            QuadTree::new(
                self.x + dimension as i32,
                self.y + dimension as i32,
                dimension,
                Some(CornerPosition::BottomRight),
                child_depth,
            ),
        ];
        self.children = Some(Box::new(children));
    }

    pub fn get_children(&mut self) -> Option<&mut [QuadTree; 4]> {
        self.children.as_ref()?;

        Some(self.children.as_mut().unwrap())
    }

    pub fn get_all_children(&self) -> Option<Vec<&QuadTree>> {
        let mut children = Vec::new();
        self.children.as_ref()?;

        for child in self.children.as_ref().unwrap().iter() {
            if child.children.is_some() {
                children.append(&mut child.get_all_children().unwrap());
            } else {
                children.push(child);
            }
        }

        Some(children)
    }

    pub fn get_size(&self) -> usize {
        let mut size = 0;
        if self.children.is_none() {
            return 1;
        }

        for child in self.children.as_ref().unwrap().iter() {
            size += child.get_size();
        }

        size
    }
}
