use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interval {
    pub begin: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub time: String,
    pub name: String,
    pub color: Color,
    pub daily_repetitions: u32,
    pub r#type: String,
    pub interval: Interval,
}
