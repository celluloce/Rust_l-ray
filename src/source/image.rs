pub struct Image<T> {
    pub data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    fn new<T>(fill: T, width: usize, height: usize) -> Image<T> {
        let v = Vec::with_capacity(height);
        for _ in 0..height {
            v.push(Vec::with_capacity(width));
        }
        Image {
            data: v,
            width: width,
            height: height,
        }
    }

    fn set<T>(&mut self, x: usize, y: usize, v: T) {
        let pub
    }
}
