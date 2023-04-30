pub struct Block {
    width: u32,
    length: u32,
    height: u32,
}

impl Block {
    pub fn new(dimensions: &[u32; 3]) -> Self {
        Block {
            width: dimensions[0],
            length: dimensions[1],
            height: dimensions[2],
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_volume(&self) -> u32 {
        self.width * self.length * self.height
    }

    pub fn get_surface_area(&self) -> u32 {
        2 * (self.width * self.length + self.length * self.height + self.height * self.width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(block.width, 2);
        assert_eq!(block.length, 4);
        assert_eq!(block.height, 6);
    }

    #[test]
    fn test_get_width() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(block.get_width(), 2);
    }

    #[test]
    fn test_get_length() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(block.get_length(), 4);
    }

    #[test]
    fn test_get_height() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(block.get_height(), 6);
    }

    #[test]
    fn test_get_volume() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(block.get_volume(), 48);
    }

    #[test]
    fn test_get_surface_area() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(block.get_surface_area(), 88);
    }
}
