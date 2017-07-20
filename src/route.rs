use ::rocket::request::LenientForm;
use ::rocket_contrib::Json;
use ::airport::{self, Airport};

#[derive(FromForm, Debug)]
pub struct FilterData {
    mach: f32,
    dep_icao: Option<airport::ICAO>,
    dep_size: Option<airport::Size>,
    arr_icao: Option<airport::ICAO>,
    arr_size: Option<airport::Size>,
}

#[derive(Debug, Serialize)]
pub struct Route {
    pub departure: Airport,
    pub arrival:   Airport,
    pub distance:  f64,
}

impl Route {
    pub fn create(departure: Airport, arrival: Airport) -> Route {
        let mut route = Route {
            departure,
            arrival,
            distance: 0.0,
        };

        route.distance = route.calculate_distance();
        route
    }

    pub fn calculate_distance(&self) -> f64 {
        let radius = 3440.; // Earth's radius in nautical miles

        let lat1 = self.departure.pos.lat.to_radians();
        let lat2 = self.arrival.pos.lat.to_radians();
        let lat  = lat2 - lat1;
        let lon  = (self.arrival.pos.lon - self.departure.pos.lon).to_radians();

        let a = (lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        radius * c
    }
}

#[post("/filter", data = "<filters>")]
pub fn parse_filters(filters: LenientForm<FilterData>) -> Json<Vec<Route>> {
    println!("{:?}", filters);
    Json(vec![
        Route::create(
            Airport {
                pos: airport::LatLon::new(38.695, -121.589),
                icao: airport::ICAO("KSMF".into()),
            },
            Airport {
                pos: airport::LatLon::new(47.45, -122.3),
                icao: airport::ICAO("KSEA".into()),
            },
        ),
        Route::create(
            Airport {
                pos: airport::LatLon::new(37.62, -122.378),
                icao: airport::ICAO("KSFO".into()),
            },
            Airport {
                pos: airport::LatLon::new(40.64, -73.776),
                icao: airport::ICAO("KJFK".into()),
            }
        ),
    ])
}