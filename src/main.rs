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
    const VALID_COMMANDS: &[&str] = &["bright", "dim", "warm", "cold", "on", "off", "max"];
    let error_message = "Usage: elgato-rs http://keylight.lan <bright|dim|warm|cold|on|off|max>";

    if args.len() == 3 {
        let url = args[1].clone();
        let command = args[2].clone();

        if VALID_COMMANDS.contains(&command.as_str()) {
            let url = format!("{}:9123/elgato/lights", url);
            Ok((url, command))
        } else {
            Err(error_message)
        }
    } else {
        Err(error_message)
    }
}

fn adjust_light(light: &mut Light, command: &str) {
    const BRIGHTNESS_MAX: u8 = 100;
    const BRIGHTNESS_MIN: u8 = 0;

    const TEMPERATURE_MAX: u16 = 344;
    const TEMPERATURE_MIN: u16 = 143;
    const TEMPERATURE_STEP: u16 = 10;

    // gamma-ish
    let brightness_step: u8 = if light.brightness <= 12 { 1 } else { 3 };

    match command {
        "bright" => {
            light.brightness = light
                .brightness
                .saturating_add(brightness_step)
                .min(BRIGHTNESS_MAX)
                .max(2); // 1 seems to be 'off' also
            light.on = 1;
        }
        "dim" => {
            light.brightness = light
                .brightness
                .saturating_sub(brightness_step)
                .max(BRIGHTNESS_MIN); // redundant but consistent
            if light.brightness <= 1 {
                light.on = 0;
                light.brightness = 0;
            };
        }
        "warm" => {
            light.temperature = light
                .temperature
                .saturating_add(TEMPERATURE_STEP)
                .min(TEMPERATURE_MAX);
        }
        "cold" => {
            light.temperature = light
                .temperature
                .saturating_sub(TEMPERATURE_STEP)
                .max(TEMPERATURE_MIN);
        }
        "on" => {
            light.on = 1;
            if light.brightness == 0 {
                light.brightness = 50;
            }
        }
        "off" => {
            light.on = 0;
        }
        "max" => {
            light.on = 1;
            light.temperature = 250;
            light.brightness = BRIGHTNESS_MAX;
        }
        _ => (),
    }
}
