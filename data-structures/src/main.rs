#![allow(unused)]

// Normal structures implemented in C, C++, C# etc.
struct WayPoint {
    name: String,
    latitude: f32,
    longitude: f32,
}

struct Segment {
    start: WayPoint,
    end: WayPoint,
}

// Rust doesn't have a concept of Object. However, functionality can be 
// implemented. impl <struct> allows functions to be tied to specific
// structs. The struct can be referenced by the use of Self keyword.
impl Segment {
    fn new(start: WayPoint, end: WayPoint) -> Self {
        Self {
            start,
            end,
        }
    }

    // What is normally a Method, distance takes in an implicit
    // self struct. This is not provided in the call, it is implicitly
    // added.
    fn distance(&self) -> f32 {
        const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
        let start_radians = self.start.latitude.to_radians();
        let end_radians = self.end.latitude.to_radians();
        let delta_latitude = (self.start.latitude - self.end.latitude).to_radians();
        let delta_longitude = (self.start.longitude - self.end.longitude).to_radians();
        let inner_central_angle = f32::powi((delta_latitude / 2.0).sin(), 2)
            + start_radians.cos()
            * end_radians.cos()
            * f32::powi((delta_longitude / 2.0).sin(), 2);
        let central_angle = 2.0 * inner_central_angle.sqrt().asin();
        let distance = EARTH_RADIUS_IN_KILOMETERS as f32 * central_angle;
        distance
    }
}

fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    // Build the struct WayPoint
    let mut kcle = WayPoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111,
    };

    // structs can be built using existing structs, where individual
    // items within the struct can be overridden.
    // let mut kslc = WayPoint {
    //     name: "KSLC".to_string(),
    //     ..kcle
    // };

    let mut kslc = WayPoint {
        name: "KSLC".to_string(),
        latitude: 40.7861,
        longitude: -111.9822
    };

    // Create a new Segment impl which is a combination of the struct values,
    // with assigned functions
    let kcle_kslc = Segment::new(kcle, kslc);

    // Calling an impl function tied to a struct that returns a value
    let distance = kcle_kslc.distance();
    println!("{:.1}", distance);
}
