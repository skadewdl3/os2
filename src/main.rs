#![no_std]
#![no_main]

use core::time::Duration;
use log::info;
use uefi::{
    Identify, Result,
    boot::{SearchType, locate_handle_buffer},
    prelude::*,
    proto::{
        device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly},
        loaded_image::LoadedImage,
    },
};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello world!");
    print_image_details().unwrap();
    boot::stall(Duration::from_secs(10));
    Status::SUCCESS
}

fn print_image_details() -> Result {
    // Get handle to bootloader image
    let handle = boot::image_handle();
    let loaded_image = boot::open_protocol_exclusive::<LoadedImage>(handle)?;
    let device_path_to_text_handle =
        *locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
            .first()
            .expect("DevicePathToText is missing");

    let device_path_to_text =
        boot::open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle)?;

    let image_device_path = loaded_image.file_path().expect("File path is not set");
    let image_device_path_text = device_path_to_text
        .convert_device_path_to_text(image_device_path, DisplayOnly(true), AllowShortcuts(false))
        .expect("convert_device_path_to_text failed");

    info!("Image path: {}", &*image_device_path_text);

    Ok(())
}
