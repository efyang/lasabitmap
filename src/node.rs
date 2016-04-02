use image::Rgb;
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

    pub fn make_pixel(&self) -> Rgb<u8> {
        if self.value {
            // wall
            Rgb([0, 0, 0])
        } else {
            if self.checked {
                Rgb([0, 255, 0])
            } else {
                Rgb([255, 255, 255])
            }
        }
    }
}
