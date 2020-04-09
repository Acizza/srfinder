mod filter;

use crate::airport_data::Airport;
use actix_web::{post, web, HttpResponse};
use filter::Filters;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Route<'a> {
    pub from: &'a Airport,
    pub to: &'a Airport,
    pub distance: f32,
    pub time: Time,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Time {
    pub hour: u8,
    pub minutes: u8,
}

impl Time {
    #[inline(always)]
    pub fn new(hour: u8, minutes: u8) -> Self {
        Self {
            hour: hour.min(99),
            minutes: minutes.min(59),
        }
    }
}

#[derive(Serialize)]
struct Response<'a> {
    routes: Vec<Route<'a>>,
}

#[post("/search_routes")]
pub async fn search_routes(
    filters: web::Json<Filters>,
    airports: web::Data<Vec<Airport>>,
) -> HttpResponse {
    println!("{:?}", filters);

    let departure = airports.iter().find(|a| a.icao == "KSFO").unwrap();
    let arrival = airports.iter().find(|a| a.icao == "RJAA").unwrap();

    let route = Route {
        from: departure,
        to: arrival,
        distance: 5300.0,
        time: Time::new(13, 30),
    };

    let resp = Response {
        routes: vec![route],
    };

    HttpResponse::Ok().json(resp)
}
