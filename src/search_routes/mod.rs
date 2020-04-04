mod filter;

use actix_web::{post, web, HttpResponse};
use filter::Filters;
use serde_derive::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Serialize)]
pub struct Route {
    pub from: SmolStr,
    pub to: SmolStr,
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
struct RouteResponse {
    routes: Vec<Route>,
}

#[post("/search_routes")]
pub async fn search_routes(filters: web::Json<Filters>) -> HttpResponse {
    println!("{:?}", filters);

    let test_response = RouteResponse {
        routes: vec![Route {
            from: "RJAA".into(),
            to: "KSFO".into(),
            distance: 5326.2,
            time: Time::new(13, 24),
        }],
    };

    HttpResponse::Ok().json(test_response)
}
