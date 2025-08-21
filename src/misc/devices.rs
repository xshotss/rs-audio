/*!
The devices module's purpose is to make interacting with audio devices easier.<br>
You can use it to check all devices and check if the user has a valid output device.<br><br>

# Usage
A simple program using this module:
```
use rs_audio::misc::devices::*;


if !has_valid_device() {
    eprintln!("No valid audio output device detected!");
    std::process::exit(1);
}

for (key, device) in all_devices().iter().enumerate() {
    println!("{}: {}", key, device.name())
}

```
*/

use rodio::{cpal::traits::HostTrait, Device};

/**
Checks if the host has a valid output device (ie. headphones, speakers, etc...).<br><br>

## Usage
Example:
```
use rs_audio::misc::devices::{has_valid_device};

if has_valid_device() {
    println!("You have a valid audio output device!");
}
else {
    println!("Couldn't find valid audio output device!");
}
```
*/
pub fn has_valid_device() -> bool {
    // finds the default host
    let host = rodio::cpal::default_host();

    if host.output_devices().is_ok() {
        return true;
    }
    false
}

/**
Finds the default host's devices and outputs a vector of `Device` structs.<br>
Note that this relies on `cpal`.<br>

# Panics
This function will panic if it cannot find any output device.<br>
It is recommended to pair this with the `has_valid_device()` function.
```
use rs_audio::misc::devices::{has_valid_device, all_devices};

if has_valid_device() {
    let devices = all_devices();
    // Do something with the devices here...
    // Simple example:
    for device in devices {
        println!("{}", device.name());
    }
}
```
*/
pub fn all_devices() -> Result<Vec<Device>, Box<dyn std::error::Error>> {
    let mut result: Vec<Device> = Vec::new();
    let host = rodio::cpal::default_host();

    for device in host.output_devices()? {
        result.push(device);
    }

    Ok(result)
}
