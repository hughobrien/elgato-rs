use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Clone)]
struct Light {
    on: u8,
    brightness: u8,
    temperature: u16,
}

#[derive(Deserialize, Serialize)]
struct LightsResponse {
    numberOfLights: u8,
    lights: Vec<Light>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || !matches!(args[1].as_str(), "up" | "down") {
        eprintln!("Usage: <program> <up|down>");
        return Ok(());
    }
    let direction = &args[1];

    let url = "http://elgato.lan:9123/elgato/lights";
    let client = Client::new();

    let res = client.get(url).send().await?;
    let lights_response: LightsResponse = res.json().await?;
    
    let mut light = lights_response.lights.first().cloned().ok_or("No lights found")?;
    adjust_brightness(&mut light, direction);

    light.on = if light.brightness > 0 { 1 } else { 0 };

    let req_body = LightsResponse {
        numberOfLights: lights_response.numberOfLights,
        lights: vec![light],
    };

    client.put(url).json(&req_body).send().await?;

    Ok(())
}

fn adjust_brightness(light: &mut Light, direction: &str) {
    let step = 3;
    match direction {
        "up" => light.brightness = light.brightness.saturating_add(step).min(100),
        "down" => light.brightness = light.brightness.saturating_sub(step).max(0),
        _ => (),
    }
}

