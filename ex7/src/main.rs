mod emulator;

use emulator::SerialPort;

trait MotorController {
    fn connect(&self, serial_port: &SerialPort);
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn can_create_cartesian_point(){
        
    }
}