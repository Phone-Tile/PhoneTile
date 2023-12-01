use super::Vector2;
use std::time;

pub const POWERUP_COUNT: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    SpeedUp,
    SpeedDown,
    SizeUp,
    SizeDown,
    FiringRateUp,
    FiringRateDown,
    Unknown,
}

impl From<Type> for usize {
    fn from(value: Type) -> Self {
        match value {
            Type::SpeedUp => 1,
            Type::SpeedDown => 2,
            Type::SizeUp => 3,
            Type::SizeDown => 4,
            Type::FiringRateUp => 5,
            Type::FiringRateDown => 6,
            Type::Unknown => 0xffff,
        }
    }
}

impl From<usize> for Type {
    fn from(value: usize) -> Self {
        match value {
            1 => Type::SpeedUp,
            2 => Type::SpeedDown,
            3 => Type::SizeUp,
            4 => Type::SizeDown,
            5 => Type::FiringRateUp,
            6 => Type::FiringRateDown,
            _ => Type::Unknown,
        }
    }
}

impl Type {
    pub fn get_modifier(&self) -> f32 {
        match self {
            Type::SpeedUp => 3.,
            Type::SpeedDown => -3.,
            Type::SizeUp => 30.,
            Type::SizeDown => -30.,
            Type::FiringRateUp => -5.,
            Type::FiringRateDown => 5.,
            Type::Unknown => 0.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PowerUp {
    t: Type,
    is_activated: bool,
    position: Vector2,
    timer: time::Instant,
    time_to_live: u64,
    modifier: f32,
}

const TIME_TO_LIVE: u64 = 10;

impl PowerUp {
    pub fn new(t: Type, position: Vector2) -> Self {
        PowerUp {
            t,
            is_activated: false,
            position,
            timer: time::Instant::now(),
            time_to_live: TIME_TO_LIVE,
            modifier: t.get_modifier(),
        }
    }

    pub fn pos(&self) -> Vector2 {
        self.position
    }

    pub fn is_activated(&self) -> bool {
        if self.is_activated {
            self.timer.elapsed().as_secs() < self.time_to_live
        } else {
            false
        }
    }

    pub fn activate(&mut self) {
        self.is_activated = true;
        self.timer = time::Instant::now();
    }

    pub fn get_type(&self) -> Type {
        self.t
    }

    pub fn modifier(&self) -> f32 {
        self.modifier
    }
}
