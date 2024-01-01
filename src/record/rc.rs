use binrw::binread;

use crate::layout::info::ProductType;
use crate::utils::sub_byte_field;

#[binread]
#[derive(Debug)]
#[br(little, import { product_type: ProductType = ProductType::None })]
pub struct RC {
    /// right stick - horizontal
    #[br(map = |x: u16| (x as f32 - 1024.0) / 0.66)]
    pub aileron: f32,
    /// right stick - vertical
    #[br(map = |x: u16| (x as f32 - 1024.0) / 0.66)]
    pub elevator: f32,
    /// left stick - vertical
    #[br(map = |x: u16| (x as f32 - 1024.0) / 0.66)]
    pub throttle: f32,
    /// left stick - horizontal
    #[br(map = |x: u16| (x as f32 - 1024.0) / 0.66)]
    pub rudder: f32,
    #[br(map = |x: u16| (x as f32 - 1024.0) / 0.66)]
    pub gimbal: f32,

    #[br(temp)]
    _bitpack1: u8,
    #[br(calc(sub_byte_field(_bitpack1, 0x01)))]
    pub wheel_btn_down: u8,
    #[br(calc(sub_byte_field(_bitpack1, 0x3E)))]
    pub wheel_offset: u8,
    #[br(calc(sub_byte_field(_bitpack1, 0x40)))]
    pub wheel_polarity: u8,
    #[br(calc(sub_byte_field(_bitpack1, 0x80)))]
    pub wheel_change: u8,

    #[br(temp)]
    _bitpack2: u8,
    #[br(calc(sub_byte_field(_bitpack2, 0x07)))]
    pub transform_btn_reserve: u8,
    #[br(calc(sub_byte_field(_bitpack2, 0x08)))]
    pub return_btn: u8,
    #[br(calc(FlightModeSwitch::from(sub_byte_field(_bitpack2, 0x30), product_type)))]
    pub flight_mode_switch: FlightModeSwitch,
    #[br(calc(sub_byte_field(_bitpack2, 0xC0)))]
    pub transform_switch: u8,

    #[br(temp)]
    _bitpack3: u8,
    #[br(calc(sub_byte_field(_bitpack3, 0x02)))]
    pub custom_function_btn4_down: u8,
    #[br(calc(sub_byte_field(_bitpack3, 0x04)))]
    pub custom_function_btn3_down: u8,
    #[br(calc(sub_byte_field(_bitpack3, 0x08)))]
    pub custom_function_btn2_down: u8,
    #[br(calc(sub_byte_field(_bitpack3, 0x10)))]
    pub custom_function_btn1_down: u8,
    #[br(calc(sub_byte_field(_bitpack3, 0x20)))]
    pub playback_btn_down: u8,
    #[br(calc(sub_byte_field(_bitpack3, 0x40)))]
    pub shutter_btn_down: u8,
    #[br(calc(sub_byte_field(_bitpack3, 0x80)))]
    pub record_btn_down: u8,

    pub bandidth: u8,
    pub gimbal_control_enable: u8,
}

#[derive(Debug)]
pub enum FlightModeSwitch {
    /// Position One. For all products except Mavic Pro, this is the left most position
    /// of the flight mode switch on a remote controller from the perspective of the
    /// pilot. For example, on a Phantom 4 remote controller,  Position One is labeled
    /// "A". For Mavic Pro, Spark and Mavic Air, this is  the position that is furthest
    /// away from the pilot and labeled "Sport".
    One,
    /// Position Two. For all products except Mavic Pro, this is the middle position of
    /// the flight mode switch on a remote controller from the perspective of the pilot.
    /// For example, on a Phantom 4 remote controller, Position Two is labeled "S". For
    /// Mavic Pro, Spark and Mavic Air, this is the position that is closest  to the
    /// pilot [the P position].
    Two,
    /// Position Three. For all products except Mavic Pro, this is the right most
    /// position of the flight mode switch on a remote controller from the perspective
    /// of the pilot. For example, on a Phantom 4 remote controller, Position Two is
    /// labeled "P". Mavic Pro, Spark or Mavic Air does not have  a third position for
    /// the flight mode switch.
    Three,
    Unknown(u8),
}

impl FlightModeSwitch {
    fn from(value: u8, product_type: ProductType) -> Self {
        let mapped_value = match product_type {
            // Remap values for Mavic Pro
            ProductType::MavicPro => match value {
                0 => 2,
                1 => 3,
                2 => 1,
                _ => value,
            },
            _ => value,
        };

        match mapped_value {
            0 => FlightModeSwitch::One,
            1 => FlightModeSwitch::Two,
            2 => FlightModeSwitch::Three,
            other => FlightModeSwitch::Unknown(other),
        }
    }
}
