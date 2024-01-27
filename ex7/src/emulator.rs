use rand::Rng;
use thiserror::Error;

pub fn init() -> Box<dyn SerialPort> {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.5) {
        Box::new(ACController::new())
    } else {
        Box::new(DCController::new())
    }
}
pub trait SerialPort {
    fn send_and_receive(&mut self, msg: &str) -> String;
}

#[derive(PartialEq, Debug)]
enum Command {
    WHO_ARE_YOU,
    SET_SPEED(f32),
    POSITION,
}

#[derive(Error, Debug)]
pub enum MessageParsingError {
    #[error("Unknow command")]
    UnknowCommand,
}

struct DCController {
    name: String,
    actual_speed_ms: f32,
    actual_position_m: f32,
    speed_setpoint_ms: f32,
}
impl DCController {
    pub fn new() -> Self {
        DCController {
            name: "DC".to_string(),
            actual_speed_ms: 0.,
            actual_position_m: 0.,
            speed_setpoint_ms: 0.,
        }
    }
}
impl SerialPort for DCController {
    /*
    Communication protocol of this controller is as follow :
    ? will return name of the controller "DC"
    SPEED,<float> will start moving the motor at the desired speed in m/s as a float stringified
    POS? will return the actual position of the motor as a float stringified
     */
    fn send_and_receive(&mut self, msg: &str) -> String {
        match parse_dc(msg) {
            Ok(Command::WHO_ARE_YOU) => self.name.clone(),
            Ok(Command::POSITION) => format!("{}", self.actual_position_m),
            Ok(Command::SET_SPEED(speed_setpoint)) => {
                self.speed_setpoint_ms = speed_setpoint;
                "OK".to_string()
            }
            _ => "PARSE ERROR".to_string(),
        }
    }
}

fn parse_dc(msg: &str) -> anyhow::Result<Command> {
    if msg == "?" {
        return Ok(Command::WHO_ARE_YOU);
    } else if msg.starts_with("SET_SPEED,") {
        let set_point = msg[10..].parse::<f32>()?;
        return Ok(Command::SET_SPEED(set_point));
    } else if msg.starts_with("POS?") {
        return Ok(Command::POSITION);
    }
    return Err(MessageParsingError::UnknowCommand.into());
}

fn parse_ac(msg: &str) -> anyhow::Result<Command> {
    if msg == "?" {
        return Ok(Command::WHO_ARE_YOU);
    } else if msg.starts_with("SET_POINT_SPEED,") {
        let set_point = msg[16..].parse::<f32>()?;
        return Ok(Command::SET_SPEED(set_point));
    } else if msg.starts_with("POSITION?") {
        return Ok(Command::POSITION);
    }
    return Err(MessageParsingError::UnknowCommand.into());
}
struct ACController {
    name: String,
    actual_speed_ms: f32,
    actual_position_m: f32,
    speed_setpoint_ms: f32,
}

impl ACController {
    pub fn new() -> Self {
        ACController {
            name: "AC".to_string(),
            actual_speed_ms: 0.,
            actual_position_m: 0.,
            speed_setpoint_ms: 0.,
        }
    }
}
impl SerialPort for ACController {
    /*
    Communication protocol of this controller is as follow :
    ? will return name of the controller "AC"
    SET_POINT_SPEED,<float> will start moving the motor at the desired speed in m/s
    as a float stringified. Will return OK.
    POSITON? will return the actual position of the motor as a float stringified
     */
    fn send_and_receive(&mut self, msg: &str) -> String {
        match parse_ac(msg) {
            Ok(Command::WHO_ARE_YOU) => self.name.clone(),
            Ok(Command::POSITION) => format!("{}", self.actual_position_m),
            Ok(Command::SET_SPEED(speed_setpoint)) => {
                self.speed_setpoint_ms = speed_setpoint;
                "OK".to_string()
            }
            _ => "PARSE ERROR".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn we_can_parse_command() {
        assert_eq!(parse_dc("?").unwrap(), Command::WHO_ARE_YOU);
        let speed_set = parse_dc("SET_SPEED,12.2").unwrap();
        match speed_set {
            Command::SET_SPEED(speed) => assert_eq!((speed - 12.2).abs() < 0.001, true),
            _ => assert!(false, "Parse error"),
        };
        assert_eq!(parse_dc("POS?").unwrap(), Command::POSITION);
    }

    #[test]
    pub fn we_can_parse_and_send_command_for_ac_controller() {
        let mut controller = ACController::new();
        assert_eq!(controller.send_and_receive("?"), "AC");
        assert_eq!(controller.send_and_receive("SET_POINT_SPEED,2.56"), "OK");
        assert_eq!((controller.speed_setpoint_ms - 2.56).abs() < 0.0001, true);
        assert_eq!(controller.send_and_receive("POSITION?"), "0");
    }
}
