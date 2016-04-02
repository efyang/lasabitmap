use bmp::Pixel;
#[derive(Clone)]
pub struct Node {
    pub value: bool,
    pub checked: bool,
}

impl Node {
    pub fn new(value: bool) -> Node {
        Node {
            value: value,
            checked: false,
        }
    }

    pub fn make_pixel(&self) -> Pixel {
        if self.value {
            // wall
            px!(0, 0, 0)
        } else {
            if self.checked {
                px!(0, 255, 0)
            } else {
                px!(255, 255, 255)
            }
        }
    }
}
