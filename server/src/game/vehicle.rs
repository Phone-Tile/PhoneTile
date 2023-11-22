#[derive(Debug, Clone)]
pub struct Vehicle {
    pub speed: f64,
    pub curve_index: usize,
    pub t: f64,
    pub car_idx: usize,
    pub is_leaving: Option<super::bezier::Point>,
}

impl Vehicle {
    pub fn new(curve_index: usize, car_idx: usize) -> Vehicle {
        Vehicle {
            speed: 0.,
            curve_index,
            t: 0.,
            car_idx,
            is_leaving: None,
        }
    }
}
