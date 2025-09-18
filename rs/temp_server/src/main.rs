use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use chrono::Local;

const DEVICE_FILE: &str = "/sys/bus/w1/devices/28-000001cda180/w1_slave";

fn format_date() -> String {
    let now = Local::now();
    now.format("%a %b %d %I:%M:%S %p %Z %Y").to_string()
}

#[get("/")]
async fn index() -> impl Responder {
    let current_date = format_date();
    let formatted_temp = match read_temp() {
        Some(temp) => format!("{:.3}°C", temp),
        None => "N/A".to_string(),
    };
    let html = format!(
        "<!DOCTYPE html>\n\
         <html lang=\"en\">\n\
         <head>\n\
             <title>FrambOS Temperature</title>\n\
             <meta http-equiv=\"refresh\" content=\"10\">\n\
         </head>\n\
         <body>\n\
             <h1>Temperature on FrambOS</h1>\n\
             On FrambOS right now, the date is {} and the temperature is {}\n\
         </body>\n\
         </html>",
        current_date, formatted_temp
    );
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

fn read_temp() -> Option<f32> {
    let file = File::open(DEVICE_FILE).ok()?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    if lines.len() >= 2 && lines[0].ends_with("YES") {
        if let Some(temp_str) = lines[1].split("t=").nth(1) {
            if let Ok(temp) = temp_str.trim().parse::<f32>() {
                return Some(temp / 1000.0);
            }
        }
    }
    None
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_format_date() {
        let now = Local::now();
        let current_date = format_date();
        assert!(current_date.contains(now.format("%a").to_string().as_str()), "Failed to contain weekday: {}", current_date);
        assert!(current_date.contains(now.format("%b").to_string().as_str()), "Failed to contain month: {}", current_date);
        assert!(current_date.contains(now.format("%d").to_string().as_str()), "Failed to contain day: {}", current_date);
        assert!(current_date.contains(now.format("%Y").to_string().as_str()), "Failed to contain year: {}", current_date);
    }
}
