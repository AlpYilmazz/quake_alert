use std::f64::consts::PI;

use crate::process::Coord;

pub fn interpolate(low: f32, high: f32, factor: f32) -> f32 {
    low + ((high - low) * factor)
}

fn to_radians(deg: f64) -> f64 {
    deg * (PI / 180.0)
}

fn pow2(num: f64) -> f64 {
    num * num
}

pub fn earth_dist_km(coord1: Coord, coord2: Coord) -> f64 {
    let lat1 = to_radians(coord1.latitude as f64);
    let long1 = to_radians(coord1.longitude as f64);
    let lat2 = to_radians(coord2.latitude as f64);
    let long2 = to_radians(coord2.longitude as f64);

    // Haversine Formula
    let dlat = lat2 - lat1;
    let dlong = long2 - long1;

    let mut ans =
        pow2(f64::sin(dlat / 2.0)) + f64::cos(lat1) * f64::cos(lat2) * pow2(f64::sin(dlong / 2.0));

    ans = 2.0 * f64::asin(f64::sqrt(ans));

    // Radius of Earth in
    // Kilometers, R = 6371
    // Use R = 3956 for miles
    const R: f64 = 6371.0;

    // Calculate the result
    ans = ans * R;

    return ans;
}

pub trait Sink {
    fn sink(self) -> ();
}
impl<T> Sink for T {
    fn sink(self) -> () {
        ()
    }
}

pub trait UnitType {
    fn unit_create() -> Self;
}

pub trait Spring<T> {
    fn spring(self) -> T;
}
impl<T: UnitType> Spring<T> for () {
    fn spring(self) -> T {
        T::unit_create()
    }
}

pub trait UnitMap<U: UnitType>
where
    Self: Sink,
{
    fn unit_map(self) -> U;
}
impl<T: Sink, U: UnitType> UnitMap<U> for T {
    fn unit_map(self) -> U {
        self.sink().spring()
    }
}

#[allow(unused_macros)]
macro_rules! todox {
    ($($expr:expr),*) => {
        {
            $(
                let _ = $expr;
            )*
            todo!();
        }
    };
}
