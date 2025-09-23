use actix_rt::time;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;
#[derive(Clone)]
struct Measurement {
    t: DateTime<Local>,
    v: f32,
}
#[get("/")]
async fn index(mes: web::Data<Mutex<VecDeque<Measurement>>>) -> impl Responder {
    let v = { mes.lock().unwrap().clone() };
    let mut msg = String::new();
    for m in v.iter() {
        msg.push_str(&format!(" {:>8.2}", m.v));
    }
    msg.push_str("\n");
    for m in v.iter() {
        msg.push_str(&format!(" {}", m.t.format("%H:%M:%S")));
    }
    msg.push_str("\n");
    HttpResponse::Ok().content_type("text/plain").body(msg)
}
async fn bg_task(m: web::Data<Mutex<VecDeque<Measurement>>>) {
    let mut interval = time::interval(Duration::from_secs(1));
    const MAX_READINGS: usize = 5;
    let mut i = 0;
    loop {
        interval.tick().await;
        i += 1;
        let temp = 24.750 + (i as f32).sin() * 0.5;
        let mut v = m.lock().unwrap();
        v.push_back(Measurement {
            t: Local::now(),
            v: temp,
        });
        if v.len() > MAX_READINGS {
            v.pop_front();
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Mutex::new(VecDeque::new()));
    actix_rt::spawn(bg_task(app_state.clone()));
    HttpServer::new(move || App::new().app_data(app_state.clone()).service(index))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
