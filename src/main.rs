#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

use core::time::Duration;
use log::info;
use uefi::{
    Identify, Result,
    boot::{SearchType, locate_handle_buffer},
    prelude::*,
    proto::{
        console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput},
        device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly},
        loaded_image::LoadedImage,
    },
};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello world!");
    let gop_handle = match boot::get_handle_for_protocol::<GraphicsOutput>() {
        Ok(h) => {
            info!("Found gop handle");
            h
        }
        _ => {
            info!("Fuck");
            panic!()
        }
    };
    match boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
        Ok(p) => info!("Opened gop"),
        _ => {
            info!("Fuck");
            panic!()
        }
    }

    show_boot_logo().unwrap();
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

struct Buffer {
    width: usize,
    height: usize,
    buffer: Vec<BltPixel>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            buffer: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }

    pub fn blit(&mut self, gop: &mut GraphicsOutput) -> Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.buffer,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }

    pub fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.buffer.get_mut(y * self.width + x)
    }
    pub fn blit_pixel(&self, x: usize, y: usize, gop: &mut GraphicsOutput) -> Result {
        let coords = (x, y);
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.buffer,
            src: BltRegion::SubRectangle {
                coords,
                px_stride: self.width,
            },
            dest: coords,
            dims: (1, 1),
        })
    }
}

fn show_boot_logo() -> Result {
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width, height);

    for y in 0..buffer.height {
        let r = ((y as f32) / (buffer.height as f32)) * 255.0;
        for x in 0..buffer.width {
            let g = ((x as f32) / (buffer.width as f32)) * 255.0;
            let pixel = buffer
                .pixel(x, y)
                .expect(&format!("No pixel at these coordinates ({}, {})", x, y));
            pixel.red = r as u8;
            pixel.green = g as u8;
            pixel.blue = 255;
        }
    }

    buffer.blit(&mut gop)?;

    Ok(())
}
