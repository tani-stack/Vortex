pub struct Waypoint { pub lat: f64, pub lon: f64, pub alt: f32 } pub struct Path { pub wps: [Waypoint; 32], pub len: usize }
