use std::io::Write;
use std::thread;
use std::time::Duration;
mod p9n_interface;
mod ps5_dualsense;
//use log::{error, info};
//use sensor_msgs::msg::JoyMessage;
use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    //selector::Selector,
    msg::common_interfaces::sensor_msgs, pr_info,
};

fn main() -> Result<(), DynError> {
    // env_logger::init();
    let mut serial = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(100))
        .open()
        .unwrap();
    let _logger = Logger::new("robot_1");
    let ctx = Context::new()?;
    let node = ctx.create_node("robot_1", None, Default::default())?;
    let mut selector = ctx.create_selector()?;
    let subscriber = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;

    // let joy_msg = sensor_msgs::msg::Joy::new().ok_or_else(|| {
    //     error!("Failed to create Joy message");
    //     Box::new(DynError::new("Failed to create Joy message")) as Box<dyn std::error::Error>
    // })?;

    selector.add_subscriber(subscriber, {
        Box::new(move |msg| {
            let mut p9n = p9n_interface::PlaystationInterface::new(&msg);
            p9n.set_joy_msg(&msg);

            if p9n.pressed_l1() {
                pr_info!(_logger, "pressed: l1");
            }
            if p9n.pressed_l2() {
                pr_info!(_logger, "pressed: l2");
            }
            if p9n.pressed_r1() {
                pr_info!(_logger, "pressed: r1");
            }
            if p9n.pressed_r2() {
                pr_info!(_logger, "pressed: r2");
            }
            if p9n.pressed_cross() {
                serial.write(b"f").expect("Failed to write to serial port");
                println!("Button F pressed, data sent: f");
                pr_info!(_logger, "pressed: cross");
            }
            else if p9n.pressed_triangle() {
                serial.write(b"g").expect("Failed to write to serial port");
                println!("Button G pressed, data sent: g");
                pr_info!(_logger, "pressed: triangle");
            }else{
            serial.write(b"h").expect("Failed to write to serial port");
            println!("Button H pressed, data sent: h");
            }
            if p9n.pressed_circle() {
                pr_info!(_logger, "pressed: circle");
            }
            else if p9n.pressed_square() {
                pr_info!(_logger, "pressed: square");
            }
            
            if p9n.pressed_dpad_down() {
                serial.write(b"b").expect("Failed to write to serial port");
                println!("Button B pressed, data sent: b");
                pr_info!(_logger, "pressed: dpad_down");
            }
            else if p9n.pressed_dpad_left() {
                serial.write(b"d").expect("Failed to write to serial port");
                println!("Button D pressed, data sent: d");
                pr_info!(_logger, "pressed: dpad_left");
            }
            else if p9n.pressed_dpad_right() {
                serial.write(b"c").expect("Failed to write to serial port");
                println!("Button C pressed, data sent: c");
                pr_info!(_logger, "pressed: dpad_right");
            }
            else if p9n.pressed_dpad_up() {
                serial.write(b"a").expect("Failed to write to serial port");
                println!("Button A pressed, data sent: a");
                pr_info!(_logger, "pressed: dpad_up");
            }else{
            serial.write(b"e").expect("Failed to write to serial port");
            println!("Button E pressed, data sent: e");
            }
        })
    });
    loop {
        selector.wait()?;
    }
}
