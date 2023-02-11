use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::util::UnitType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuakeMag {
    pub md: Option<f32>,
    pub ml: f32,
    pub mw: Option<f32>,
}

impl QuakeMag {
    pub fn get_main(&self) -> f32 {
        self.ml
    }

    fn parse_mag(s: &str) -> Option<f32> {
        match s {
            "_._" => None,
            val => f32::from_str(val).ok(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Coord {
    pub latitude: f32,  // should contain
    pub longitude: f32, // should contain
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuakeData {
    pub date: String, // should contain
    pub time: String, // should contain
    pub coord: Coord,
    pub depth: Option<f32>,
    pub magnitude: QuakeMag,
    pub location: String, // can be empty
    pub metadata: String, // can be empty
}

impl QuakeData {
    pub fn same_instance(&self, other: &Self) -> bool {
        let date_eq = self.date.eq(&other.date);
        let time_eq = self.time.eq(&other.time);
        let coord_eq = self.coord.eq(&other.coord);

        date_eq && time_eq && coord_eq
    }
}

#[derive(Debug)]
pub struct ParseError;
impl UnitType for ParseError {
    fn unit_create() -> Self {
        ParseError
    }
}

impl FromStr for QuakeData {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut words = line.split_whitespace();

        let date = words.next().ok_or_else(|| ParseError)?.to_string();
        let time = words.next().ok_or_else(|| ParseError)?.to_string();

        let latitude = words
            .next()
            .map(f32::from_str)
            .ok_or_else(|| ParseError)?
            .map_err(|_| ParseError)?;
        let longitude = words
            .next()
            .map(f32::from_str)
            .ok_or_else(|| ParseError)?
            .map_err(|_| ParseError)?;
        let coord = Coord {
            latitude,
            longitude,
        };

        let depth = words.next().map(f32::from_str).map(Result::ok).flatten();

        let md = words.next().map(QuakeMag::parse_mag).flatten();
        let ml = words
            .next()
            .map(f32::from_str)
            .ok_or_else(|| ParseError)?
            .map_err(|_| ParseError)?;
        let mw = words.next().map(QuakeMag::parse_mag).flatten();
        let magnitude = QuakeMag { md, ml, mw };

        let loc1 = words.next().unwrap_or_else(|| "Unknown");
        let loc2 = words.next();
        let meta = words.fold(None::<String>, |acc, w| {
            match acc {
                Some(mut acc) => {
                    acc.push_str(w);
                    acc.push(' ');
                    Some(acc)
                },
                None => Some(w.to_string())
            }
        });

        let mut location = String::from(loc1);
        let metadata = match (loc2, meta) {
            (Some(loc2), Some(meta)) => {
                location += &" ";
                location += loc2;
                meta.to_string()
            },
            (Some(meta), None) => meta.to_string(),
            _ => "".to_string(),
        };

        Ok(QuakeData {
            date,
            time,
            coord,
            depth,
            magnitude,
            location,
            metadata,
        })
    }
}

pub fn process_quake_data(data: String) -> Vec<QuakeData> {
    let mut column_headers_passed = false;
    let mut dash_line_passed = false;

    data.trim()
        .lines()
        .skip_while(|line| {
            // Skip header lines
            if column_headers_passed && dash_line_passed {
                false
            } else if line.starts_with("Tarih") {
                column_headers_passed = true;
                true
            } else if column_headers_passed && line.starts_with("-") {
                dash_line_passed = true;
                true
            } else {
                true
            }
        })
        .map(QuakeData::from_str)
        .filter(|q| q.is_ok())
        .map(|q| q.unwrap())
        .collect()
}
