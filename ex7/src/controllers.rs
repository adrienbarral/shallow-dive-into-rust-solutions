use std::{num::ParseFloatError, thread, time::Duration};

use emulator::SerialPort;

use crate::emulator;

struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
}

trait MotorController {
    fn set_speed(&mut self, speed_ms: f32);
    fn get_position(&mut self) -> Result<f32, ParseFloatError>;
    fn get_pid(&self) -> &PID;
}

struct ACMotorController<'a> {
    serial_port: &'a mut dyn SerialPort,
    pid: PID,
}

impl<'a> MotorController for ACMotorController<'a> {
    fn set_speed(&mut self, speed_ms: f32) {
        self.serial_port
            .send_and_receive(format!("SET_POINT_SPEED,{speed_ms}").as_str());
    }
    fn get_position(&mut self) -> Result<f32, ParseFloatError> {
        let pos_str = self.serial_port.send_and_receive("POSITION?");
        pos_str.parse::<f32>()
    }
    fn get_pid(&self) -> &PID {
        &self.pid
    }
}

struct DCMotorController<'a> {
    serial_port: &'a mut dyn SerialPort,
    pid: PID,
}

impl<'a> MotorController for DCMotorController<'a> {
    fn set_speed(&mut self, speed_ms: f32) {
        self.serial_port
            .send_and_receive(format!("SET_SPEED,{speed_ms}").as_str());
    }
    fn get_position(&mut self) -> Result<f32, ParseFloatError> {
        let pos_str = self.serial_port.send_and_receive("POS?");
        pos_str.parse::<f32>()
    }
    fn get_pid(&self) -> &PID {
        &self.pid
    }
}

pub fn controller_main(mut serial_port: Box<dyn SerialPort>) {
    let controller_type = serial_port.send_and_receive("?");
    let mut controller: Box<dyn MotorController> = match controller_type.as_str() {
        r"AC" => Box::new(ACMotorController {
            serial_port: serial_port.as_mut(),
            pid: PID {
                kp: 1.0,
                ki: 0.0,
                kd: 0.0,
            },
        }),
        r"DC" => Box::new(DCMotorController {
            serial_port: serial_port.as_mut(),
            pid: PID {
                kp: 1.0,
                ki: 0.0,
                kd: 0.0,
            },
        }),
        _ => panic!("Unknow controller type"),
    };
    let command = 10.0_f32;

    loop {
        let error = command - controller.get_position().unwrap();
        let pid = controller.get_pid();
        controller.set_speed(pid.kp * error + pid.ki * error + pid.kd * error);
        thread::sleep(Duration::from_millis(10));
    }
}
