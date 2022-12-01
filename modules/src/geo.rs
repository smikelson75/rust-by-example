// Create a mod within the geo mod
// mod must be marked public (pub) in order
// for it to be imported into other Rust
// files
pub mod calculations {
    const EARTH_RADIUS_IN_KILOMETERS:f64 = 6371.0;

    // Create a function within the calculations mod
    // Again, this must be marked as public (pub) in 
    // order for it to be used in other Rust files
    pub fn distance(start_latitude: f64, end_latitude: f64, start_longitude: f64, end_longitude: f64) -> f64 {
        let start_radians = start_latitude.to_radians();
        let end_radians = end_latitude.to_radians();
        let delta_latitude = (start_latitude - start_latitude).to_radians();
        let delta_longitude = (start_longitude - end_longitude).to_radians();
        let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
            + start_radians.cos()
            * end_radians.cos()
            * f64::powi((delta_longitude / 2.0).sin(), 2);
        let central_angle = 2.0 * inner_central_angle.sqrt().asin();
        let distance = EARTH_RADIUS_IN_KILOMETERS  * central_angle;
        distance
    }
}