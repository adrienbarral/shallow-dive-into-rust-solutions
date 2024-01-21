use std::string::ParseError;

trait SerialPort {
    fn send_and_receive(&mut self, msg: &str) -> String;
}

#[derive(PartialEq, Debug)]
enum Command {
    WHO_ARE_YOU,
    SET_SPEED(f32),
    POSITION
}

struct DCController {
    name: String,
    actual_speed_ms: f32,
    actual_position_m: f32
}

impl SerialPort for DCController {
    /*
    Communication protocol of this controller is as follow : 
    ? will return name of the controller "DC"
    SPEED,<float> will start moving the motor at the desired speed in m/s as a float stringified
    POS? will return the actual position of the motor as a float stringified
     */
    fn send_and_receive(&mut self, msg: &str) -> String {

    }
}

fn parse(msg: &str) -> Result<Command, ParseError> {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn we_can_parse_command(){
        assert_eq!(parse("?"), Ok(Command::WHO_ARE_YOU));
        let speed_set = parse("SET_SPEED,12.2").unwrap();
        match speed_set {
            Command::SET_SPEED(speed) => assert_eq!((speed-12.2).abs() < 0.001, true),
            _ => assert!(false, "Parse error"),
        };
        assert_eq!(parse("?"), Ok(Command::WHO_ARE_YOU));
        
    }
}