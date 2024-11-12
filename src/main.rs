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
    let direction = parse_direction(env::args().collect())?;
    let url = "http://elgato.lan:9123/elgato/lights";

    let client = Client::new();
    let lights_response = client.get(url).send().await?.json::<LightsResponse>().await?;
    
    if let Some(mut light) = lights_response.lights.first().cloned() {
        adjust_brightness(&mut light, direction);
        
        let req_body = LightsResponse {
            numberOfLights: lights_response.numberOfLights,
            lights: vec![light],
        };
        
        client.put(url).json(&req_body).send().await?;
    } else {
        eprintln!("No lights found");
    }

    Ok(())
}

fn parse_direction(args: Vec<String>) -> Result<&'static str, &'static str> {
    if args.len() == 2 {
        match args[1].as_str() {
            "up" => Ok("up"),
            "down" => Ok("down"),
            _ => Err("Usage: <program> <up|down>"),
        }
    } else {
        Err("Usage: <program> <up|down>")
    }
}

fn adjust_brightness(light: &mut Light, direction: &str) {
    const STEP: u8 = 3;
    match direction {
        "up" => light.brightness = light.brightness.saturating_add(STEP).min(100),
        "down" => light.brightness = light.brightness.saturating_sub(STEP).max(0),
        _ => (),
    }
    light.on = if light.brightness > 0 { 1 } else { 0 };
}

