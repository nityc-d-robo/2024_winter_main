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
use motor_lib::{
    USBHandle,
    Error,
    md,
    GrpcHandle
};

fn main() -> Result<(), DynError> {
    let handle = GrpcHandle::new("http://127.0.0.1:50051");
    let _logger = Logger::new("robot_2");
    let ctx = Context::new()?;
    let node = ctx.create_node("robot_2", None, Default::default())?;
    let mut selector = ctx.create_selector()?;
    let subscriber = node.create_subscriber::<sensor_msgs::msg::Joy>("joy_2", None)?;

    selector.add_subscriber(subscriber, {
        Box::new(move |msg| {
            let mut p9n = p9n_interface::PlaystationInterface::new(&msg);
            p9n.set_joy_msg(&msg);

            if p9n.pressed_l1() {
                pr_info!(_logger, "pressed: L1");
            }
            if p9n.pressed_l2() {
                pr_info!(_logger, "pressed: L2");
            }
            if p9n.pressed_r1() {
                pr_info!(_logger, "pressed: R1");
            }
            if p9n.pressed_r2() {
                pr_info!(_logger, "pressed: R2");
            }
            if p9n.pressed_cross() {
                pr_info!(_logger, "pressed: X");
            }
            else if p9n.pressed_triangle() {
                pr_info!(_logger, "pressed: triangle");
            }

            if p9n.pressed_circle() {
                pr_info!(_logger, "pressed: circle");
            }
            else if p9n.pressed_square() {
                pr_info!(_logger, "pressed: square");
            }
            
            if p9n.pressed_dpad_down() {
                md::send_pwm(&handle, 3, 800);
                pr_info!(_logger, "pressed: dpad_down");
            }
            else if p9n.pressed_dpad_left() {
                pr_info!(_logger, "pressed: dpad_left");
            }
            else if p9n.pressed_dpad_right() {
                pr_info!(_logger, "pressed: dpad_right");
            }
            else if p9n.pressed_dpad_up() {
                md::send_pwm(&handle, 3, -800);
                pr_info!(_logger, "pressed: dpad_up");
            } else {
                md::send_pwm(&handle, 3, 0);
            }
        })
    });
    loop {
        selector.wait()?;
    }
}
