#[allow(unused_imports)]
use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    pr_info,
    topic::publisher::Publisher,
};

use motor_lib::{
    USBHandle,
    Error,
    md,
    GrpcHandle
};

use safe_drive::msg::common_interfaces::geometry_msgs::msg;
use ndarray::prelude::{arr1, arr2};
use std::collections::HashMap;
const MIN:f64 = 60.;

pub struct Tire {
    pub id: usize,
    pub radius: f64,// m
}

pub struct Chassis {
    pub f: Tire,
    pub bl: Tire,
    pub br: Tire,
}

// const OMNI_DIA:f64 =  0.1;
const CHASSIS: Chassis = Chassis {
    bl: Tire { id: 0, radius: 0.05 },
    br: Tire { id: 1, radius: 0.05 },
    f: Tire { id: 2,  radius: 0.05 },
};

pub struct OmniSetting {
    pub chassis: Chassis,
    pub radius: f64, // m
}

impl OmniSetting {
    fn ms_rpm(&self,radius:f64,ms:f64) -> f64 {
        (ms/radius)*MIN
    }

    pub fn move_chassis(
        &self,
        linear_x: f64,
        linear_y: f64,
        angular_z: f64,
    ) -> HashMap<usize, f64> {
        // 回転成分はあと
        let control_matrixn = arr2(&[
            [1., 0.,self.radius],
            [-1. / 2., 3_f64.sqrt() / 2.,self.radius],
            [-1. / 2., -1. * 3_f64.sqrt() / 2.,self.radius],
        ]);

        let vx_vy_az = arr1(&[linear_x, linear_y,angular_z]);

        // m/s
        let v1_v2_v3 = control_matrixn.dot(&vx_vy_az);

        let mut motor_power = HashMap::new();
        motor_power.insert(self.chassis.f.id, self.ms_rpm(self.chassis.f.radius, v1_v2_v3[[0]]));
        motor_power.insert(self.chassis.bl.id, self.ms_rpm(self.chassis.bl.radius, v1_v2_v3[[1]]));
        motor_power.insert(self.chassis.br.id, self.ms_rpm(self.chassis.f.radius, v1_v2_v3[[2]]));

        motor_power

        // Ratio adjustment
    }
}

fn main() -> Result<(), DynError> {
    let omni_setting = OmniSetting {
        chassis: CHASSIS,
        radius: 0.5,
    };

    // for debug

    let handle = GrpcHandle::new("http://127.0.0.1:50051");
    let ctx = Context::new()?;
    let node = ctx.create_node("robot_2", None, Default::default())?;
    let subscriber = node.create_subscriber::<msg::Twist>("cmd_vel", None)?;
    let mut selector = ctx.create_selector()?;

    selector.add_subscriber(subscriber, {
        Box::new(move |msg| {
            let _logger = Logger::new("robot_2");

            let motor_power =
                omni_setting.move_chassis(-msg.linear.x, -msg.linear.y, -msg.angular.z);
            pr_info!(
                _logger,
                "{:?}",
                &[msg.linear.x, msg.linear.y, msg.angular.z]
            );

            pr_info!(_logger, "{:?}", &motor_power);

            for i in 0..=2 as usize {
                md::send_speed(&handle, i as u8, motor_power[&i] as i16);
                // md::send_pwm(&handle, i as u8, motor_power[&i] as i16);
            }
        })
    });

    loop {
        selector.wait()?;
    }
}
