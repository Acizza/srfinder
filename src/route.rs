use ::rocket::request::LenientForm;
use ::rocket_contrib::Json;
use ::airport::{self, Airport, Runway, RunwayIdentifier, Frequencies};

#[derive(FromForm, Debug)]
pub struct FilterData {
    mach: f32,
    dep_icao: Option<airport::ICAO>,
    dep_type: Option<airport::Type>,
    arr_icao: Option<airport::ICAO>,
    arr_type: Option<airport::Type>,
}

#[derive(Debug, Serialize)]
pub struct Route {
    pub departure: Airport,
    pub arrival:   Airport,
    pub distance:  f32,
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

    pub fn calculate_distance(&self) -> f32 {
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
                icao: "KSMF".into(),
                pos: airport::LatLon::new(38.695, -121.589),
                _type: airport::Type::Large,
                runways: Some(vec![
                    Runway {
                        ident: RunwayIdentifier {
                            north: "16R".into(),
                            south: "34R".into(),
                        },
                        width: Some(150),
                        length: Some(8600),
                        closed: Some(false),
                    }
                ]),
                frequencies: Some(Frequencies {
                    ground:    Some("121.7".into()),
                    tower:     Some("125.7".into()),
                    departure: Some("125.25".into()),
                    approach:  Some("125.25".into()),
                    atis:      Some("128.4".into()),
                }),
            },
            Airport {
                pos: airport::LatLon::new(47.45, -122.3),
                icao: "KSEA".into(),
                _type: airport::Type::Large,
                runways: Some(vec![
                    Runway {
                        ident: RunwayIdentifier {
                            north: "16R".into(),
                            south: "34R".into(),
                        },
                        width: Some(160),
                        length: Some(8800),
                        closed: Some(false),
                    }
                ]),
                frequencies: Some(Frequencies {
                    ground:    Some("121.7".into()),
                    tower:     Some("125.7".into()),
                    departure: Some("125.25".into()),
                    approach:  Some("125.25".into()),
                    atis:      Some("128.4".into()),
                }),
            },
        ),
    ])
}