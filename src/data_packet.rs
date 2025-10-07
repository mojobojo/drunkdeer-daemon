#[derive(Debug)]
pub struct DataPacket {
    pub report_id: u8,
    pub command: u8,
    pub data: [u8; 62],
}

impl DataPacket {
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        let mut data = [0u8; 62];

        assert_eq!(bytes[0], 4); 

        data.copy_from_slice(&bytes[2..]);
        Self {
            report_id: bytes[0],
            command: bytes[1],
            data,
        }
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        let mut data = [0u8; 64];

        assert_eq!(self.report_id, 4); 

        data[0] = self.report_id;
        data[1] = self.command;
        data[2..].copy_from_slice(&self.data);

        data
    }

    pub fn heartbeat() -> Self {
        let mut packet = Self {
            report_id: 4, 
            command: 160,
            data: [0u8; 62],
        };
        packet.data[0] = 2;

        packet
    }

    pub fn common(turbo: bool, rapid_trigger: bool, dual_trigger: bool, last_win: bool) -> Self {
        let mut packet = Self {
            report_id: 4, 
            command: 181,
            data: [0u8; 62],
        };

        packet.data[1] = 0x1E;
        packet.data[2] = 0x01;
        packet.data[5] = 0x01;

        if turbo {
            packet.data[6] = 1;
        }

        if rapid_trigger {
            packet.data[7] = 1;
        }

        if dual_trigger && last_win {
            packet.data[9] = 3;
        } else if dual_trigger {
            packet.data[9] = 2;
        } else if last_win {
            packet.data[9] = 1;
        }

        packet.data[10] = 0; // NOTE: this is whats known as the RTMatch value. I am not sure where it comes from yet.

        packet
    }

    pub fn led_mode(b: u8, mode: u8, speed: u8, brightness: u8, color: u8) -> Self {
        let mut packet = Self {
            report_id: 4, 
            command: 174,
            data: [0u8; 62],
        };

        packet.data[0] = 1;
        packet.data[1] = 0;
        packet.data[2] = b;
        packet.data[3] = mode;
        packet.data[4] = speed;
        packet.data[5] = brightness;
        packet.data[6] = color;

        packet
    }
}
