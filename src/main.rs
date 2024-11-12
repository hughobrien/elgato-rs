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
#[allow(non_snake_case)]
struct LightsResponse {
    numberOfLights: u8,
    lights: Vec<Light>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (url, command) = parse_args(env::args().collect())?;

    let client = Client::new();
    let lights_response = client
        .get(&url)
        .send()
        .await?
        .json::<LightsResponse>()
        .await?;

    if let Some(mut light) = lights_response.lights.first().cloned() {
        adjust_light(&mut light, &command);

        let req_body = LightsResponse {
            numberOfLights: lights_response.numberOfLights,
            lights: vec![light],
        };

        client.put(&url).json(&req_body).send().await?;
    } else {
        eprintln!("No lights found");
    }

    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(String, String), &'static str> {
    const VALID_COMMANDS: &[&str] = &["bright", "dim", "warm", "cold"];
    let error_message =
        "Usage: elgato-rs http://elgato.lan:9123/elgato/lights <bright|dim|warm|cold>";

    if args.len() == 3 {
        let url = args[1].clone();
        let command = args[2].clone();

        if VALID_COMMANDS.contains(&command.as_str()) {
            Ok((url, command))
        } else {
            Err(error_message)
        }
    } else {
        Err(error_message)
    }
}

fn adjust_light(light: &mut Light, command: &str) {
    const BRIGHTNESS_STEP: u8 = 3;
    const TEMPERATURE_STEP: u16 = 10;

    match command {
        "bright" => light.brightness = light.brightness.saturating_add(BRIGHTNESS_STEP).min(100),
        "dim" => light.brightness = light.brightness.saturating_sub(BRIGHTNESS_STEP).max(0),
        "warm" => light.temperature = light.temperature.saturating_add(TEMPERATURE_STEP).min(344),
        "cold" => light.temperature = light.temperature.saturating_sub(TEMPERATURE_STEP).max(143),
        _ => (),
    }

    light.on = if light.brightness > 0 { 1 } else { 0 };
}
