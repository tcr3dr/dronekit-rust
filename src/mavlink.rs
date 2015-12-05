use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

pub trait Parsable {
    fn parse(payload: &[u8]) -> Self;
    fn serialize(&self) -> Vec<u8>;
}

#[derive(Clone, Debug)]
pub struct HEARTBEAT_DATA {
    pub custom_mode: u32,
    pub mavtype: u8,
    pub autopilot: u8,
    pub base_mode: u8,
    pub system_status: u8,
    pub mavlink_version: u8,
}

impl Parsable for HEARTBEAT_DATA {
    fn parse(payload: &[u8]) -> HEARTBEAT_DATA {
        let mut cur = Cursor::new(payload);
        HEARTBEAT_DATA {
            custom_mode: cur.read_u32::<LittleEndian>().unwrap(),
            mavtype: cur.read_u8().unwrap(),
            autopilot: cur.read_u8().unwrap(),
            base_mode: cur.read_u8().unwrap(),
            system_status: cur.read_u8().unwrap(),
            mavlink_version: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.custom_mode).unwrap();
        wtr.write_u8(self.mavtype).unwrap();
        wtr.write_u8(self.autopilot).unwrap();
        wtr.write_u8(self.base_mode).unwrap();
        wtr.write_u8(self.system_status).unwrap();
        wtr.write_u8(self.mavlink_version).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SYS_STATUS_DATA {
    pub onboard_control_sensors_present: u32,
    pub onboard_control_sensors_enabled: u32,
    pub onboard_control_sensors_health: u32,
    pub load: u16,
    pub voltage_battery: u16,
    pub current_battery: i16,
    pub drop_rate_comm: u16,
    pub errors_comm: u16,
    pub errors_count1: u16,
    pub errors_count2: u16,
    pub errors_count3: u16,
    pub errors_count4: u16,
    pub battery_remaining: i8,
}

impl Parsable for SYS_STATUS_DATA {
    fn parse(payload: &[u8]) -> SYS_STATUS_DATA {
        let mut cur = Cursor::new(payload);
        SYS_STATUS_DATA {
            onboard_control_sensors_present: cur.read_u32::<LittleEndian>().unwrap(),
            onboard_control_sensors_enabled: cur.read_u32::<LittleEndian>().unwrap(),
            onboard_control_sensors_health: cur.read_u32::<LittleEndian>().unwrap(),
            load: cur.read_u16::<LittleEndian>().unwrap(),
            voltage_battery: cur.read_u16::<LittleEndian>().unwrap(),
            current_battery: cur.read_i16::<LittleEndian>().unwrap(),
            drop_rate_comm: cur.read_u16::<LittleEndian>().unwrap(),
            errors_comm: cur.read_u16::<LittleEndian>().unwrap(),
            errors_count1: cur.read_u16::<LittleEndian>().unwrap(),
            errors_count2: cur.read_u16::<LittleEndian>().unwrap(),
            errors_count3: cur.read_u16::<LittleEndian>().unwrap(),
            errors_count4: cur.read_u16::<LittleEndian>().unwrap(),
            battery_remaining: cur.read_i8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.onboard_control_sensors_present).unwrap();
        wtr.write_u32::<LittleEndian>(self.onboard_control_sensors_enabled).unwrap();
        wtr.write_u32::<LittleEndian>(self.onboard_control_sensors_health).unwrap();
        wtr.write_u16::<LittleEndian>(self.load).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltage_battery).unwrap();
        wtr.write_i16::<LittleEndian>(self.current_battery).unwrap();
        wtr.write_u16::<LittleEndian>(self.drop_rate_comm).unwrap();
        wtr.write_u16::<LittleEndian>(self.errors_comm).unwrap();
        wtr.write_u16::<LittleEndian>(self.errors_count1).unwrap();
        wtr.write_u16::<LittleEndian>(self.errors_count2).unwrap();
        wtr.write_u16::<LittleEndian>(self.errors_count3).unwrap();
        wtr.write_u16::<LittleEndian>(self.errors_count4).unwrap();
        wtr.write_i8(self.battery_remaining).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SYSTEM_TIME_DATA {
    pub time_unix_usec: u64,
    pub time_boot_ms: u32,
}

impl Parsable for SYSTEM_TIME_DATA {
    fn parse(payload: &[u8]) -> SYSTEM_TIME_DATA {
        let mut cur = Cursor::new(payload);
        SYSTEM_TIME_DATA {
            time_unix_usec: cur.read_u64::<LittleEndian>().unwrap(),
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_unix_usec).unwrap();
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct PING_DATA {
    pub time_usec: u64,
    pub seq: u32,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for PING_DATA {
    fn parse(payload: &[u8]) -> PING_DATA {
        let mut cur = Cursor::new(payload);
        PING_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            seq: cur.read_u32::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_u32::<LittleEndian>(self.seq).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct CHANGE_OPERATOR_CONTROL_DATA {
    pub target_system: u8,
    pub control_request: u8,
    pub version: u8,
    pub passkey: Vec<u8> /* 25 */,
}

impl Parsable for CHANGE_OPERATOR_CONTROL_DATA {
    fn parse(payload: &[u8]) -> CHANGE_OPERATOR_CONTROL_DATA {
        let mut cur = Cursor::new(payload);
        CHANGE_OPERATOR_CONTROL_DATA {
            target_system: cur.read_u8().unwrap(),
            control_request: cur.read_u8().unwrap(),
            version: cur.read_u8().unwrap(),
            passkey: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.control_request).unwrap();
        wtr.write_u8(self.version).unwrap();
        wtr.write_u8(self.passkey[0]).unwrap();
        wtr.write_u8(self.passkey[1]).unwrap();
        wtr.write_u8(self.passkey[2]).unwrap();
        wtr.write_u8(self.passkey[3]).unwrap();
        wtr.write_u8(self.passkey[4]).unwrap();
        wtr.write_u8(self.passkey[5]).unwrap();
        wtr.write_u8(self.passkey[6]).unwrap();
        wtr.write_u8(self.passkey[7]).unwrap();
        wtr.write_u8(self.passkey[8]).unwrap();
        wtr.write_u8(self.passkey[9]).unwrap();
        wtr.write_u8(self.passkey[10]).unwrap();
        wtr.write_u8(self.passkey[11]).unwrap();
        wtr.write_u8(self.passkey[12]).unwrap();
        wtr.write_u8(self.passkey[13]).unwrap();
        wtr.write_u8(self.passkey[14]).unwrap();
        wtr.write_u8(self.passkey[15]).unwrap();
        wtr.write_u8(self.passkey[16]).unwrap();
        wtr.write_u8(self.passkey[17]).unwrap();
        wtr.write_u8(self.passkey[18]).unwrap();
        wtr.write_u8(self.passkey[19]).unwrap();
        wtr.write_u8(self.passkey[20]).unwrap();
        wtr.write_u8(self.passkey[21]).unwrap();
        wtr.write_u8(self.passkey[22]).unwrap();
        wtr.write_u8(self.passkey[23]).unwrap();
        wtr.write_u8(self.passkey[24]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct CHANGE_OPERATOR_CONTROL_ACK_DATA {
    pub gcs_system_id: u8,
    pub control_request: u8,
    pub ack: u8,
}

impl Parsable for CHANGE_OPERATOR_CONTROL_ACK_DATA {
    fn parse(payload: &[u8]) -> CHANGE_OPERATOR_CONTROL_ACK_DATA {
        let mut cur = Cursor::new(payload);
        CHANGE_OPERATOR_CONTROL_ACK_DATA {
            gcs_system_id: cur.read_u8().unwrap(),
            control_request: cur.read_u8().unwrap(),
            ack: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.gcs_system_id).unwrap();
        wtr.write_u8(self.control_request).unwrap();
        wtr.write_u8(self.ack).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct AUTH_KEY_DATA {
    pub key: Vec<u8> /* 32 */,
}

impl Parsable for AUTH_KEY_DATA {
    fn parse(payload: &[u8]) -> AUTH_KEY_DATA {
        let mut cur = Cursor::new(payload);
        AUTH_KEY_DATA {
            key: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.key[0]).unwrap();
        wtr.write_u8(self.key[1]).unwrap();
        wtr.write_u8(self.key[2]).unwrap();
        wtr.write_u8(self.key[3]).unwrap();
        wtr.write_u8(self.key[4]).unwrap();
        wtr.write_u8(self.key[5]).unwrap();
        wtr.write_u8(self.key[6]).unwrap();
        wtr.write_u8(self.key[7]).unwrap();
        wtr.write_u8(self.key[8]).unwrap();
        wtr.write_u8(self.key[9]).unwrap();
        wtr.write_u8(self.key[10]).unwrap();
        wtr.write_u8(self.key[11]).unwrap();
        wtr.write_u8(self.key[12]).unwrap();
        wtr.write_u8(self.key[13]).unwrap();
        wtr.write_u8(self.key[14]).unwrap();
        wtr.write_u8(self.key[15]).unwrap();
        wtr.write_u8(self.key[16]).unwrap();
        wtr.write_u8(self.key[17]).unwrap();
        wtr.write_u8(self.key[18]).unwrap();
        wtr.write_u8(self.key[19]).unwrap();
        wtr.write_u8(self.key[20]).unwrap();
        wtr.write_u8(self.key[21]).unwrap();
        wtr.write_u8(self.key[22]).unwrap();
        wtr.write_u8(self.key[23]).unwrap();
        wtr.write_u8(self.key[24]).unwrap();
        wtr.write_u8(self.key[25]).unwrap();
        wtr.write_u8(self.key[26]).unwrap();
        wtr.write_u8(self.key[27]).unwrap();
        wtr.write_u8(self.key[28]).unwrap();
        wtr.write_u8(self.key[29]).unwrap();
        wtr.write_u8(self.key[30]).unwrap();
        wtr.write_u8(self.key[31]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SET_MODE_DATA {
    pub custom_mode: u32,
    pub target_system: u8,
    pub base_mode: u8,
}

impl Parsable for SET_MODE_DATA {
    fn parse(payload: &[u8]) -> SET_MODE_DATA {
        let mut cur = Cursor::new(payload);
        SET_MODE_DATA {
            custom_mode: cur.read_u32::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            base_mode: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.custom_mode).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.base_mode).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct PARAM_REQUEST_READ_DATA {
    pub param_index: i16,
    pub target_system: u8,
    pub target_component: u8,
    pub param_id: Vec<u8> /* 16 */,
}

impl Parsable for PARAM_REQUEST_READ_DATA {
    fn parse(payload: &[u8]) -> PARAM_REQUEST_READ_DATA {
        let mut cur = Cursor::new(payload);
        PARAM_REQUEST_READ_DATA {
            param_index: cur.read_i16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            param_id: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i16::<LittleEndian>(self.param_index).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.param_id[0]).unwrap();
        wtr.write_u8(self.param_id[1]).unwrap();
        wtr.write_u8(self.param_id[2]).unwrap();
        wtr.write_u8(self.param_id[3]).unwrap();
        wtr.write_u8(self.param_id[4]).unwrap();
        wtr.write_u8(self.param_id[5]).unwrap();
        wtr.write_u8(self.param_id[6]).unwrap();
        wtr.write_u8(self.param_id[7]).unwrap();
        wtr.write_u8(self.param_id[8]).unwrap();
        wtr.write_u8(self.param_id[9]).unwrap();
        wtr.write_u8(self.param_id[10]).unwrap();
        wtr.write_u8(self.param_id[11]).unwrap();
        wtr.write_u8(self.param_id[12]).unwrap();
        wtr.write_u8(self.param_id[13]).unwrap();
        wtr.write_u8(self.param_id[14]).unwrap();
        wtr.write_u8(self.param_id[15]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct PARAM_REQUEST_LIST_DATA {
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for PARAM_REQUEST_LIST_DATA {
    fn parse(payload: &[u8]) -> PARAM_REQUEST_LIST_DATA {
        let mut cur = Cursor::new(payload);
        PARAM_REQUEST_LIST_DATA {
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct PARAM_VALUE_DATA {
    pub param_value: f32,
    pub param_count: u16,
    pub param_index: u16,
    pub param_id: Vec<u8> /* 16 */,
    pub param_type: u8,
}

impl Parsable for PARAM_VALUE_DATA {
    fn parse(payload: &[u8]) -> PARAM_VALUE_DATA {
        let mut cur = Cursor::new(payload);
        PARAM_VALUE_DATA {
            param_value: cur.read_f32::<LittleEndian>().unwrap(),
            param_count: cur.read_u16::<LittleEndian>().unwrap(),
            param_index: cur.read_u16::<LittleEndian>().unwrap(),
            param_id: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            param_type: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.param_value).unwrap();
        wtr.write_u16::<LittleEndian>(self.param_count).unwrap();
        wtr.write_u16::<LittleEndian>(self.param_index).unwrap();
        wtr.write_u8(self.param_id[0]).unwrap();
        wtr.write_u8(self.param_id[1]).unwrap();
        wtr.write_u8(self.param_id[2]).unwrap();
        wtr.write_u8(self.param_id[3]).unwrap();
        wtr.write_u8(self.param_id[4]).unwrap();
        wtr.write_u8(self.param_id[5]).unwrap();
        wtr.write_u8(self.param_id[6]).unwrap();
        wtr.write_u8(self.param_id[7]).unwrap();
        wtr.write_u8(self.param_id[8]).unwrap();
        wtr.write_u8(self.param_id[9]).unwrap();
        wtr.write_u8(self.param_id[10]).unwrap();
        wtr.write_u8(self.param_id[11]).unwrap();
        wtr.write_u8(self.param_id[12]).unwrap();
        wtr.write_u8(self.param_id[13]).unwrap();
        wtr.write_u8(self.param_id[14]).unwrap();
        wtr.write_u8(self.param_id[15]).unwrap();
        wtr.write_u8(self.param_type).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct PARAM_SET_DATA {
    pub param_value: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub param_id: Vec<u8> /* 16 */,
    pub param_type: u8,
}

impl Parsable for PARAM_SET_DATA {
    fn parse(payload: &[u8]) -> PARAM_SET_DATA {
        let mut cur = Cursor::new(payload);
        PARAM_SET_DATA {
            param_value: cur.read_f32::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            param_id: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            param_type: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.param_value).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.param_id[0]).unwrap();
        wtr.write_u8(self.param_id[1]).unwrap();
        wtr.write_u8(self.param_id[2]).unwrap();
        wtr.write_u8(self.param_id[3]).unwrap();
        wtr.write_u8(self.param_id[4]).unwrap();
        wtr.write_u8(self.param_id[5]).unwrap();
        wtr.write_u8(self.param_id[6]).unwrap();
        wtr.write_u8(self.param_id[7]).unwrap();
        wtr.write_u8(self.param_id[8]).unwrap();
        wtr.write_u8(self.param_id[9]).unwrap();
        wtr.write_u8(self.param_id[10]).unwrap();
        wtr.write_u8(self.param_id[11]).unwrap();
        wtr.write_u8(self.param_id[12]).unwrap();
        wtr.write_u8(self.param_id[13]).unwrap();
        wtr.write_u8(self.param_id[14]).unwrap();
        wtr.write_u8(self.param_id[15]).unwrap();
        wtr.write_u8(self.param_type).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GPS_RAW_INT_DATA {
    pub time_usec: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub eph: u16,
    pub epv: u16,
    pub vel: u16,
    pub cog: u16,
    pub fix_type: u8,
    pub satellites_visible: u8,
}

impl Parsable for GPS_RAW_INT_DATA {
    fn parse(payload: &[u8]) -> GPS_RAW_INT_DATA {
        let mut cur = Cursor::new(payload);
        GPS_RAW_INT_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_i32::<LittleEndian>().unwrap(),
            eph: cur.read_u16::<LittleEndian>().unwrap(),
            epv: cur.read_u16::<LittleEndian>().unwrap(),
            vel: cur.read_u16::<LittleEndian>().unwrap(),
            cog: cur.read_u16::<LittleEndian>().unwrap(),
            fix_type: cur.read_u8().unwrap(),
            satellites_visible: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_i32::<LittleEndian>(self.alt).unwrap();
        wtr.write_u16::<LittleEndian>(self.eph).unwrap();
        wtr.write_u16::<LittleEndian>(self.epv).unwrap();
        wtr.write_u16::<LittleEndian>(self.vel).unwrap();
        wtr.write_u16::<LittleEndian>(self.cog).unwrap();
        wtr.write_u8(self.fix_type).unwrap();
        wtr.write_u8(self.satellites_visible).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GPS_STATUS_DATA {
    pub satellites_visible: u8,
    pub satellite_prn: Vec<u8> /* 20 */,
    pub satellite_used: Vec<u8> /* 20 */,
    pub satellite_elevation: Vec<u8> /* 20 */,
    pub satellite_azimuth: Vec<u8> /* 20 */,
    pub satellite_snr: Vec<u8> /* 20 */,
}

impl Parsable for GPS_STATUS_DATA {
    fn parse(payload: &[u8]) -> GPS_STATUS_DATA {
        let mut cur = Cursor::new(payload);
        GPS_STATUS_DATA {
            satellites_visible: cur.read_u8().unwrap(),
            satellite_prn: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            satellite_used: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            satellite_elevation: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            satellite_azimuth: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            satellite_snr: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.satellites_visible).unwrap();
        wtr.write_u8(self.satellite_prn[0]).unwrap();
        wtr.write_u8(self.satellite_prn[1]).unwrap();
        wtr.write_u8(self.satellite_prn[2]).unwrap();
        wtr.write_u8(self.satellite_prn[3]).unwrap();
        wtr.write_u8(self.satellite_prn[4]).unwrap();
        wtr.write_u8(self.satellite_prn[5]).unwrap();
        wtr.write_u8(self.satellite_prn[6]).unwrap();
        wtr.write_u8(self.satellite_prn[7]).unwrap();
        wtr.write_u8(self.satellite_prn[8]).unwrap();
        wtr.write_u8(self.satellite_prn[9]).unwrap();
        wtr.write_u8(self.satellite_prn[10]).unwrap();
        wtr.write_u8(self.satellite_prn[11]).unwrap();
        wtr.write_u8(self.satellite_prn[12]).unwrap();
        wtr.write_u8(self.satellite_prn[13]).unwrap();
        wtr.write_u8(self.satellite_prn[14]).unwrap();
        wtr.write_u8(self.satellite_prn[15]).unwrap();
        wtr.write_u8(self.satellite_prn[16]).unwrap();
        wtr.write_u8(self.satellite_prn[17]).unwrap();
        wtr.write_u8(self.satellite_prn[18]).unwrap();
        wtr.write_u8(self.satellite_prn[19]).unwrap();
        wtr.write_u8(self.satellite_used[0]).unwrap();
        wtr.write_u8(self.satellite_used[1]).unwrap();
        wtr.write_u8(self.satellite_used[2]).unwrap();
        wtr.write_u8(self.satellite_used[3]).unwrap();
        wtr.write_u8(self.satellite_used[4]).unwrap();
        wtr.write_u8(self.satellite_used[5]).unwrap();
        wtr.write_u8(self.satellite_used[6]).unwrap();
        wtr.write_u8(self.satellite_used[7]).unwrap();
        wtr.write_u8(self.satellite_used[8]).unwrap();
        wtr.write_u8(self.satellite_used[9]).unwrap();
        wtr.write_u8(self.satellite_used[10]).unwrap();
        wtr.write_u8(self.satellite_used[11]).unwrap();
        wtr.write_u8(self.satellite_used[12]).unwrap();
        wtr.write_u8(self.satellite_used[13]).unwrap();
        wtr.write_u8(self.satellite_used[14]).unwrap();
        wtr.write_u8(self.satellite_used[15]).unwrap();
        wtr.write_u8(self.satellite_used[16]).unwrap();
        wtr.write_u8(self.satellite_used[17]).unwrap();
        wtr.write_u8(self.satellite_used[18]).unwrap();
        wtr.write_u8(self.satellite_used[19]).unwrap();
        wtr.write_u8(self.satellite_elevation[0]).unwrap();
        wtr.write_u8(self.satellite_elevation[1]).unwrap();
        wtr.write_u8(self.satellite_elevation[2]).unwrap();
        wtr.write_u8(self.satellite_elevation[3]).unwrap();
        wtr.write_u8(self.satellite_elevation[4]).unwrap();
        wtr.write_u8(self.satellite_elevation[5]).unwrap();
        wtr.write_u8(self.satellite_elevation[6]).unwrap();
        wtr.write_u8(self.satellite_elevation[7]).unwrap();
        wtr.write_u8(self.satellite_elevation[8]).unwrap();
        wtr.write_u8(self.satellite_elevation[9]).unwrap();
        wtr.write_u8(self.satellite_elevation[10]).unwrap();
        wtr.write_u8(self.satellite_elevation[11]).unwrap();
        wtr.write_u8(self.satellite_elevation[12]).unwrap();
        wtr.write_u8(self.satellite_elevation[13]).unwrap();
        wtr.write_u8(self.satellite_elevation[14]).unwrap();
        wtr.write_u8(self.satellite_elevation[15]).unwrap();
        wtr.write_u8(self.satellite_elevation[16]).unwrap();
        wtr.write_u8(self.satellite_elevation[17]).unwrap();
        wtr.write_u8(self.satellite_elevation[18]).unwrap();
        wtr.write_u8(self.satellite_elevation[19]).unwrap();
        wtr.write_u8(self.satellite_azimuth[0]).unwrap();
        wtr.write_u8(self.satellite_azimuth[1]).unwrap();
        wtr.write_u8(self.satellite_azimuth[2]).unwrap();
        wtr.write_u8(self.satellite_azimuth[3]).unwrap();
        wtr.write_u8(self.satellite_azimuth[4]).unwrap();
        wtr.write_u8(self.satellite_azimuth[5]).unwrap();
        wtr.write_u8(self.satellite_azimuth[6]).unwrap();
        wtr.write_u8(self.satellite_azimuth[7]).unwrap();
        wtr.write_u8(self.satellite_azimuth[8]).unwrap();
        wtr.write_u8(self.satellite_azimuth[9]).unwrap();
        wtr.write_u8(self.satellite_azimuth[10]).unwrap();
        wtr.write_u8(self.satellite_azimuth[11]).unwrap();
        wtr.write_u8(self.satellite_azimuth[12]).unwrap();
        wtr.write_u8(self.satellite_azimuth[13]).unwrap();
        wtr.write_u8(self.satellite_azimuth[14]).unwrap();
        wtr.write_u8(self.satellite_azimuth[15]).unwrap();
        wtr.write_u8(self.satellite_azimuth[16]).unwrap();
        wtr.write_u8(self.satellite_azimuth[17]).unwrap();
        wtr.write_u8(self.satellite_azimuth[18]).unwrap();
        wtr.write_u8(self.satellite_azimuth[19]).unwrap();
        wtr.write_u8(self.satellite_snr[0]).unwrap();
        wtr.write_u8(self.satellite_snr[1]).unwrap();
        wtr.write_u8(self.satellite_snr[2]).unwrap();
        wtr.write_u8(self.satellite_snr[3]).unwrap();
        wtr.write_u8(self.satellite_snr[4]).unwrap();
        wtr.write_u8(self.satellite_snr[5]).unwrap();
        wtr.write_u8(self.satellite_snr[6]).unwrap();
        wtr.write_u8(self.satellite_snr[7]).unwrap();
        wtr.write_u8(self.satellite_snr[8]).unwrap();
        wtr.write_u8(self.satellite_snr[9]).unwrap();
        wtr.write_u8(self.satellite_snr[10]).unwrap();
        wtr.write_u8(self.satellite_snr[11]).unwrap();
        wtr.write_u8(self.satellite_snr[12]).unwrap();
        wtr.write_u8(self.satellite_snr[13]).unwrap();
        wtr.write_u8(self.satellite_snr[14]).unwrap();
        wtr.write_u8(self.satellite_snr[15]).unwrap();
        wtr.write_u8(self.satellite_snr[16]).unwrap();
        wtr.write_u8(self.satellite_snr[17]).unwrap();
        wtr.write_u8(self.satellite_snr[18]).unwrap();
        wtr.write_u8(self.satellite_snr[19]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SCALED_IMU_DATA {
    pub time_boot_ms: u32,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
    pub xgyro: i16,
    pub ygyro: i16,
    pub zgyro: i16,
    pub xmag: i16,
    pub ymag: i16,
    pub zmag: i16,
}

impl Parsable for SCALED_IMU_DATA {
    fn parse(payload: &[u8]) -> SCALED_IMU_DATA {
        let mut cur = Cursor::new(payload);
        SCALED_IMU_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            xacc: cur.read_i16::<LittleEndian>().unwrap(),
            yacc: cur.read_i16::<LittleEndian>().unwrap(),
            zacc: cur.read_i16::<LittleEndian>().unwrap(),
            xgyro: cur.read_i16::<LittleEndian>().unwrap(),
            ygyro: cur.read_i16::<LittleEndian>().unwrap(),
            zgyro: cur.read_i16::<LittleEndian>().unwrap(),
            xmag: cur.read_i16::<LittleEndian>().unwrap(),
            ymag: cur.read_i16::<LittleEndian>().unwrap(),
            zmag: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i16::<LittleEndian>(self.xacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.yacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.zacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.xgyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.ygyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.zgyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.xmag).unwrap();
        wtr.write_i16::<LittleEndian>(self.ymag).unwrap();
        wtr.write_i16::<LittleEndian>(self.zmag).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct RAW_IMU_DATA {
    pub time_usec: u64,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
    pub xgyro: i16,
    pub ygyro: i16,
    pub zgyro: i16,
    pub xmag: i16,
    pub ymag: i16,
    pub zmag: i16,
}

impl Parsable for RAW_IMU_DATA {
    fn parse(payload: &[u8]) -> RAW_IMU_DATA {
        let mut cur = Cursor::new(payload);
        RAW_IMU_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            xacc: cur.read_i16::<LittleEndian>().unwrap(),
            yacc: cur.read_i16::<LittleEndian>().unwrap(),
            zacc: cur.read_i16::<LittleEndian>().unwrap(),
            xgyro: cur.read_i16::<LittleEndian>().unwrap(),
            ygyro: cur.read_i16::<LittleEndian>().unwrap(),
            zgyro: cur.read_i16::<LittleEndian>().unwrap(),
            xmag: cur.read_i16::<LittleEndian>().unwrap(),
            ymag: cur.read_i16::<LittleEndian>().unwrap(),
            zmag: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_i16::<LittleEndian>(self.xacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.yacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.zacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.xgyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.ygyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.zgyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.xmag).unwrap();
        wtr.write_i16::<LittleEndian>(self.ymag).unwrap();
        wtr.write_i16::<LittleEndian>(self.zmag).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct RAW_PRESSURE_DATA {
    pub time_usec: u64,
    pub press_abs: i16,
    pub press_diff1: i16,
    pub press_diff2: i16,
    pub temperature: i16,
}

impl Parsable for RAW_PRESSURE_DATA {
    fn parse(payload: &[u8]) -> RAW_PRESSURE_DATA {
        let mut cur = Cursor::new(payload);
        RAW_PRESSURE_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            press_abs: cur.read_i16::<LittleEndian>().unwrap(),
            press_diff1: cur.read_i16::<LittleEndian>().unwrap(),
            press_diff2: cur.read_i16::<LittleEndian>().unwrap(),
            temperature: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_i16::<LittleEndian>(self.press_abs).unwrap();
        wtr.write_i16::<LittleEndian>(self.press_diff1).unwrap();
        wtr.write_i16::<LittleEndian>(self.press_diff2).unwrap();
        wtr.write_i16::<LittleEndian>(self.temperature).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SCALED_PRESSURE_DATA {
    pub time_boot_ms: u32,
    pub press_abs: f32,
    pub press_diff: f32,
    pub temperature: i16,
}

impl Parsable for SCALED_PRESSURE_DATA {
    fn parse(payload: &[u8]) -> SCALED_PRESSURE_DATA {
        let mut cur = Cursor::new(payload);
        SCALED_PRESSURE_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            press_abs: cur.read_f32::<LittleEndian>().unwrap(),
            press_diff: cur.read_f32::<LittleEndian>().unwrap(),
            temperature: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.press_abs).unwrap();
        wtr.write_f32::<LittleEndian>(self.press_diff).unwrap();
        wtr.write_i16::<LittleEndian>(self.temperature).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ATTITUDE_DATA {
    pub time_boot_ms: u32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
}

impl Parsable for ATTITUDE_DATA {
    fn parse(payload: &[u8]) -> ATTITUDE_DATA {
        let mut cur = Cursor::new(payload);
        ATTITUDE_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            roll: cur.read_f32::<LittleEndian>().unwrap(),
            pitch: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
            rollspeed: cur.read_f32::<LittleEndian>().unwrap(),
            pitchspeed: cur.read_f32::<LittleEndian>().unwrap(),
            yawspeed: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr.write_f32::<LittleEndian>(self.rollspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitchspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.yawspeed).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ATTITUDE_QUATERNION_DATA {
    pub time_boot_ms: u32,
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub q4: f32,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
}

impl Parsable for ATTITUDE_QUATERNION_DATA {
    fn parse(payload: &[u8]) -> ATTITUDE_QUATERNION_DATA {
        let mut cur = Cursor::new(payload);
        ATTITUDE_QUATERNION_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            q1: cur.read_f32::<LittleEndian>().unwrap(),
            q2: cur.read_f32::<LittleEndian>().unwrap(),
            q3: cur.read_f32::<LittleEndian>().unwrap(),
            q4: cur.read_f32::<LittleEndian>().unwrap(),
            rollspeed: cur.read_f32::<LittleEndian>().unwrap(),
            pitchspeed: cur.read_f32::<LittleEndian>().unwrap(),
            yawspeed: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.q1).unwrap();
        wtr.write_f32::<LittleEndian>(self.q2).unwrap();
        wtr.write_f32::<LittleEndian>(self.q3).unwrap();
        wtr.write_f32::<LittleEndian>(self.q4).unwrap();
        wtr.write_f32::<LittleEndian>(self.rollspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitchspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.yawspeed).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOCAL_POSITION_NED_DATA {
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

impl Parsable for LOCAL_POSITION_NED_DATA {
    fn parse(payload: &[u8]) -> LOCAL_POSITION_NED_DATA {
        let mut cur = Cursor::new(payload);
        LOCAL_POSITION_NED_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            vx: cur.read_f32::<LittleEndian>().unwrap(),
            vy: cur.read_f32::<LittleEndian>().unwrap(),
            vz: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.vx).unwrap();
        wtr.write_f32::<LittleEndian>(self.vy).unwrap();
        wtr.write_f32::<LittleEndian>(self.vz).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GLOBAL_POSITION_INT_DATA {
    pub time_boot_ms: u32,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub relative_alt: i32,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub hdg: u16,
}

impl Parsable for GLOBAL_POSITION_INT_DATA {
    fn parse(payload: &[u8]) -> GLOBAL_POSITION_INT_DATA {
        let mut cur = Cursor::new(payload);
        GLOBAL_POSITION_INT_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_i32::<LittleEndian>().unwrap(),
            relative_alt: cur.read_i32::<LittleEndian>().unwrap(),
            vx: cur.read_i16::<LittleEndian>().unwrap(),
            vy: cur.read_i16::<LittleEndian>().unwrap(),
            vz: cur.read_i16::<LittleEndian>().unwrap(),
            hdg: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_i32::<LittleEndian>(self.alt).unwrap();
        wtr.write_i32::<LittleEndian>(self.relative_alt).unwrap();
        wtr.write_i16::<LittleEndian>(self.vx).unwrap();
        wtr.write_i16::<LittleEndian>(self.vy).unwrap();
        wtr.write_i16::<LittleEndian>(self.vz).unwrap();
        wtr.write_u16::<LittleEndian>(self.hdg).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct RC_CHANNELS_SCALED_DATA {
    pub time_boot_ms: u32,
    pub chan1_scaled: i16,
    pub chan2_scaled: i16,
    pub chan3_scaled: i16,
    pub chan4_scaled: i16,
    pub chan5_scaled: i16,
    pub chan6_scaled: i16,
    pub chan7_scaled: i16,
    pub chan8_scaled: i16,
    pub port: u8,
    pub rssi: u8,
}

impl Parsable for RC_CHANNELS_SCALED_DATA {
    fn parse(payload: &[u8]) -> RC_CHANNELS_SCALED_DATA {
        let mut cur = Cursor::new(payload);
        RC_CHANNELS_SCALED_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            chan1_scaled: cur.read_i16::<LittleEndian>().unwrap(),
            chan2_scaled: cur.read_i16::<LittleEndian>().unwrap(),
            chan3_scaled: cur.read_i16::<LittleEndian>().unwrap(),
            chan4_scaled: cur.read_i16::<LittleEndian>().unwrap(),
            chan5_scaled: cur.read_i16::<LittleEndian>().unwrap(),
            chan6_scaled: cur.read_i16::<LittleEndian>().unwrap(),
            chan7_scaled: cur.read_i16::<LittleEndian>().unwrap(),
            chan8_scaled: cur.read_i16::<LittleEndian>().unwrap(),
            port: cur.read_u8().unwrap(),
            rssi: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i16::<LittleEndian>(self.chan1_scaled).unwrap();
        wtr.write_i16::<LittleEndian>(self.chan2_scaled).unwrap();
        wtr.write_i16::<LittleEndian>(self.chan3_scaled).unwrap();
        wtr.write_i16::<LittleEndian>(self.chan4_scaled).unwrap();
        wtr.write_i16::<LittleEndian>(self.chan5_scaled).unwrap();
        wtr.write_i16::<LittleEndian>(self.chan6_scaled).unwrap();
        wtr.write_i16::<LittleEndian>(self.chan7_scaled).unwrap();
        wtr.write_i16::<LittleEndian>(self.chan8_scaled).unwrap();
        wtr.write_u8(self.port).unwrap();
        wtr.write_u8(self.rssi).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct RC_CHANNELS_RAW_DATA {
    pub time_boot_ms: u32,
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub port: u8,
    pub rssi: u8,
}

impl Parsable for RC_CHANNELS_RAW_DATA {
    fn parse(payload: &[u8]) -> RC_CHANNELS_RAW_DATA {
        let mut cur = Cursor::new(payload);
        RC_CHANNELS_RAW_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            chan1_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan2_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan3_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan4_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan5_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan6_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan7_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan8_raw: cur.read_u16::<LittleEndian>().unwrap(),
            port: cur.read_u8().unwrap(),
            rssi: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan1_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan2_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan3_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan4_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan5_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan6_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan7_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan8_raw).unwrap();
        wtr.write_u8(self.port).unwrap();
        wtr.write_u8(self.rssi).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SERVO_OUTPUT_RAW_DATA {
    pub time_usec: u32,
    pub servo1_raw: u16,
    pub servo2_raw: u16,
    pub servo3_raw: u16,
    pub servo4_raw: u16,
    pub servo5_raw: u16,
    pub servo6_raw: u16,
    pub servo7_raw: u16,
    pub servo8_raw: u16,
    pub port: u8,
}

impl Parsable for SERVO_OUTPUT_RAW_DATA {
    fn parse(payload: &[u8]) -> SERVO_OUTPUT_RAW_DATA {
        let mut cur = Cursor::new(payload);
        SERVO_OUTPUT_RAW_DATA {
            time_usec: cur.read_u32::<LittleEndian>().unwrap(),
            servo1_raw: cur.read_u16::<LittleEndian>().unwrap(),
            servo2_raw: cur.read_u16::<LittleEndian>().unwrap(),
            servo3_raw: cur.read_u16::<LittleEndian>().unwrap(),
            servo4_raw: cur.read_u16::<LittleEndian>().unwrap(),
            servo5_raw: cur.read_u16::<LittleEndian>().unwrap(),
            servo6_raw: cur.read_u16::<LittleEndian>().unwrap(),
            servo7_raw: cur.read_u16::<LittleEndian>().unwrap(),
            servo8_raw: cur.read_u16::<LittleEndian>().unwrap(),
            port: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_u16::<LittleEndian>(self.servo1_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.servo2_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.servo3_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.servo4_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.servo5_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.servo6_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.servo7_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.servo8_raw).unwrap();
        wtr.write_u8(self.port).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_REQUEST_PARTIAL_LIST_DATA {
    pub start_index: i16,
    pub end_index: i16,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for MISSION_REQUEST_PARTIAL_LIST_DATA {
    fn parse(payload: &[u8]) -> MISSION_REQUEST_PARTIAL_LIST_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_REQUEST_PARTIAL_LIST_DATA {
            start_index: cur.read_i16::<LittleEndian>().unwrap(),
            end_index: cur.read_i16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i16::<LittleEndian>(self.start_index).unwrap();
        wtr.write_i16::<LittleEndian>(self.end_index).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_WRITE_PARTIAL_LIST_DATA {
    pub start_index: i16,
    pub end_index: i16,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for MISSION_WRITE_PARTIAL_LIST_DATA {
    fn parse(payload: &[u8]) -> MISSION_WRITE_PARTIAL_LIST_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_WRITE_PARTIAL_LIST_DATA {
            start_index: cur.read_i16::<LittleEndian>().unwrap(),
            end_index: cur.read_i16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i16::<LittleEndian>(self.start_index).unwrap();
        wtr.write_i16::<LittleEndian>(self.end_index).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_ITEM_DATA {
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub seq: u16,
    pub command: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub frame: u8,
    pub current: u8,
    pub autocontinue: u8,
}

impl Parsable for MISSION_ITEM_DATA {
    fn parse(payload: &[u8]) -> MISSION_ITEM_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_ITEM_DATA {
            param1: cur.read_f32::<LittleEndian>().unwrap(),
            param2: cur.read_f32::<LittleEndian>().unwrap(),
            param3: cur.read_f32::<LittleEndian>().unwrap(),
            param4: cur.read_f32::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            seq: cur.read_u16::<LittleEndian>().unwrap(),
            command: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            frame: cur.read_u8().unwrap(),
            current: cur.read_u8().unwrap(),
            autocontinue: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.param1).unwrap();
        wtr.write_f32::<LittleEndian>(self.param2).unwrap();
        wtr.write_f32::<LittleEndian>(self.param3).unwrap();
        wtr.write_f32::<LittleEndian>(self.param4).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_u16::<LittleEndian>(self.seq).unwrap();
        wtr.write_u16::<LittleEndian>(self.command).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.frame).unwrap();
        wtr.write_u8(self.current).unwrap();
        wtr.write_u8(self.autocontinue).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_REQUEST_DATA {
    pub seq: u16,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for MISSION_REQUEST_DATA {
    fn parse(payload: &[u8]) -> MISSION_REQUEST_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_REQUEST_DATA {
            seq: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.seq).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_SET_CURRENT_DATA {
    pub seq: u16,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for MISSION_SET_CURRENT_DATA {
    fn parse(payload: &[u8]) -> MISSION_SET_CURRENT_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_SET_CURRENT_DATA {
            seq: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.seq).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_CURRENT_DATA {
    pub seq: u16,
}

impl Parsable for MISSION_CURRENT_DATA {
    fn parse(payload: &[u8]) -> MISSION_CURRENT_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_CURRENT_DATA {
            seq: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.seq).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_REQUEST_LIST_DATA {
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for MISSION_REQUEST_LIST_DATA {
    fn parse(payload: &[u8]) -> MISSION_REQUEST_LIST_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_REQUEST_LIST_DATA {
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_COUNT_DATA {
    pub count: u16,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for MISSION_COUNT_DATA {
    fn parse(payload: &[u8]) -> MISSION_COUNT_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_COUNT_DATA {
            count: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.count).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_CLEAR_ALL_DATA {
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for MISSION_CLEAR_ALL_DATA {
    fn parse(payload: &[u8]) -> MISSION_CLEAR_ALL_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_CLEAR_ALL_DATA {
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_ITEM_REACHED_DATA {
    pub seq: u16,
}

impl Parsable for MISSION_ITEM_REACHED_DATA {
    fn parse(payload: &[u8]) -> MISSION_ITEM_REACHED_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_ITEM_REACHED_DATA {
            seq: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.seq).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_ACK_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub mavtype: u8,
}

impl Parsable for MISSION_ACK_DATA {
    fn parse(payload: &[u8]) -> MISSION_ACK_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_ACK_DATA {
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            mavtype: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.mavtype).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SET_GPS_GLOBAL_ORIGIN_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub target_system: u8,
}

impl Parsable for SET_GPS_GLOBAL_ORIGIN_DATA {
    fn parse(payload: &[u8]) -> SET_GPS_GLOBAL_ORIGIN_DATA {
        let mut cur = Cursor::new(payload);
        SET_GPS_GLOBAL_ORIGIN_DATA {
            latitude: cur.read_i32::<LittleEndian>().unwrap(),
            longitude: cur.read_i32::<LittleEndian>().unwrap(),
            altitude: cur.read_i32::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.latitude).unwrap();
        wtr.write_i32::<LittleEndian>(self.longitude).unwrap();
        wtr.write_i32::<LittleEndian>(self.altitude).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GPS_GLOBAL_ORIGIN_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
}

impl Parsable for GPS_GLOBAL_ORIGIN_DATA {
    fn parse(payload: &[u8]) -> GPS_GLOBAL_ORIGIN_DATA {
        let mut cur = Cursor::new(payload);
        GPS_GLOBAL_ORIGIN_DATA {
            latitude: cur.read_i32::<LittleEndian>().unwrap(),
            longitude: cur.read_i32::<LittleEndian>().unwrap(),
            altitude: cur.read_i32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.latitude).unwrap();
        wtr.write_i32::<LittleEndian>(self.longitude).unwrap();
        wtr.write_i32::<LittleEndian>(self.altitude).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct PARAM_MAP_RC_DATA {
    pub param_value0: f32,
    pub scale: f32,
    pub param_value_min: f32,
    pub param_value_max: f32,
    pub param_index: i16,
    pub target_system: u8,
    pub target_component: u8,
    pub param_id: Vec<u8> /* 16 */,
    pub parameter_rc_channel_index: u8,
}

impl Parsable for PARAM_MAP_RC_DATA {
    fn parse(payload: &[u8]) -> PARAM_MAP_RC_DATA {
        let mut cur = Cursor::new(payload);
        PARAM_MAP_RC_DATA {
            param_value0: cur.read_f32::<LittleEndian>().unwrap(),
            scale: cur.read_f32::<LittleEndian>().unwrap(),
            param_value_min: cur.read_f32::<LittleEndian>().unwrap(),
            param_value_max: cur.read_f32::<LittleEndian>().unwrap(),
            param_index: cur.read_i16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            param_id: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            parameter_rc_channel_index: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.param_value0).unwrap();
        wtr.write_f32::<LittleEndian>(self.scale).unwrap();
        wtr.write_f32::<LittleEndian>(self.param_value_min).unwrap();
        wtr.write_f32::<LittleEndian>(self.param_value_max).unwrap();
        wtr.write_i16::<LittleEndian>(self.param_index).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.param_id[0]).unwrap();
        wtr.write_u8(self.param_id[1]).unwrap();
        wtr.write_u8(self.param_id[2]).unwrap();
        wtr.write_u8(self.param_id[3]).unwrap();
        wtr.write_u8(self.param_id[4]).unwrap();
        wtr.write_u8(self.param_id[5]).unwrap();
        wtr.write_u8(self.param_id[6]).unwrap();
        wtr.write_u8(self.param_id[7]).unwrap();
        wtr.write_u8(self.param_id[8]).unwrap();
        wtr.write_u8(self.param_id[9]).unwrap();
        wtr.write_u8(self.param_id[10]).unwrap();
        wtr.write_u8(self.param_id[11]).unwrap();
        wtr.write_u8(self.param_id[12]).unwrap();
        wtr.write_u8(self.param_id[13]).unwrap();
        wtr.write_u8(self.param_id[14]).unwrap();
        wtr.write_u8(self.param_id[15]).unwrap();
        wtr.write_u8(self.parameter_rc_channel_index).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SAFETY_SET_ALLOWED_AREA_DATA {
    pub p1x: f32,
    pub p1y: f32,
    pub p1z: f32,
    pub p2x: f32,
    pub p2y: f32,
    pub p2z: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub frame: u8,
}

impl Parsable for SAFETY_SET_ALLOWED_AREA_DATA {
    fn parse(payload: &[u8]) -> SAFETY_SET_ALLOWED_AREA_DATA {
        let mut cur = Cursor::new(payload);
        SAFETY_SET_ALLOWED_AREA_DATA {
            p1x: cur.read_f32::<LittleEndian>().unwrap(),
            p1y: cur.read_f32::<LittleEndian>().unwrap(),
            p1z: cur.read_f32::<LittleEndian>().unwrap(),
            p2x: cur.read_f32::<LittleEndian>().unwrap(),
            p2y: cur.read_f32::<LittleEndian>().unwrap(),
            p2z: cur.read_f32::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            frame: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.p1x).unwrap();
        wtr.write_f32::<LittleEndian>(self.p1y).unwrap();
        wtr.write_f32::<LittleEndian>(self.p1z).unwrap();
        wtr.write_f32::<LittleEndian>(self.p2x).unwrap();
        wtr.write_f32::<LittleEndian>(self.p2y).unwrap();
        wtr.write_f32::<LittleEndian>(self.p2z).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.frame).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SAFETY_ALLOWED_AREA_DATA {
    pub p1x: f32,
    pub p1y: f32,
    pub p1z: f32,
    pub p2x: f32,
    pub p2y: f32,
    pub p2z: f32,
    pub frame: u8,
}

impl Parsable for SAFETY_ALLOWED_AREA_DATA {
    fn parse(payload: &[u8]) -> SAFETY_ALLOWED_AREA_DATA {
        let mut cur = Cursor::new(payload);
        SAFETY_ALLOWED_AREA_DATA {
            p1x: cur.read_f32::<LittleEndian>().unwrap(),
            p1y: cur.read_f32::<LittleEndian>().unwrap(),
            p1z: cur.read_f32::<LittleEndian>().unwrap(),
            p2x: cur.read_f32::<LittleEndian>().unwrap(),
            p2y: cur.read_f32::<LittleEndian>().unwrap(),
            p2z: cur.read_f32::<LittleEndian>().unwrap(),
            frame: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.p1x).unwrap();
        wtr.write_f32::<LittleEndian>(self.p1y).unwrap();
        wtr.write_f32::<LittleEndian>(self.p1z).unwrap();
        wtr.write_f32::<LittleEndian>(self.p2x).unwrap();
        wtr.write_f32::<LittleEndian>(self.p2y).unwrap();
        wtr.write_f32::<LittleEndian>(self.p2z).unwrap();
        wtr.write_u8(self.frame).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ATTITUDE_QUATERNION_COV_DATA {
    pub time_boot_ms: u32,
    pub q: Vec<f32> /* 4 */,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
    pub covariance: Vec<f32> /* 9 */,
}

impl Parsable for ATTITUDE_QUATERNION_COV_DATA {
    fn parse(payload: &[u8]) -> ATTITUDE_QUATERNION_COV_DATA {
        let mut cur = Cursor::new(payload);
        ATTITUDE_QUATERNION_COV_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            q: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            rollspeed: cur.read_f32::<LittleEndian>().unwrap(),
            pitchspeed: cur.read_f32::<LittleEndian>().unwrap(),
            yawspeed: cur.read_f32::<LittleEndian>().unwrap(),
            covariance: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.rollspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitchspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.yawspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[4]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[5]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[6]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[7]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[8]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct NAV_CONTROLLER_OUTPUT_DATA {
    pub nav_roll: f32,
    pub nav_pitch: f32,
    pub alt_error: f32,
    pub aspd_error: f32,
    pub xtrack_error: f32,
    pub nav_bearing: i16,
    pub target_bearing: i16,
    pub wp_dist: u16,
}

impl Parsable for NAV_CONTROLLER_OUTPUT_DATA {
    fn parse(payload: &[u8]) -> NAV_CONTROLLER_OUTPUT_DATA {
        let mut cur = Cursor::new(payload);
        NAV_CONTROLLER_OUTPUT_DATA {
            nav_roll: cur.read_f32::<LittleEndian>().unwrap(),
            nav_pitch: cur.read_f32::<LittleEndian>().unwrap(),
            alt_error: cur.read_f32::<LittleEndian>().unwrap(),
            aspd_error: cur.read_f32::<LittleEndian>().unwrap(),
            xtrack_error: cur.read_f32::<LittleEndian>().unwrap(),
            nav_bearing: cur.read_i16::<LittleEndian>().unwrap(),
            target_bearing: cur.read_i16::<LittleEndian>().unwrap(),
            wp_dist: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.nav_roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.nav_pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.alt_error).unwrap();
        wtr.write_f32::<LittleEndian>(self.aspd_error).unwrap();
        wtr.write_f32::<LittleEndian>(self.xtrack_error).unwrap();
        wtr.write_i16::<LittleEndian>(self.nav_bearing).unwrap();
        wtr.write_i16::<LittleEndian>(self.target_bearing).unwrap();
        wtr.write_u16::<LittleEndian>(self.wp_dist).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GLOBAL_POSITION_INT_COV_DATA {
    pub time_utc: u64,
    pub time_boot_ms: u32,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub relative_alt: i32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub covariance: Vec<f32> /* 36 */,
    pub estimator_type: u8,
}

impl Parsable for GLOBAL_POSITION_INT_COV_DATA {
    fn parse(payload: &[u8]) -> GLOBAL_POSITION_INT_COV_DATA {
        let mut cur = Cursor::new(payload);
        GLOBAL_POSITION_INT_COV_DATA {
            time_utc: cur.read_u64::<LittleEndian>().unwrap(),
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_i32::<LittleEndian>().unwrap(),
            relative_alt: cur.read_i32::<LittleEndian>().unwrap(),
            vx: cur.read_f32::<LittleEndian>().unwrap(),
            vy: cur.read_f32::<LittleEndian>().unwrap(),
            vz: cur.read_f32::<LittleEndian>().unwrap(),
            covariance: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            estimator_type: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_utc).unwrap();
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_i32::<LittleEndian>(self.alt).unwrap();
        wtr.write_i32::<LittleEndian>(self.relative_alt).unwrap();
        wtr.write_f32::<LittleEndian>(self.vx).unwrap();
        wtr.write_f32::<LittleEndian>(self.vy).unwrap();
        wtr.write_f32::<LittleEndian>(self.vz).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[4]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[5]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[6]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[7]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[8]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[9]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[10]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[11]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[12]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[13]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[14]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[15]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[16]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[17]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[18]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[19]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[20]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[21]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[22]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[23]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[24]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[25]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[26]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[27]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[28]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[29]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[30]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[31]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[32]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[33]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[34]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[35]).unwrap();
        wtr.write_u8(self.estimator_type).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOCAL_POSITION_NED_COV_DATA {
    pub time_utc: u64,
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
    pub covariance: Vec<f32> /* 45 */,
    pub estimator_type: u8,
}

impl Parsable for LOCAL_POSITION_NED_COV_DATA {
    fn parse(payload: &[u8]) -> LOCAL_POSITION_NED_COV_DATA {
        let mut cur = Cursor::new(payload);
        LOCAL_POSITION_NED_COV_DATA {
            time_utc: cur.read_u64::<LittleEndian>().unwrap(),
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            vx: cur.read_f32::<LittleEndian>().unwrap(),
            vy: cur.read_f32::<LittleEndian>().unwrap(),
            vz: cur.read_f32::<LittleEndian>().unwrap(),
            ax: cur.read_f32::<LittleEndian>().unwrap(),
            ay: cur.read_f32::<LittleEndian>().unwrap(),
            az: cur.read_f32::<LittleEndian>().unwrap(),
            covariance: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            estimator_type: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_utc).unwrap();
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.vx).unwrap();
        wtr.write_f32::<LittleEndian>(self.vy).unwrap();
        wtr.write_f32::<LittleEndian>(self.vz).unwrap();
        wtr.write_f32::<LittleEndian>(self.ax).unwrap();
        wtr.write_f32::<LittleEndian>(self.ay).unwrap();
        wtr.write_f32::<LittleEndian>(self.az).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[4]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[5]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[6]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[7]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[8]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[9]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[10]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[11]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[12]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[13]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[14]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[15]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[16]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[17]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[18]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[19]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[20]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[21]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[22]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[23]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[24]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[25]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[26]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[27]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[28]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[29]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[30]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[31]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[32]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[33]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[34]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[35]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[36]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[37]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[38]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[39]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[40]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[41]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[42]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[43]).unwrap();
        wtr.write_f32::<LittleEndian>(self.covariance[44]).unwrap();
        wtr.write_u8(self.estimator_type).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct RC_CHANNELS_DATA {
    pub time_boot_ms: u32,
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub chan9_raw: u16,
    pub chan10_raw: u16,
    pub chan11_raw: u16,
    pub chan12_raw: u16,
    pub chan13_raw: u16,
    pub chan14_raw: u16,
    pub chan15_raw: u16,
    pub chan16_raw: u16,
    pub chan17_raw: u16,
    pub chan18_raw: u16,
    pub chancount: u8,
    pub rssi: u8,
}

impl Parsable for RC_CHANNELS_DATA {
    fn parse(payload: &[u8]) -> RC_CHANNELS_DATA {
        let mut cur = Cursor::new(payload);
        RC_CHANNELS_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            chan1_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan2_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan3_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan4_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan5_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan6_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan7_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan8_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan9_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan10_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan11_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan12_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan13_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan14_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan15_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan16_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan17_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan18_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chancount: cur.read_u8().unwrap(),
            rssi: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan1_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan2_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan3_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan4_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan5_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan6_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan7_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan8_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan9_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan10_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan11_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan12_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan13_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan14_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan15_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan16_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan17_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan18_raw).unwrap();
        wtr.write_u8(self.chancount).unwrap();
        wtr.write_u8(self.rssi).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct REQUEST_DATA_STREAM_DATA {
    pub req_message_rate: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub req_stream_id: u8,
    pub start_stop: u8,
}

impl Parsable for REQUEST_DATA_STREAM_DATA {
    fn parse(payload: &[u8]) -> REQUEST_DATA_STREAM_DATA {
        let mut cur = Cursor::new(payload);
        REQUEST_DATA_STREAM_DATA {
            req_message_rate: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            req_stream_id: cur.read_u8().unwrap(),
            start_stop: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.req_message_rate).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.req_stream_id).unwrap();
        wtr.write_u8(self.start_stop).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct DATA_STREAM_DATA {
    pub message_rate: u16,
    pub stream_id: u8,
    pub on_off: u8,
}

impl Parsable for DATA_STREAM_DATA {
    fn parse(payload: &[u8]) -> DATA_STREAM_DATA {
        let mut cur = Cursor::new(payload);
        DATA_STREAM_DATA {
            message_rate: cur.read_u16::<LittleEndian>().unwrap(),
            stream_id: cur.read_u8().unwrap(),
            on_off: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.message_rate).unwrap();
        wtr.write_u8(self.stream_id).unwrap();
        wtr.write_u8(self.on_off).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MANUAL_CONTROL_DATA {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub r: i16,
    pub buttons: u16,
    pub target: u8,
}

impl Parsable for MANUAL_CONTROL_DATA {
    fn parse(payload: &[u8]) -> MANUAL_CONTROL_DATA {
        let mut cur = Cursor::new(payload);
        MANUAL_CONTROL_DATA {
            x: cur.read_i16::<LittleEndian>().unwrap(),
            y: cur.read_i16::<LittleEndian>().unwrap(),
            z: cur.read_i16::<LittleEndian>().unwrap(),
            r: cur.read_i16::<LittleEndian>().unwrap(),
            buttons: cur.read_u16::<LittleEndian>().unwrap(),
            target: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i16::<LittleEndian>(self.x).unwrap();
        wtr.write_i16::<LittleEndian>(self.y).unwrap();
        wtr.write_i16::<LittleEndian>(self.z).unwrap();
        wtr.write_i16::<LittleEndian>(self.r).unwrap();
        wtr.write_u16::<LittleEndian>(self.buttons).unwrap();
        wtr.write_u8(self.target).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct RC_CHANNELS_OVERRIDE_DATA {
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for RC_CHANNELS_OVERRIDE_DATA {
    fn parse(payload: &[u8]) -> RC_CHANNELS_OVERRIDE_DATA {
        let mut cur = Cursor::new(payload);
        RC_CHANNELS_OVERRIDE_DATA {
            chan1_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan2_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan3_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan4_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan5_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan6_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan7_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan8_raw: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.chan1_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan2_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan3_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan4_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan5_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan6_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan7_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan8_raw).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MISSION_ITEM_INT_DATA {
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub seq: u16,
    pub command: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub frame: u8,
    pub current: u8,
    pub autocontinue: u8,
}

impl Parsable for MISSION_ITEM_INT_DATA {
    fn parse(payload: &[u8]) -> MISSION_ITEM_INT_DATA {
        let mut cur = Cursor::new(payload);
        MISSION_ITEM_INT_DATA {
            param1: cur.read_f32::<LittleEndian>().unwrap(),
            param2: cur.read_f32::<LittleEndian>().unwrap(),
            param3: cur.read_f32::<LittleEndian>().unwrap(),
            param4: cur.read_f32::<LittleEndian>().unwrap(),
            x: cur.read_i32::<LittleEndian>().unwrap(),
            y: cur.read_i32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            seq: cur.read_u16::<LittleEndian>().unwrap(),
            command: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            frame: cur.read_u8().unwrap(),
            current: cur.read_u8().unwrap(),
            autocontinue: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.param1).unwrap();
        wtr.write_f32::<LittleEndian>(self.param2).unwrap();
        wtr.write_f32::<LittleEndian>(self.param3).unwrap();
        wtr.write_f32::<LittleEndian>(self.param4).unwrap();
        wtr.write_i32::<LittleEndian>(self.x).unwrap();
        wtr.write_i32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_u16::<LittleEndian>(self.seq).unwrap();
        wtr.write_u16::<LittleEndian>(self.command).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.frame).unwrap();
        wtr.write_u8(self.current).unwrap();
        wtr.write_u8(self.autocontinue).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct VFR_HUD_DATA {
    pub airspeed: f32,
    pub groundspeed: f32,
    pub alt: f32,
    pub climb: f32,
    pub heading: i16,
    pub throttle: u16,
}

impl Parsable for VFR_HUD_DATA {
    fn parse(payload: &[u8]) -> VFR_HUD_DATA {
        let mut cur = Cursor::new(payload);
        VFR_HUD_DATA {
            airspeed: cur.read_f32::<LittleEndian>().unwrap(),
            groundspeed: cur.read_f32::<LittleEndian>().unwrap(),
            alt: cur.read_f32::<LittleEndian>().unwrap(),
            climb: cur.read_f32::<LittleEndian>().unwrap(),
            heading: cur.read_i16::<LittleEndian>().unwrap(),
            throttle: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.airspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.groundspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.alt).unwrap();
        wtr.write_f32::<LittleEndian>(self.climb).unwrap();
        wtr.write_i16::<LittleEndian>(self.heading).unwrap();
        wtr.write_u16::<LittleEndian>(self.throttle).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct COMMAND_INT_DATA {
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub command: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub frame: u8,
    pub current: u8,
    pub autocontinue: u8,
}

impl Parsable for COMMAND_INT_DATA {
    fn parse(payload: &[u8]) -> COMMAND_INT_DATA {
        let mut cur = Cursor::new(payload);
        COMMAND_INT_DATA {
            param1: cur.read_f32::<LittleEndian>().unwrap(),
            param2: cur.read_f32::<LittleEndian>().unwrap(),
            param3: cur.read_f32::<LittleEndian>().unwrap(),
            param4: cur.read_f32::<LittleEndian>().unwrap(),
            x: cur.read_i32::<LittleEndian>().unwrap(),
            y: cur.read_i32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            command: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            frame: cur.read_u8().unwrap(),
            current: cur.read_u8().unwrap(),
            autocontinue: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.param1).unwrap();
        wtr.write_f32::<LittleEndian>(self.param2).unwrap();
        wtr.write_f32::<LittleEndian>(self.param3).unwrap();
        wtr.write_f32::<LittleEndian>(self.param4).unwrap();
        wtr.write_i32::<LittleEndian>(self.x).unwrap();
        wtr.write_i32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_u16::<LittleEndian>(self.command).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.frame).unwrap();
        wtr.write_u8(self.current).unwrap();
        wtr.write_u8(self.autocontinue).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct COMMAND_LONG_DATA {
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub param5: f32,
    pub param6: f32,
    pub param7: f32,
    pub command: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub confirmation: u8,
}

impl Parsable for COMMAND_LONG_DATA {
    fn parse(payload: &[u8]) -> COMMAND_LONG_DATA {
        let mut cur = Cursor::new(payload);
        COMMAND_LONG_DATA {
            param1: cur.read_f32::<LittleEndian>().unwrap(),
            param2: cur.read_f32::<LittleEndian>().unwrap(),
            param3: cur.read_f32::<LittleEndian>().unwrap(),
            param4: cur.read_f32::<LittleEndian>().unwrap(),
            param5: cur.read_f32::<LittleEndian>().unwrap(),
            param6: cur.read_f32::<LittleEndian>().unwrap(),
            param7: cur.read_f32::<LittleEndian>().unwrap(),
            command: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            confirmation: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.param1).unwrap();
        wtr.write_f32::<LittleEndian>(self.param2).unwrap();
        wtr.write_f32::<LittleEndian>(self.param3).unwrap();
        wtr.write_f32::<LittleEndian>(self.param4).unwrap();
        wtr.write_f32::<LittleEndian>(self.param5).unwrap();
        wtr.write_f32::<LittleEndian>(self.param6).unwrap();
        wtr.write_f32::<LittleEndian>(self.param7).unwrap();
        wtr.write_u16::<LittleEndian>(self.command).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.confirmation).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct COMMAND_ACK_DATA {
    pub command: u16,
    pub result: u8,
}

impl Parsable for COMMAND_ACK_DATA {
    fn parse(payload: &[u8]) -> COMMAND_ACK_DATA {
        let mut cur = Cursor::new(payload);
        COMMAND_ACK_DATA {
            command: cur.read_u16::<LittleEndian>().unwrap(),
            result: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.command).unwrap();
        wtr.write_u8(self.result).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MANUAL_SETPOINT_DATA {
    pub time_boot_ms: u32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub thrust: f32,
    pub mode_switch: u8,
    pub manual_override_switch: u8,
}

impl Parsable for MANUAL_SETPOINT_DATA {
    fn parse(payload: &[u8]) -> MANUAL_SETPOINT_DATA {
        let mut cur = Cursor::new(payload);
        MANUAL_SETPOINT_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            roll: cur.read_f32::<LittleEndian>().unwrap(),
            pitch: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
            thrust: cur.read_f32::<LittleEndian>().unwrap(),
            mode_switch: cur.read_u8().unwrap(),
            manual_override_switch: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr.write_f32::<LittleEndian>(self.thrust).unwrap();
        wtr.write_u8(self.mode_switch).unwrap();
        wtr.write_u8(self.manual_override_switch).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SET_ATTITUDE_TARGET_DATA {
    pub time_boot_ms: u32,
    pub q: Vec<f32> /* 4 */,
    pub body_roll_rate: f32,
    pub body_pitch_rate: f32,
    pub body_yaw_rate: f32,
    pub thrust: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub type_mask: u8,
}

impl Parsable for SET_ATTITUDE_TARGET_DATA {
    fn parse(payload: &[u8]) -> SET_ATTITUDE_TARGET_DATA {
        let mut cur = Cursor::new(payload);
        SET_ATTITUDE_TARGET_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            q: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            body_roll_rate: cur.read_f32::<LittleEndian>().unwrap(),
            body_pitch_rate: cur.read_f32::<LittleEndian>().unwrap(),
            body_yaw_rate: cur.read_f32::<LittleEndian>().unwrap(),
            thrust: cur.read_f32::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            type_mask: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.body_roll_rate).unwrap();
        wtr.write_f32::<LittleEndian>(self.body_pitch_rate).unwrap();
        wtr.write_f32::<LittleEndian>(self.body_yaw_rate).unwrap();
        wtr.write_f32::<LittleEndian>(self.thrust).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.type_mask).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ATTITUDE_TARGET_DATA {
    pub time_boot_ms: u32,
    pub q: Vec<f32> /* 4 */,
    pub body_roll_rate: f32,
    pub body_pitch_rate: f32,
    pub body_yaw_rate: f32,
    pub thrust: f32,
    pub type_mask: u8,
}

impl Parsable for ATTITUDE_TARGET_DATA {
    fn parse(payload: &[u8]) -> ATTITUDE_TARGET_DATA {
        let mut cur = Cursor::new(payload);
        ATTITUDE_TARGET_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            q: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            body_roll_rate: cur.read_f32::<LittleEndian>().unwrap(),
            body_pitch_rate: cur.read_f32::<LittleEndian>().unwrap(),
            body_yaw_rate: cur.read_f32::<LittleEndian>().unwrap(),
            thrust: cur.read_f32::<LittleEndian>().unwrap(),
            type_mask: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.body_roll_rate).unwrap();
        wtr.write_f32::<LittleEndian>(self.body_pitch_rate).unwrap();
        wtr.write_f32::<LittleEndian>(self.body_yaw_rate).unwrap();
        wtr.write_f32::<LittleEndian>(self.thrust).unwrap();
        wtr.write_u8(self.type_mask).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SET_POSITION_TARGET_LOCAL_NED_DATA {
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub afx: f32,
    pub afy: f32,
    pub afz: f32,
    pub yaw: f32,
    pub yaw_rate: f32,
    pub type_mask: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub coordinate_frame: u8,
}

impl Parsable for SET_POSITION_TARGET_LOCAL_NED_DATA {
    fn parse(payload: &[u8]) -> SET_POSITION_TARGET_LOCAL_NED_DATA {
        let mut cur = Cursor::new(payload);
        SET_POSITION_TARGET_LOCAL_NED_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            vx: cur.read_f32::<LittleEndian>().unwrap(),
            vy: cur.read_f32::<LittleEndian>().unwrap(),
            vz: cur.read_f32::<LittleEndian>().unwrap(),
            afx: cur.read_f32::<LittleEndian>().unwrap(),
            afy: cur.read_f32::<LittleEndian>().unwrap(),
            afz: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
            yaw_rate: cur.read_f32::<LittleEndian>().unwrap(),
            type_mask: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            coordinate_frame: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.vx).unwrap();
        wtr.write_f32::<LittleEndian>(self.vy).unwrap();
        wtr.write_f32::<LittleEndian>(self.vz).unwrap();
        wtr.write_f32::<LittleEndian>(self.afx).unwrap();
        wtr.write_f32::<LittleEndian>(self.afy).unwrap();
        wtr.write_f32::<LittleEndian>(self.afz).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw_rate).unwrap();
        wtr.write_u16::<LittleEndian>(self.type_mask).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.coordinate_frame).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct POSITION_TARGET_LOCAL_NED_DATA {
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub afx: f32,
    pub afy: f32,
    pub afz: f32,
    pub yaw: f32,
    pub yaw_rate: f32,
    pub type_mask: u16,
    pub coordinate_frame: u8,
}

impl Parsable for POSITION_TARGET_LOCAL_NED_DATA {
    fn parse(payload: &[u8]) -> POSITION_TARGET_LOCAL_NED_DATA {
        let mut cur = Cursor::new(payload);
        POSITION_TARGET_LOCAL_NED_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            vx: cur.read_f32::<LittleEndian>().unwrap(),
            vy: cur.read_f32::<LittleEndian>().unwrap(),
            vz: cur.read_f32::<LittleEndian>().unwrap(),
            afx: cur.read_f32::<LittleEndian>().unwrap(),
            afy: cur.read_f32::<LittleEndian>().unwrap(),
            afz: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
            yaw_rate: cur.read_f32::<LittleEndian>().unwrap(),
            type_mask: cur.read_u16::<LittleEndian>().unwrap(),
            coordinate_frame: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.vx).unwrap();
        wtr.write_f32::<LittleEndian>(self.vy).unwrap();
        wtr.write_f32::<LittleEndian>(self.vz).unwrap();
        wtr.write_f32::<LittleEndian>(self.afx).unwrap();
        wtr.write_f32::<LittleEndian>(self.afy).unwrap();
        wtr.write_f32::<LittleEndian>(self.afz).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw_rate).unwrap();
        wtr.write_u16::<LittleEndian>(self.type_mask).unwrap();
        wtr.write_u8(self.coordinate_frame).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SET_POSITION_TARGET_GLOBAL_INT_DATA {
    pub time_boot_ms: u32,
    pub lat_int: i32,
    pub lon_int: i32,
    pub alt: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub afx: f32,
    pub afy: f32,
    pub afz: f32,
    pub yaw: f32,
    pub yaw_rate: f32,
    pub type_mask: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub coordinate_frame: u8,
}

impl Parsable for SET_POSITION_TARGET_GLOBAL_INT_DATA {
    fn parse(payload: &[u8]) -> SET_POSITION_TARGET_GLOBAL_INT_DATA {
        let mut cur = Cursor::new(payload);
        SET_POSITION_TARGET_GLOBAL_INT_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            lat_int: cur.read_i32::<LittleEndian>().unwrap(),
            lon_int: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_f32::<LittleEndian>().unwrap(),
            vx: cur.read_f32::<LittleEndian>().unwrap(),
            vy: cur.read_f32::<LittleEndian>().unwrap(),
            vz: cur.read_f32::<LittleEndian>().unwrap(),
            afx: cur.read_f32::<LittleEndian>().unwrap(),
            afy: cur.read_f32::<LittleEndian>().unwrap(),
            afz: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
            yaw_rate: cur.read_f32::<LittleEndian>().unwrap(),
            type_mask: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            coordinate_frame: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat_int).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon_int).unwrap();
        wtr.write_f32::<LittleEndian>(self.alt).unwrap();
        wtr.write_f32::<LittleEndian>(self.vx).unwrap();
        wtr.write_f32::<LittleEndian>(self.vy).unwrap();
        wtr.write_f32::<LittleEndian>(self.vz).unwrap();
        wtr.write_f32::<LittleEndian>(self.afx).unwrap();
        wtr.write_f32::<LittleEndian>(self.afy).unwrap();
        wtr.write_f32::<LittleEndian>(self.afz).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw_rate).unwrap();
        wtr.write_u16::<LittleEndian>(self.type_mask).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.coordinate_frame).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct POSITION_TARGET_GLOBAL_INT_DATA {
    pub time_boot_ms: u32,
    pub lat_int: i32,
    pub lon_int: i32,
    pub alt: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub afx: f32,
    pub afy: f32,
    pub afz: f32,
    pub yaw: f32,
    pub yaw_rate: f32,
    pub type_mask: u16,
    pub coordinate_frame: u8,
}

impl Parsable for POSITION_TARGET_GLOBAL_INT_DATA {
    fn parse(payload: &[u8]) -> POSITION_TARGET_GLOBAL_INT_DATA {
        let mut cur = Cursor::new(payload);
        POSITION_TARGET_GLOBAL_INT_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            lat_int: cur.read_i32::<LittleEndian>().unwrap(),
            lon_int: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_f32::<LittleEndian>().unwrap(),
            vx: cur.read_f32::<LittleEndian>().unwrap(),
            vy: cur.read_f32::<LittleEndian>().unwrap(),
            vz: cur.read_f32::<LittleEndian>().unwrap(),
            afx: cur.read_f32::<LittleEndian>().unwrap(),
            afy: cur.read_f32::<LittleEndian>().unwrap(),
            afz: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
            yaw_rate: cur.read_f32::<LittleEndian>().unwrap(),
            type_mask: cur.read_u16::<LittleEndian>().unwrap(),
            coordinate_frame: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat_int).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon_int).unwrap();
        wtr.write_f32::<LittleEndian>(self.alt).unwrap();
        wtr.write_f32::<LittleEndian>(self.vx).unwrap();
        wtr.write_f32::<LittleEndian>(self.vy).unwrap();
        wtr.write_f32::<LittleEndian>(self.vz).unwrap();
        wtr.write_f32::<LittleEndian>(self.afx).unwrap();
        wtr.write_f32::<LittleEndian>(self.afy).unwrap();
        wtr.write_f32::<LittleEndian>(self.afz).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw_rate).unwrap();
        wtr.write_u16::<LittleEndian>(self.type_mask).unwrap();
        wtr.write_u8(self.coordinate_frame).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA {
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Parsable for LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA {
    fn parse(payload: &[u8]) -> LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA {
        let mut cur = Cursor::new(payload);
        LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            roll: cur.read_f32::<LittleEndian>().unwrap(),
            pitch: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HIL_STATE_DATA {
    pub time_usec: u64,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
}

impl Parsable for HIL_STATE_DATA {
    fn parse(payload: &[u8]) -> HIL_STATE_DATA {
        let mut cur = Cursor::new(payload);
        HIL_STATE_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            roll: cur.read_f32::<LittleEndian>().unwrap(),
            pitch: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
            rollspeed: cur.read_f32::<LittleEndian>().unwrap(),
            pitchspeed: cur.read_f32::<LittleEndian>().unwrap(),
            yawspeed: cur.read_f32::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_i32::<LittleEndian>().unwrap(),
            vx: cur.read_i16::<LittleEndian>().unwrap(),
            vy: cur.read_i16::<LittleEndian>().unwrap(),
            vz: cur.read_i16::<LittleEndian>().unwrap(),
            xacc: cur.read_i16::<LittleEndian>().unwrap(),
            yacc: cur.read_i16::<LittleEndian>().unwrap(),
            zacc: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr.write_f32::<LittleEndian>(self.rollspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitchspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.yawspeed).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_i32::<LittleEndian>(self.alt).unwrap();
        wtr.write_i16::<LittleEndian>(self.vx).unwrap();
        wtr.write_i16::<LittleEndian>(self.vy).unwrap();
        wtr.write_i16::<LittleEndian>(self.vz).unwrap();
        wtr.write_i16::<LittleEndian>(self.xacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.yacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.zacc).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HIL_CONTROLS_DATA {
    pub time_usec: u64,
    pub roll_ailerons: f32,
    pub pitch_elevator: f32,
    pub yaw_rudder: f32,
    pub throttle: f32,
    pub aux1: f32,
    pub aux2: f32,
    pub aux3: f32,
    pub aux4: f32,
    pub mode: u8,
    pub nav_mode: u8,
}

impl Parsable for HIL_CONTROLS_DATA {
    fn parse(payload: &[u8]) -> HIL_CONTROLS_DATA {
        let mut cur = Cursor::new(payload);
        HIL_CONTROLS_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            roll_ailerons: cur.read_f32::<LittleEndian>().unwrap(),
            pitch_elevator: cur.read_f32::<LittleEndian>().unwrap(),
            yaw_rudder: cur.read_f32::<LittleEndian>().unwrap(),
            throttle: cur.read_f32::<LittleEndian>().unwrap(),
            aux1: cur.read_f32::<LittleEndian>().unwrap(),
            aux2: cur.read_f32::<LittleEndian>().unwrap(),
            aux3: cur.read_f32::<LittleEndian>().unwrap(),
            aux4: cur.read_f32::<LittleEndian>().unwrap(),
            mode: cur.read_u8().unwrap(),
            nav_mode: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll_ailerons).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch_elevator).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw_rudder).unwrap();
        wtr.write_f32::<LittleEndian>(self.throttle).unwrap();
        wtr.write_f32::<LittleEndian>(self.aux1).unwrap();
        wtr.write_f32::<LittleEndian>(self.aux2).unwrap();
        wtr.write_f32::<LittleEndian>(self.aux3).unwrap();
        wtr.write_f32::<LittleEndian>(self.aux4).unwrap();
        wtr.write_u8(self.mode).unwrap();
        wtr.write_u8(self.nav_mode).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HIL_RC_INPUTS_RAW_DATA {
    pub time_usec: u64,
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub chan9_raw: u16,
    pub chan10_raw: u16,
    pub chan11_raw: u16,
    pub chan12_raw: u16,
    pub rssi: u8,
}

impl Parsable for HIL_RC_INPUTS_RAW_DATA {
    fn parse(payload: &[u8]) -> HIL_RC_INPUTS_RAW_DATA {
        let mut cur = Cursor::new(payload);
        HIL_RC_INPUTS_RAW_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            chan1_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan2_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan3_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan4_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan5_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan6_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan7_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan8_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan9_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan10_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan11_raw: cur.read_u16::<LittleEndian>().unwrap(),
            chan12_raw: cur.read_u16::<LittleEndian>().unwrap(),
            rssi: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan1_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan2_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan3_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan4_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan5_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan6_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan7_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan8_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan9_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan10_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan11_raw).unwrap();
        wtr.write_u16::<LittleEndian>(self.chan12_raw).unwrap();
        wtr.write_u8(self.rssi).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct OPTICAL_FLOW_DATA {
    pub time_usec: u64,
    pub flow_comp_m_x: f32,
    pub flow_comp_m_y: f32,
    pub ground_distance: f32,
    pub flow_x: i16,
    pub flow_y: i16,
    pub sensor_id: u8,
    pub quality: u8,
}

impl Parsable for OPTICAL_FLOW_DATA {
    fn parse(payload: &[u8]) -> OPTICAL_FLOW_DATA {
        let mut cur = Cursor::new(payload);
        OPTICAL_FLOW_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            flow_comp_m_x: cur.read_f32::<LittleEndian>().unwrap(),
            flow_comp_m_y: cur.read_f32::<LittleEndian>().unwrap(),
            ground_distance: cur.read_f32::<LittleEndian>().unwrap(),
            flow_x: cur.read_i16::<LittleEndian>().unwrap(),
            flow_y: cur.read_i16::<LittleEndian>().unwrap(),
            sensor_id: cur.read_u8().unwrap(),
            quality: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.flow_comp_m_x).unwrap();
        wtr.write_f32::<LittleEndian>(self.flow_comp_m_y).unwrap();
        wtr.write_f32::<LittleEndian>(self.ground_distance).unwrap();
        wtr.write_i16::<LittleEndian>(self.flow_x).unwrap();
        wtr.write_i16::<LittleEndian>(self.flow_y).unwrap();
        wtr.write_u8(self.sensor_id).unwrap();
        wtr.write_u8(self.quality).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GLOBAL_VISION_POSITION_ESTIMATE_DATA {
    pub usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Parsable for GLOBAL_VISION_POSITION_ESTIMATE_DATA {
    fn parse(payload: &[u8]) -> GLOBAL_VISION_POSITION_ESTIMATE_DATA {
        let mut cur = Cursor::new(payload);
        GLOBAL_VISION_POSITION_ESTIMATE_DATA {
            usec: cur.read_u64::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            roll: cur.read_f32::<LittleEndian>().unwrap(),
            pitch: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct VISION_POSITION_ESTIMATE_DATA {
    pub usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Parsable for VISION_POSITION_ESTIMATE_DATA {
    fn parse(payload: &[u8]) -> VISION_POSITION_ESTIMATE_DATA {
        let mut cur = Cursor::new(payload);
        VISION_POSITION_ESTIMATE_DATA {
            usec: cur.read_u64::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            roll: cur.read_f32::<LittleEndian>().unwrap(),
            pitch: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct VISION_SPEED_ESTIMATE_DATA {
    pub usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Parsable for VISION_SPEED_ESTIMATE_DATA {
    fn parse(payload: &[u8]) -> VISION_SPEED_ESTIMATE_DATA {
        let mut cur = Cursor::new(payload);
        VISION_SPEED_ESTIMATE_DATA {
            usec: cur.read_u64::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct VICON_POSITION_ESTIMATE_DATA {
    pub usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Parsable for VICON_POSITION_ESTIMATE_DATA {
    fn parse(payload: &[u8]) -> VICON_POSITION_ESTIMATE_DATA {
        let mut cur = Cursor::new(payload);
        VICON_POSITION_ESTIMATE_DATA {
            usec: cur.read_u64::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            roll: cur.read_f32::<LittleEndian>().unwrap(),
            pitch: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HIGHRES_IMU_DATA {
    pub time_usec: u64,
    pub xacc: f32,
    pub yacc: f32,
    pub zacc: f32,
    pub xgyro: f32,
    pub ygyro: f32,
    pub zgyro: f32,
    pub xmag: f32,
    pub ymag: f32,
    pub zmag: f32,
    pub abs_pressure: f32,
    pub diff_pressure: f32,
    pub pressure_alt: f32,
    pub temperature: f32,
    pub fields_updated: u16,
}

impl Parsable for HIGHRES_IMU_DATA {
    fn parse(payload: &[u8]) -> HIGHRES_IMU_DATA {
        let mut cur = Cursor::new(payload);
        HIGHRES_IMU_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            xacc: cur.read_f32::<LittleEndian>().unwrap(),
            yacc: cur.read_f32::<LittleEndian>().unwrap(),
            zacc: cur.read_f32::<LittleEndian>().unwrap(),
            xgyro: cur.read_f32::<LittleEndian>().unwrap(),
            ygyro: cur.read_f32::<LittleEndian>().unwrap(),
            zgyro: cur.read_f32::<LittleEndian>().unwrap(),
            xmag: cur.read_f32::<LittleEndian>().unwrap(),
            ymag: cur.read_f32::<LittleEndian>().unwrap(),
            zmag: cur.read_f32::<LittleEndian>().unwrap(),
            abs_pressure: cur.read_f32::<LittleEndian>().unwrap(),
            diff_pressure: cur.read_f32::<LittleEndian>().unwrap(),
            pressure_alt: cur.read_f32::<LittleEndian>().unwrap(),
            temperature: cur.read_f32::<LittleEndian>().unwrap(),
            fields_updated: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.xacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.yacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.zacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.xgyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.ygyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.zgyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.xmag).unwrap();
        wtr.write_f32::<LittleEndian>(self.ymag).unwrap();
        wtr.write_f32::<LittleEndian>(self.zmag).unwrap();
        wtr.write_f32::<LittleEndian>(self.abs_pressure).unwrap();
        wtr.write_f32::<LittleEndian>(self.diff_pressure).unwrap();
        wtr.write_f32::<LittleEndian>(self.pressure_alt).unwrap();
        wtr.write_f32::<LittleEndian>(self.temperature).unwrap();
        wtr.write_u16::<LittleEndian>(self.fields_updated).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct OPTICAL_FLOW_RAD_DATA {
    pub time_usec: u64,
    pub integration_time_us: u32,
    pub integrated_x: f32,
    pub integrated_y: f32,
    pub integrated_xgyro: f32,
    pub integrated_ygyro: f32,
    pub integrated_zgyro: f32,
    pub time_delta_distance_us: u32,
    pub distance: f32,
    pub temperature: i16,
    pub sensor_id: u8,
    pub quality: u8,
}

impl Parsable for OPTICAL_FLOW_RAD_DATA {
    fn parse(payload: &[u8]) -> OPTICAL_FLOW_RAD_DATA {
        let mut cur = Cursor::new(payload);
        OPTICAL_FLOW_RAD_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            integration_time_us: cur.read_u32::<LittleEndian>().unwrap(),
            integrated_x: cur.read_f32::<LittleEndian>().unwrap(),
            integrated_y: cur.read_f32::<LittleEndian>().unwrap(),
            integrated_xgyro: cur.read_f32::<LittleEndian>().unwrap(),
            integrated_ygyro: cur.read_f32::<LittleEndian>().unwrap(),
            integrated_zgyro: cur.read_f32::<LittleEndian>().unwrap(),
            time_delta_distance_us: cur.read_u32::<LittleEndian>().unwrap(),
            distance: cur.read_f32::<LittleEndian>().unwrap(),
            temperature: cur.read_i16::<LittleEndian>().unwrap(),
            sensor_id: cur.read_u8().unwrap(),
            quality: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_u32::<LittleEndian>(self.integration_time_us).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_x).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_y).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_xgyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_ygyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_zgyro).unwrap();
        wtr.write_u32::<LittleEndian>(self.time_delta_distance_us).unwrap();
        wtr.write_f32::<LittleEndian>(self.distance).unwrap();
        wtr.write_i16::<LittleEndian>(self.temperature).unwrap();
        wtr.write_u8(self.sensor_id).unwrap();
        wtr.write_u8(self.quality).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HIL_SENSOR_DATA {
    pub time_usec: u64,
    pub xacc: f32,
    pub yacc: f32,
    pub zacc: f32,
    pub xgyro: f32,
    pub ygyro: f32,
    pub zgyro: f32,
    pub xmag: f32,
    pub ymag: f32,
    pub zmag: f32,
    pub abs_pressure: f32,
    pub diff_pressure: f32,
    pub pressure_alt: f32,
    pub temperature: f32,
    pub fields_updated: u32,
}

impl Parsable for HIL_SENSOR_DATA {
    fn parse(payload: &[u8]) -> HIL_SENSOR_DATA {
        let mut cur = Cursor::new(payload);
        HIL_SENSOR_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            xacc: cur.read_f32::<LittleEndian>().unwrap(),
            yacc: cur.read_f32::<LittleEndian>().unwrap(),
            zacc: cur.read_f32::<LittleEndian>().unwrap(),
            xgyro: cur.read_f32::<LittleEndian>().unwrap(),
            ygyro: cur.read_f32::<LittleEndian>().unwrap(),
            zgyro: cur.read_f32::<LittleEndian>().unwrap(),
            xmag: cur.read_f32::<LittleEndian>().unwrap(),
            ymag: cur.read_f32::<LittleEndian>().unwrap(),
            zmag: cur.read_f32::<LittleEndian>().unwrap(),
            abs_pressure: cur.read_f32::<LittleEndian>().unwrap(),
            diff_pressure: cur.read_f32::<LittleEndian>().unwrap(),
            pressure_alt: cur.read_f32::<LittleEndian>().unwrap(),
            temperature: cur.read_f32::<LittleEndian>().unwrap(),
            fields_updated: cur.read_u32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.xacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.yacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.zacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.xgyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.ygyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.zgyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.xmag).unwrap();
        wtr.write_f32::<LittleEndian>(self.ymag).unwrap();
        wtr.write_f32::<LittleEndian>(self.zmag).unwrap();
        wtr.write_f32::<LittleEndian>(self.abs_pressure).unwrap();
        wtr.write_f32::<LittleEndian>(self.diff_pressure).unwrap();
        wtr.write_f32::<LittleEndian>(self.pressure_alt).unwrap();
        wtr.write_f32::<LittleEndian>(self.temperature).unwrap();
        wtr.write_u32::<LittleEndian>(self.fields_updated).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SIM_STATE_DATA {
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub q4: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub xacc: f32,
    pub yacc: f32,
    pub zacc: f32,
    pub xgyro: f32,
    pub ygyro: f32,
    pub zgyro: f32,
    pub lat: f32,
    pub lon: f32,
    pub alt: f32,
    pub std_dev_horz: f32,
    pub std_dev_vert: f32,
    pub vn: f32,
    pub ve: f32,
    pub vd: f32,
}

impl Parsable for SIM_STATE_DATA {
    fn parse(payload: &[u8]) -> SIM_STATE_DATA {
        let mut cur = Cursor::new(payload);
        SIM_STATE_DATA {
            q1: cur.read_f32::<LittleEndian>().unwrap(),
            q2: cur.read_f32::<LittleEndian>().unwrap(),
            q3: cur.read_f32::<LittleEndian>().unwrap(),
            q4: cur.read_f32::<LittleEndian>().unwrap(),
            roll: cur.read_f32::<LittleEndian>().unwrap(),
            pitch: cur.read_f32::<LittleEndian>().unwrap(),
            yaw: cur.read_f32::<LittleEndian>().unwrap(),
            xacc: cur.read_f32::<LittleEndian>().unwrap(),
            yacc: cur.read_f32::<LittleEndian>().unwrap(),
            zacc: cur.read_f32::<LittleEndian>().unwrap(),
            xgyro: cur.read_f32::<LittleEndian>().unwrap(),
            ygyro: cur.read_f32::<LittleEndian>().unwrap(),
            zgyro: cur.read_f32::<LittleEndian>().unwrap(),
            lat: cur.read_f32::<LittleEndian>().unwrap(),
            lon: cur.read_f32::<LittleEndian>().unwrap(),
            alt: cur.read_f32::<LittleEndian>().unwrap(),
            std_dev_horz: cur.read_f32::<LittleEndian>().unwrap(),
            std_dev_vert: cur.read_f32::<LittleEndian>().unwrap(),
            vn: cur.read_f32::<LittleEndian>().unwrap(),
            ve: cur.read_f32::<LittleEndian>().unwrap(),
            vd: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(self.q1).unwrap();
        wtr.write_f32::<LittleEndian>(self.q2).unwrap();
        wtr.write_f32::<LittleEndian>(self.q3).unwrap();
        wtr.write_f32::<LittleEndian>(self.q4).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw).unwrap();
        wtr.write_f32::<LittleEndian>(self.xacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.yacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.zacc).unwrap();
        wtr.write_f32::<LittleEndian>(self.xgyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.ygyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.zgyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.lat).unwrap();
        wtr.write_f32::<LittleEndian>(self.lon).unwrap();
        wtr.write_f32::<LittleEndian>(self.alt).unwrap();
        wtr.write_f32::<LittleEndian>(self.std_dev_horz).unwrap();
        wtr.write_f32::<LittleEndian>(self.std_dev_vert).unwrap();
        wtr.write_f32::<LittleEndian>(self.vn).unwrap();
        wtr.write_f32::<LittleEndian>(self.ve).unwrap();
        wtr.write_f32::<LittleEndian>(self.vd).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct RADIO_STATUS_DATA {
    pub rxerrors: u16,
    pub fixed: u16,
    pub rssi: u8,
    pub remrssi: u8,
    pub txbuf: u8,
    pub noise: u8,
    pub remnoise: u8,
}

impl Parsable for RADIO_STATUS_DATA {
    fn parse(payload: &[u8]) -> RADIO_STATUS_DATA {
        let mut cur = Cursor::new(payload);
        RADIO_STATUS_DATA {
            rxerrors: cur.read_u16::<LittleEndian>().unwrap(),
            fixed: cur.read_u16::<LittleEndian>().unwrap(),
            rssi: cur.read_u8().unwrap(),
            remrssi: cur.read_u8().unwrap(),
            txbuf: cur.read_u8().unwrap(),
            noise: cur.read_u8().unwrap(),
            remnoise: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.rxerrors).unwrap();
        wtr.write_u16::<LittleEndian>(self.fixed).unwrap();
        wtr.write_u8(self.rssi).unwrap();
        wtr.write_u8(self.remrssi).unwrap();
        wtr.write_u8(self.txbuf).unwrap();
        wtr.write_u8(self.noise).unwrap();
        wtr.write_u8(self.remnoise).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct FILE_TRANSFER_PROTOCOL_DATA {
    pub target_network: u8,
    pub target_system: u8,
    pub target_component: u8,
    pub payload: Vec<u8> /* 251 */,
}

impl Parsable for FILE_TRANSFER_PROTOCOL_DATA {
    fn parse(payload: &[u8]) -> FILE_TRANSFER_PROTOCOL_DATA {
        let mut cur = Cursor::new(payload);
        FILE_TRANSFER_PROTOCOL_DATA {
            target_network: cur.read_u8().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            payload: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_network).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.payload[0]).unwrap();
        wtr.write_u8(self.payload[1]).unwrap();
        wtr.write_u8(self.payload[2]).unwrap();
        wtr.write_u8(self.payload[3]).unwrap();
        wtr.write_u8(self.payload[4]).unwrap();
        wtr.write_u8(self.payload[5]).unwrap();
        wtr.write_u8(self.payload[6]).unwrap();
        wtr.write_u8(self.payload[7]).unwrap();
        wtr.write_u8(self.payload[8]).unwrap();
        wtr.write_u8(self.payload[9]).unwrap();
        wtr.write_u8(self.payload[10]).unwrap();
        wtr.write_u8(self.payload[11]).unwrap();
        wtr.write_u8(self.payload[12]).unwrap();
        wtr.write_u8(self.payload[13]).unwrap();
        wtr.write_u8(self.payload[14]).unwrap();
        wtr.write_u8(self.payload[15]).unwrap();
        wtr.write_u8(self.payload[16]).unwrap();
        wtr.write_u8(self.payload[17]).unwrap();
        wtr.write_u8(self.payload[18]).unwrap();
        wtr.write_u8(self.payload[19]).unwrap();
        wtr.write_u8(self.payload[20]).unwrap();
        wtr.write_u8(self.payload[21]).unwrap();
        wtr.write_u8(self.payload[22]).unwrap();
        wtr.write_u8(self.payload[23]).unwrap();
        wtr.write_u8(self.payload[24]).unwrap();
        wtr.write_u8(self.payload[25]).unwrap();
        wtr.write_u8(self.payload[26]).unwrap();
        wtr.write_u8(self.payload[27]).unwrap();
        wtr.write_u8(self.payload[28]).unwrap();
        wtr.write_u8(self.payload[29]).unwrap();
        wtr.write_u8(self.payload[30]).unwrap();
        wtr.write_u8(self.payload[31]).unwrap();
        wtr.write_u8(self.payload[32]).unwrap();
        wtr.write_u8(self.payload[33]).unwrap();
        wtr.write_u8(self.payload[34]).unwrap();
        wtr.write_u8(self.payload[35]).unwrap();
        wtr.write_u8(self.payload[36]).unwrap();
        wtr.write_u8(self.payload[37]).unwrap();
        wtr.write_u8(self.payload[38]).unwrap();
        wtr.write_u8(self.payload[39]).unwrap();
        wtr.write_u8(self.payload[40]).unwrap();
        wtr.write_u8(self.payload[41]).unwrap();
        wtr.write_u8(self.payload[42]).unwrap();
        wtr.write_u8(self.payload[43]).unwrap();
        wtr.write_u8(self.payload[44]).unwrap();
        wtr.write_u8(self.payload[45]).unwrap();
        wtr.write_u8(self.payload[46]).unwrap();
        wtr.write_u8(self.payload[47]).unwrap();
        wtr.write_u8(self.payload[48]).unwrap();
        wtr.write_u8(self.payload[49]).unwrap();
        wtr.write_u8(self.payload[50]).unwrap();
        wtr.write_u8(self.payload[51]).unwrap();
        wtr.write_u8(self.payload[52]).unwrap();
        wtr.write_u8(self.payload[53]).unwrap();
        wtr.write_u8(self.payload[54]).unwrap();
        wtr.write_u8(self.payload[55]).unwrap();
        wtr.write_u8(self.payload[56]).unwrap();
        wtr.write_u8(self.payload[57]).unwrap();
        wtr.write_u8(self.payload[58]).unwrap();
        wtr.write_u8(self.payload[59]).unwrap();
        wtr.write_u8(self.payload[60]).unwrap();
        wtr.write_u8(self.payload[61]).unwrap();
        wtr.write_u8(self.payload[62]).unwrap();
        wtr.write_u8(self.payload[63]).unwrap();
        wtr.write_u8(self.payload[64]).unwrap();
        wtr.write_u8(self.payload[65]).unwrap();
        wtr.write_u8(self.payload[66]).unwrap();
        wtr.write_u8(self.payload[67]).unwrap();
        wtr.write_u8(self.payload[68]).unwrap();
        wtr.write_u8(self.payload[69]).unwrap();
        wtr.write_u8(self.payload[70]).unwrap();
        wtr.write_u8(self.payload[71]).unwrap();
        wtr.write_u8(self.payload[72]).unwrap();
        wtr.write_u8(self.payload[73]).unwrap();
        wtr.write_u8(self.payload[74]).unwrap();
        wtr.write_u8(self.payload[75]).unwrap();
        wtr.write_u8(self.payload[76]).unwrap();
        wtr.write_u8(self.payload[77]).unwrap();
        wtr.write_u8(self.payload[78]).unwrap();
        wtr.write_u8(self.payload[79]).unwrap();
        wtr.write_u8(self.payload[80]).unwrap();
        wtr.write_u8(self.payload[81]).unwrap();
        wtr.write_u8(self.payload[82]).unwrap();
        wtr.write_u8(self.payload[83]).unwrap();
        wtr.write_u8(self.payload[84]).unwrap();
        wtr.write_u8(self.payload[85]).unwrap();
        wtr.write_u8(self.payload[86]).unwrap();
        wtr.write_u8(self.payload[87]).unwrap();
        wtr.write_u8(self.payload[88]).unwrap();
        wtr.write_u8(self.payload[89]).unwrap();
        wtr.write_u8(self.payload[90]).unwrap();
        wtr.write_u8(self.payload[91]).unwrap();
        wtr.write_u8(self.payload[92]).unwrap();
        wtr.write_u8(self.payload[93]).unwrap();
        wtr.write_u8(self.payload[94]).unwrap();
        wtr.write_u8(self.payload[95]).unwrap();
        wtr.write_u8(self.payload[96]).unwrap();
        wtr.write_u8(self.payload[97]).unwrap();
        wtr.write_u8(self.payload[98]).unwrap();
        wtr.write_u8(self.payload[99]).unwrap();
        wtr.write_u8(self.payload[100]).unwrap();
        wtr.write_u8(self.payload[101]).unwrap();
        wtr.write_u8(self.payload[102]).unwrap();
        wtr.write_u8(self.payload[103]).unwrap();
        wtr.write_u8(self.payload[104]).unwrap();
        wtr.write_u8(self.payload[105]).unwrap();
        wtr.write_u8(self.payload[106]).unwrap();
        wtr.write_u8(self.payload[107]).unwrap();
        wtr.write_u8(self.payload[108]).unwrap();
        wtr.write_u8(self.payload[109]).unwrap();
        wtr.write_u8(self.payload[110]).unwrap();
        wtr.write_u8(self.payload[111]).unwrap();
        wtr.write_u8(self.payload[112]).unwrap();
        wtr.write_u8(self.payload[113]).unwrap();
        wtr.write_u8(self.payload[114]).unwrap();
        wtr.write_u8(self.payload[115]).unwrap();
        wtr.write_u8(self.payload[116]).unwrap();
        wtr.write_u8(self.payload[117]).unwrap();
        wtr.write_u8(self.payload[118]).unwrap();
        wtr.write_u8(self.payload[119]).unwrap();
        wtr.write_u8(self.payload[120]).unwrap();
        wtr.write_u8(self.payload[121]).unwrap();
        wtr.write_u8(self.payload[122]).unwrap();
        wtr.write_u8(self.payload[123]).unwrap();
        wtr.write_u8(self.payload[124]).unwrap();
        wtr.write_u8(self.payload[125]).unwrap();
        wtr.write_u8(self.payload[126]).unwrap();
        wtr.write_u8(self.payload[127]).unwrap();
        wtr.write_u8(self.payload[128]).unwrap();
        wtr.write_u8(self.payload[129]).unwrap();
        wtr.write_u8(self.payload[130]).unwrap();
        wtr.write_u8(self.payload[131]).unwrap();
        wtr.write_u8(self.payload[132]).unwrap();
        wtr.write_u8(self.payload[133]).unwrap();
        wtr.write_u8(self.payload[134]).unwrap();
        wtr.write_u8(self.payload[135]).unwrap();
        wtr.write_u8(self.payload[136]).unwrap();
        wtr.write_u8(self.payload[137]).unwrap();
        wtr.write_u8(self.payload[138]).unwrap();
        wtr.write_u8(self.payload[139]).unwrap();
        wtr.write_u8(self.payload[140]).unwrap();
        wtr.write_u8(self.payload[141]).unwrap();
        wtr.write_u8(self.payload[142]).unwrap();
        wtr.write_u8(self.payload[143]).unwrap();
        wtr.write_u8(self.payload[144]).unwrap();
        wtr.write_u8(self.payload[145]).unwrap();
        wtr.write_u8(self.payload[146]).unwrap();
        wtr.write_u8(self.payload[147]).unwrap();
        wtr.write_u8(self.payload[148]).unwrap();
        wtr.write_u8(self.payload[149]).unwrap();
        wtr.write_u8(self.payload[150]).unwrap();
        wtr.write_u8(self.payload[151]).unwrap();
        wtr.write_u8(self.payload[152]).unwrap();
        wtr.write_u8(self.payload[153]).unwrap();
        wtr.write_u8(self.payload[154]).unwrap();
        wtr.write_u8(self.payload[155]).unwrap();
        wtr.write_u8(self.payload[156]).unwrap();
        wtr.write_u8(self.payload[157]).unwrap();
        wtr.write_u8(self.payload[158]).unwrap();
        wtr.write_u8(self.payload[159]).unwrap();
        wtr.write_u8(self.payload[160]).unwrap();
        wtr.write_u8(self.payload[161]).unwrap();
        wtr.write_u8(self.payload[162]).unwrap();
        wtr.write_u8(self.payload[163]).unwrap();
        wtr.write_u8(self.payload[164]).unwrap();
        wtr.write_u8(self.payload[165]).unwrap();
        wtr.write_u8(self.payload[166]).unwrap();
        wtr.write_u8(self.payload[167]).unwrap();
        wtr.write_u8(self.payload[168]).unwrap();
        wtr.write_u8(self.payload[169]).unwrap();
        wtr.write_u8(self.payload[170]).unwrap();
        wtr.write_u8(self.payload[171]).unwrap();
        wtr.write_u8(self.payload[172]).unwrap();
        wtr.write_u8(self.payload[173]).unwrap();
        wtr.write_u8(self.payload[174]).unwrap();
        wtr.write_u8(self.payload[175]).unwrap();
        wtr.write_u8(self.payload[176]).unwrap();
        wtr.write_u8(self.payload[177]).unwrap();
        wtr.write_u8(self.payload[178]).unwrap();
        wtr.write_u8(self.payload[179]).unwrap();
        wtr.write_u8(self.payload[180]).unwrap();
        wtr.write_u8(self.payload[181]).unwrap();
        wtr.write_u8(self.payload[182]).unwrap();
        wtr.write_u8(self.payload[183]).unwrap();
        wtr.write_u8(self.payload[184]).unwrap();
        wtr.write_u8(self.payload[185]).unwrap();
        wtr.write_u8(self.payload[186]).unwrap();
        wtr.write_u8(self.payload[187]).unwrap();
        wtr.write_u8(self.payload[188]).unwrap();
        wtr.write_u8(self.payload[189]).unwrap();
        wtr.write_u8(self.payload[190]).unwrap();
        wtr.write_u8(self.payload[191]).unwrap();
        wtr.write_u8(self.payload[192]).unwrap();
        wtr.write_u8(self.payload[193]).unwrap();
        wtr.write_u8(self.payload[194]).unwrap();
        wtr.write_u8(self.payload[195]).unwrap();
        wtr.write_u8(self.payload[196]).unwrap();
        wtr.write_u8(self.payload[197]).unwrap();
        wtr.write_u8(self.payload[198]).unwrap();
        wtr.write_u8(self.payload[199]).unwrap();
        wtr.write_u8(self.payload[200]).unwrap();
        wtr.write_u8(self.payload[201]).unwrap();
        wtr.write_u8(self.payload[202]).unwrap();
        wtr.write_u8(self.payload[203]).unwrap();
        wtr.write_u8(self.payload[204]).unwrap();
        wtr.write_u8(self.payload[205]).unwrap();
        wtr.write_u8(self.payload[206]).unwrap();
        wtr.write_u8(self.payload[207]).unwrap();
        wtr.write_u8(self.payload[208]).unwrap();
        wtr.write_u8(self.payload[209]).unwrap();
        wtr.write_u8(self.payload[210]).unwrap();
        wtr.write_u8(self.payload[211]).unwrap();
        wtr.write_u8(self.payload[212]).unwrap();
        wtr.write_u8(self.payload[213]).unwrap();
        wtr.write_u8(self.payload[214]).unwrap();
        wtr.write_u8(self.payload[215]).unwrap();
        wtr.write_u8(self.payload[216]).unwrap();
        wtr.write_u8(self.payload[217]).unwrap();
        wtr.write_u8(self.payload[218]).unwrap();
        wtr.write_u8(self.payload[219]).unwrap();
        wtr.write_u8(self.payload[220]).unwrap();
        wtr.write_u8(self.payload[221]).unwrap();
        wtr.write_u8(self.payload[222]).unwrap();
        wtr.write_u8(self.payload[223]).unwrap();
        wtr.write_u8(self.payload[224]).unwrap();
        wtr.write_u8(self.payload[225]).unwrap();
        wtr.write_u8(self.payload[226]).unwrap();
        wtr.write_u8(self.payload[227]).unwrap();
        wtr.write_u8(self.payload[228]).unwrap();
        wtr.write_u8(self.payload[229]).unwrap();
        wtr.write_u8(self.payload[230]).unwrap();
        wtr.write_u8(self.payload[231]).unwrap();
        wtr.write_u8(self.payload[232]).unwrap();
        wtr.write_u8(self.payload[233]).unwrap();
        wtr.write_u8(self.payload[234]).unwrap();
        wtr.write_u8(self.payload[235]).unwrap();
        wtr.write_u8(self.payload[236]).unwrap();
        wtr.write_u8(self.payload[237]).unwrap();
        wtr.write_u8(self.payload[238]).unwrap();
        wtr.write_u8(self.payload[239]).unwrap();
        wtr.write_u8(self.payload[240]).unwrap();
        wtr.write_u8(self.payload[241]).unwrap();
        wtr.write_u8(self.payload[242]).unwrap();
        wtr.write_u8(self.payload[243]).unwrap();
        wtr.write_u8(self.payload[244]).unwrap();
        wtr.write_u8(self.payload[245]).unwrap();
        wtr.write_u8(self.payload[246]).unwrap();
        wtr.write_u8(self.payload[247]).unwrap();
        wtr.write_u8(self.payload[248]).unwrap();
        wtr.write_u8(self.payload[249]).unwrap();
        wtr.write_u8(self.payload[250]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct TIMESYNC_DATA {
    pub tc1: i64,
    pub ts1: i64,
}

impl Parsable for TIMESYNC_DATA {
    fn parse(payload: &[u8]) -> TIMESYNC_DATA {
        let mut cur = Cursor::new(payload);
        TIMESYNC_DATA {
            tc1: cur.read_i64::<LittleEndian>().unwrap(),
            ts1: cur.read_i64::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i64::<LittleEndian>(self.tc1).unwrap();
        wtr.write_i64::<LittleEndian>(self.ts1).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct CAMERA_TRIGGER_DATA {
    pub time_usec: u64,
    pub seq: u32,
}

impl Parsable for CAMERA_TRIGGER_DATA {
    fn parse(payload: &[u8]) -> CAMERA_TRIGGER_DATA {
        let mut cur = Cursor::new(payload);
        CAMERA_TRIGGER_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            seq: cur.read_u32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_u32::<LittleEndian>(self.seq).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HIL_GPS_DATA {
    pub time_usec: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub eph: u16,
    pub epv: u16,
    pub vel: u16,
    pub vn: i16,
    pub ve: i16,
    pub vd: i16,
    pub cog: u16,
    pub fix_type: u8,
    pub satellites_visible: u8,
}

impl Parsable for HIL_GPS_DATA {
    fn parse(payload: &[u8]) -> HIL_GPS_DATA {
        let mut cur = Cursor::new(payload);
        HIL_GPS_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_i32::<LittleEndian>().unwrap(),
            eph: cur.read_u16::<LittleEndian>().unwrap(),
            epv: cur.read_u16::<LittleEndian>().unwrap(),
            vel: cur.read_u16::<LittleEndian>().unwrap(),
            vn: cur.read_i16::<LittleEndian>().unwrap(),
            ve: cur.read_i16::<LittleEndian>().unwrap(),
            vd: cur.read_i16::<LittleEndian>().unwrap(),
            cog: cur.read_u16::<LittleEndian>().unwrap(),
            fix_type: cur.read_u8().unwrap(),
            satellites_visible: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_i32::<LittleEndian>(self.alt).unwrap();
        wtr.write_u16::<LittleEndian>(self.eph).unwrap();
        wtr.write_u16::<LittleEndian>(self.epv).unwrap();
        wtr.write_u16::<LittleEndian>(self.vel).unwrap();
        wtr.write_i16::<LittleEndian>(self.vn).unwrap();
        wtr.write_i16::<LittleEndian>(self.ve).unwrap();
        wtr.write_i16::<LittleEndian>(self.vd).unwrap();
        wtr.write_u16::<LittleEndian>(self.cog).unwrap();
        wtr.write_u8(self.fix_type).unwrap();
        wtr.write_u8(self.satellites_visible).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HIL_OPTICAL_FLOW_DATA {
    pub time_usec: u64,
    pub integration_time_us: u32,
    pub integrated_x: f32,
    pub integrated_y: f32,
    pub integrated_xgyro: f32,
    pub integrated_ygyro: f32,
    pub integrated_zgyro: f32,
    pub time_delta_distance_us: u32,
    pub distance: f32,
    pub temperature: i16,
    pub sensor_id: u8,
    pub quality: u8,
}

impl Parsable for HIL_OPTICAL_FLOW_DATA {
    fn parse(payload: &[u8]) -> HIL_OPTICAL_FLOW_DATA {
        let mut cur = Cursor::new(payload);
        HIL_OPTICAL_FLOW_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            integration_time_us: cur.read_u32::<LittleEndian>().unwrap(),
            integrated_x: cur.read_f32::<LittleEndian>().unwrap(),
            integrated_y: cur.read_f32::<LittleEndian>().unwrap(),
            integrated_xgyro: cur.read_f32::<LittleEndian>().unwrap(),
            integrated_ygyro: cur.read_f32::<LittleEndian>().unwrap(),
            integrated_zgyro: cur.read_f32::<LittleEndian>().unwrap(),
            time_delta_distance_us: cur.read_u32::<LittleEndian>().unwrap(),
            distance: cur.read_f32::<LittleEndian>().unwrap(),
            temperature: cur.read_i16::<LittleEndian>().unwrap(),
            sensor_id: cur.read_u8().unwrap(),
            quality: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_u32::<LittleEndian>(self.integration_time_us).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_x).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_y).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_xgyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_ygyro).unwrap();
        wtr.write_f32::<LittleEndian>(self.integrated_zgyro).unwrap();
        wtr.write_u32::<LittleEndian>(self.time_delta_distance_us).unwrap();
        wtr.write_f32::<LittleEndian>(self.distance).unwrap();
        wtr.write_i16::<LittleEndian>(self.temperature).unwrap();
        wtr.write_u8(self.sensor_id).unwrap();
        wtr.write_u8(self.quality).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HIL_STATE_QUATERNION_DATA {
    pub time_usec: u64,
    pub attitude_quaternion: Vec<f32> /* 4 */,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub ind_airspeed: u16,
    pub true_airspeed: u16,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
}

impl Parsable for HIL_STATE_QUATERNION_DATA {
    fn parse(payload: &[u8]) -> HIL_STATE_QUATERNION_DATA {
        let mut cur = Cursor::new(payload);
        HIL_STATE_QUATERNION_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            attitude_quaternion: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            rollspeed: cur.read_f32::<LittleEndian>().unwrap(),
            pitchspeed: cur.read_f32::<LittleEndian>().unwrap(),
            yawspeed: cur.read_f32::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_i32::<LittleEndian>().unwrap(),
            vx: cur.read_i16::<LittleEndian>().unwrap(),
            vy: cur.read_i16::<LittleEndian>().unwrap(),
            vz: cur.read_i16::<LittleEndian>().unwrap(),
            ind_airspeed: cur.read_u16::<LittleEndian>().unwrap(),
            true_airspeed: cur.read_u16::<LittleEndian>().unwrap(),
            xacc: cur.read_i16::<LittleEndian>().unwrap(),
            yacc: cur.read_i16::<LittleEndian>().unwrap(),
            zacc: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.attitude_quaternion[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.attitude_quaternion[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.attitude_quaternion[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.attitude_quaternion[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.rollspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitchspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.yawspeed).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_i32::<LittleEndian>(self.alt).unwrap();
        wtr.write_i16::<LittleEndian>(self.vx).unwrap();
        wtr.write_i16::<LittleEndian>(self.vy).unwrap();
        wtr.write_i16::<LittleEndian>(self.vz).unwrap();
        wtr.write_u16::<LittleEndian>(self.ind_airspeed).unwrap();
        wtr.write_u16::<LittleEndian>(self.true_airspeed).unwrap();
        wtr.write_i16::<LittleEndian>(self.xacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.yacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.zacc).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SCALED_IMU2_DATA {
    pub time_boot_ms: u32,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
    pub xgyro: i16,
    pub ygyro: i16,
    pub zgyro: i16,
    pub xmag: i16,
    pub ymag: i16,
    pub zmag: i16,
}

impl Parsable for SCALED_IMU2_DATA {
    fn parse(payload: &[u8]) -> SCALED_IMU2_DATA {
        let mut cur = Cursor::new(payload);
        SCALED_IMU2_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            xacc: cur.read_i16::<LittleEndian>().unwrap(),
            yacc: cur.read_i16::<LittleEndian>().unwrap(),
            zacc: cur.read_i16::<LittleEndian>().unwrap(),
            xgyro: cur.read_i16::<LittleEndian>().unwrap(),
            ygyro: cur.read_i16::<LittleEndian>().unwrap(),
            zgyro: cur.read_i16::<LittleEndian>().unwrap(),
            xmag: cur.read_i16::<LittleEndian>().unwrap(),
            ymag: cur.read_i16::<LittleEndian>().unwrap(),
            zmag: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i16::<LittleEndian>(self.xacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.yacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.zacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.xgyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.ygyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.zgyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.xmag).unwrap();
        wtr.write_i16::<LittleEndian>(self.ymag).unwrap();
        wtr.write_i16::<LittleEndian>(self.zmag).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOG_REQUEST_LIST_DATA {
    pub start: u16,
    pub end: u16,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for LOG_REQUEST_LIST_DATA {
    fn parse(payload: &[u8]) -> LOG_REQUEST_LIST_DATA {
        let mut cur = Cursor::new(payload);
        LOG_REQUEST_LIST_DATA {
            start: cur.read_u16::<LittleEndian>().unwrap(),
            end: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.start).unwrap();
        wtr.write_u16::<LittleEndian>(self.end).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOG_ENTRY_DATA {
    pub time_utc: u32,
    pub size: u32,
    pub id: u16,
    pub num_logs: u16,
    pub last_log_num: u16,
}

impl Parsable for LOG_ENTRY_DATA {
    fn parse(payload: &[u8]) -> LOG_ENTRY_DATA {
        let mut cur = Cursor::new(payload);
        LOG_ENTRY_DATA {
            time_utc: cur.read_u32::<LittleEndian>().unwrap(),
            size: cur.read_u32::<LittleEndian>().unwrap(),
            id: cur.read_u16::<LittleEndian>().unwrap(),
            num_logs: cur.read_u16::<LittleEndian>().unwrap(),
            last_log_num: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_utc).unwrap();
        wtr.write_u32::<LittleEndian>(self.size).unwrap();
        wtr.write_u16::<LittleEndian>(self.id).unwrap();
        wtr.write_u16::<LittleEndian>(self.num_logs).unwrap();
        wtr.write_u16::<LittleEndian>(self.last_log_num).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOG_REQUEST_DATA_DATA {
    pub ofs: u32,
    pub count: u32,
    pub id: u16,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for LOG_REQUEST_DATA_DATA {
    fn parse(payload: &[u8]) -> LOG_REQUEST_DATA_DATA {
        let mut cur = Cursor::new(payload);
        LOG_REQUEST_DATA_DATA {
            ofs: cur.read_u32::<LittleEndian>().unwrap(),
            count: cur.read_u32::<LittleEndian>().unwrap(),
            id: cur.read_u16::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.ofs).unwrap();
        wtr.write_u32::<LittleEndian>(self.count).unwrap();
        wtr.write_u16::<LittleEndian>(self.id).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOG_DATA_DATA {
    pub ofs: u32,
    pub id: u16,
    pub count: u8,
    pub data: Vec<u8> /* 90 */,
}

impl Parsable for LOG_DATA_DATA {
    fn parse(payload: &[u8]) -> LOG_DATA_DATA {
        let mut cur = Cursor::new(payload);
        LOG_DATA_DATA {
            ofs: cur.read_u32::<LittleEndian>().unwrap(),
            id: cur.read_u16::<LittleEndian>().unwrap(),
            count: cur.read_u8().unwrap(),
            data: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.ofs).unwrap();
        wtr.write_u16::<LittleEndian>(self.id).unwrap();
        wtr.write_u8(self.count).unwrap();
        wtr.write_u8(self.data[0]).unwrap();
        wtr.write_u8(self.data[1]).unwrap();
        wtr.write_u8(self.data[2]).unwrap();
        wtr.write_u8(self.data[3]).unwrap();
        wtr.write_u8(self.data[4]).unwrap();
        wtr.write_u8(self.data[5]).unwrap();
        wtr.write_u8(self.data[6]).unwrap();
        wtr.write_u8(self.data[7]).unwrap();
        wtr.write_u8(self.data[8]).unwrap();
        wtr.write_u8(self.data[9]).unwrap();
        wtr.write_u8(self.data[10]).unwrap();
        wtr.write_u8(self.data[11]).unwrap();
        wtr.write_u8(self.data[12]).unwrap();
        wtr.write_u8(self.data[13]).unwrap();
        wtr.write_u8(self.data[14]).unwrap();
        wtr.write_u8(self.data[15]).unwrap();
        wtr.write_u8(self.data[16]).unwrap();
        wtr.write_u8(self.data[17]).unwrap();
        wtr.write_u8(self.data[18]).unwrap();
        wtr.write_u8(self.data[19]).unwrap();
        wtr.write_u8(self.data[20]).unwrap();
        wtr.write_u8(self.data[21]).unwrap();
        wtr.write_u8(self.data[22]).unwrap();
        wtr.write_u8(self.data[23]).unwrap();
        wtr.write_u8(self.data[24]).unwrap();
        wtr.write_u8(self.data[25]).unwrap();
        wtr.write_u8(self.data[26]).unwrap();
        wtr.write_u8(self.data[27]).unwrap();
        wtr.write_u8(self.data[28]).unwrap();
        wtr.write_u8(self.data[29]).unwrap();
        wtr.write_u8(self.data[30]).unwrap();
        wtr.write_u8(self.data[31]).unwrap();
        wtr.write_u8(self.data[32]).unwrap();
        wtr.write_u8(self.data[33]).unwrap();
        wtr.write_u8(self.data[34]).unwrap();
        wtr.write_u8(self.data[35]).unwrap();
        wtr.write_u8(self.data[36]).unwrap();
        wtr.write_u8(self.data[37]).unwrap();
        wtr.write_u8(self.data[38]).unwrap();
        wtr.write_u8(self.data[39]).unwrap();
        wtr.write_u8(self.data[40]).unwrap();
        wtr.write_u8(self.data[41]).unwrap();
        wtr.write_u8(self.data[42]).unwrap();
        wtr.write_u8(self.data[43]).unwrap();
        wtr.write_u8(self.data[44]).unwrap();
        wtr.write_u8(self.data[45]).unwrap();
        wtr.write_u8(self.data[46]).unwrap();
        wtr.write_u8(self.data[47]).unwrap();
        wtr.write_u8(self.data[48]).unwrap();
        wtr.write_u8(self.data[49]).unwrap();
        wtr.write_u8(self.data[50]).unwrap();
        wtr.write_u8(self.data[51]).unwrap();
        wtr.write_u8(self.data[52]).unwrap();
        wtr.write_u8(self.data[53]).unwrap();
        wtr.write_u8(self.data[54]).unwrap();
        wtr.write_u8(self.data[55]).unwrap();
        wtr.write_u8(self.data[56]).unwrap();
        wtr.write_u8(self.data[57]).unwrap();
        wtr.write_u8(self.data[58]).unwrap();
        wtr.write_u8(self.data[59]).unwrap();
        wtr.write_u8(self.data[60]).unwrap();
        wtr.write_u8(self.data[61]).unwrap();
        wtr.write_u8(self.data[62]).unwrap();
        wtr.write_u8(self.data[63]).unwrap();
        wtr.write_u8(self.data[64]).unwrap();
        wtr.write_u8(self.data[65]).unwrap();
        wtr.write_u8(self.data[66]).unwrap();
        wtr.write_u8(self.data[67]).unwrap();
        wtr.write_u8(self.data[68]).unwrap();
        wtr.write_u8(self.data[69]).unwrap();
        wtr.write_u8(self.data[70]).unwrap();
        wtr.write_u8(self.data[71]).unwrap();
        wtr.write_u8(self.data[72]).unwrap();
        wtr.write_u8(self.data[73]).unwrap();
        wtr.write_u8(self.data[74]).unwrap();
        wtr.write_u8(self.data[75]).unwrap();
        wtr.write_u8(self.data[76]).unwrap();
        wtr.write_u8(self.data[77]).unwrap();
        wtr.write_u8(self.data[78]).unwrap();
        wtr.write_u8(self.data[79]).unwrap();
        wtr.write_u8(self.data[80]).unwrap();
        wtr.write_u8(self.data[81]).unwrap();
        wtr.write_u8(self.data[82]).unwrap();
        wtr.write_u8(self.data[83]).unwrap();
        wtr.write_u8(self.data[84]).unwrap();
        wtr.write_u8(self.data[85]).unwrap();
        wtr.write_u8(self.data[86]).unwrap();
        wtr.write_u8(self.data[87]).unwrap();
        wtr.write_u8(self.data[88]).unwrap();
        wtr.write_u8(self.data[89]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOG_ERASE_DATA {
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for LOG_ERASE_DATA {
    fn parse(payload: &[u8]) -> LOG_ERASE_DATA {
        let mut cur = Cursor::new(payload);
        LOG_ERASE_DATA {
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LOG_REQUEST_END_DATA {
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for LOG_REQUEST_END_DATA {
    fn parse(payload: &[u8]) -> LOG_REQUEST_END_DATA {
        let mut cur = Cursor::new(payload);
        LOG_REQUEST_END_DATA {
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GPS_INJECT_DATA_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub len: u8,
    pub data: Vec<u8> /* 110 */,
}

impl Parsable for GPS_INJECT_DATA_DATA {
    fn parse(payload: &[u8]) -> GPS_INJECT_DATA_DATA {
        let mut cur = Cursor::new(payload);
        GPS_INJECT_DATA_DATA {
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            len: cur.read_u8().unwrap(),
            data: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.len).unwrap();
        wtr.write_u8(self.data[0]).unwrap();
        wtr.write_u8(self.data[1]).unwrap();
        wtr.write_u8(self.data[2]).unwrap();
        wtr.write_u8(self.data[3]).unwrap();
        wtr.write_u8(self.data[4]).unwrap();
        wtr.write_u8(self.data[5]).unwrap();
        wtr.write_u8(self.data[6]).unwrap();
        wtr.write_u8(self.data[7]).unwrap();
        wtr.write_u8(self.data[8]).unwrap();
        wtr.write_u8(self.data[9]).unwrap();
        wtr.write_u8(self.data[10]).unwrap();
        wtr.write_u8(self.data[11]).unwrap();
        wtr.write_u8(self.data[12]).unwrap();
        wtr.write_u8(self.data[13]).unwrap();
        wtr.write_u8(self.data[14]).unwrap();
        wtr.write_u8(self.data[15]).unwrap();
        wtr.write_u8(self.data[16]).unwrap();
        wtr.write_u8(self.data[17]).unwrap();
        wtr.write_u8(self.data[18]).unwrap();
        wtr.write_u8(self.data[19]).unwrap();
        wtr.write_u8(self.data[20]).unwrap();
        wtr.write_u8(self.data[21]).unwrap();
        wtr.write_u8(self.data[22]).unwrap();
        wtr.write_u8(self.data[23]).unwrap();
        wtr.write_u8(self.data[24]).unwrap();
        wtr.write_u8(self.data[25]).unwrap();
        wtr.write_u8(self.data[26]).unwrap();
        wtr.write_u8(self.data[27]).unwrap();
        wtr.write_u8(self.data[28]).unwrap();
        wtr.write_u8(self.data[29]).unwrap();
        wtr.write_u8(self.data[30]).unwrap();
        wtr.write_u8(self.data[31]).unwrap();
        wtr.write_u8(self.data[32]).unwrap();
        wtr.write_u8(self.data[33]).unwrap();
        wtr.write_u8(self.data[34]).unwrap();
        wtr.write_u8(self.data[35]).unwrap();
        wtr.write_u8(self.data[36]).unwrap();
        wtr.write_u8(self.data[37]).unwrap();
        wtr.write_u8(self.data[38]).unwrap();
        wtr.write_u8(self.data[39]).unwrap();
        wtr.write_u8(self.data[40]).unwrap();
        wtr.write_u8(self.data[41]).unwrap();
        wtr.write_u8(self.data[42]).unwrap();
        wtr.write_u8(self.data[43]).unwrap();
        wtr.write_u8(self.data[44]).unwrap();
        wtr.write_u8(self.data[45]).unwrap();
        wtr.write_u8(self.data[46]).unwrap();
        wtr.write_u8(self.data[47]).unwrap();
        wtr.write_u8(self.data[48]).unwrap();
        wtr.write_u8(self.data[49]).unwrap();
        wtr.write_u8(self.data[50]).unwrap();
        wtr.write_u8(self.data[51]).unwrap();
        wtr.write_u8(self.data[52]).unwrap();
        wtr.write_u8(self.data[53]).unwrap();
        wtr.write_u8(self.data[54]).unwrap();
        wtr.write_u8(self.data[55]).unwrap();
        wtr.write_u8(self.data[56]).unwrap();
        wtr.write_u8(self.data[57]).unwrap();
        wtr.write_u8(self.data[58]).unwrap();
        wtr.write_u8(self.data[59]).unwrap();
        wtr.write_u8(self.data[60]).unwrap();
        wtr.write_u8(self.data[61]).unwrap();
        wtr.write_u8(self.data[62]).unwrap();
        wtr.write_u8(self.data[63]).unwrap();
        wtr.write_u8(self.data[64]).unwrap();
        wtr.write_u8(self.data[65]).unwrap();
        wtr.write_u8(self.data[66]).unwrap();
        wtr.write_u8(self.data[67]).unwrap();
        wtr.write_u8(self.data[68]).unwrap();
        wtr.write_u8(self.data[69]).unwrap();
        wtr.write_u8(self.data[70]).unwrap();
        wtr.write_u8(self.data[71]).unwrap();
        wtr.write_u8(self.data[72]).unwrap();
        wtr.write_u8(self.data[73]).unwrap();
        wtr.write_u8(self.data[74]).unwrap();
        wtr.write_u8(self.data[75]).unwrap();
        wtr.write_u8(self.data[76]).unwrap();
        wtr.write_u8(self.data[77]).unwrap();
        wtr.write_u8(self.data[78]).unwrap();
        wtr.write_u8(self.data[79]).unwrap();
        wtr.write_u8(self.data[80]).unwrap();
        wtr.write_u8(self.data[81]).unwrap();
        wtr.write_u8(self.data[82]).unwrap();
        wtr.write_u8(self.data[83]).unwrap();
        wtr.write_u8(self.data[84]).unwrap();
        wtr.write_u8(self.data[85]).unwrap();
        wtr.write_u8(self.data[86]).unwrap();
        wtr.write_u8(self.data[87]).unwrap();
        wtr.write_u8(self.data[88]).unwrap();
        wtr.write_u8(self.data[89]).unwrap();
        wtr.write_u8(self.data[90]).unwrap();
        wtr.write_u8(self.data[91]).unwrap();
        wtr.write_u8(self.data[92]).unwrap();
        wtr.write_u8(self.data[93]).unwrap();
        wtr.write_u8(self.data[94]).unwrap();
        wtr.write_u8(self.data[95]).unwrap();
        wtr.write_u8(self.data[96]).unwrap();
        wtr.write_u8(self.data[97]).unwrap();
        wtr.write_u8(self.data[98]).unwrap();
        wtr.write_u8(self.data[99]).unwrap();
        wtr.write_u8(self.data[100]).unwrap();
        wtr.write_u8(self.data[101]).unwrap();
        wtr.write_u8(self.data[102]).unwrap();
        wtr.write_u8(self.data[103]).unwrap();
        wtr.write_u8(self.data[104]).unwrap();
        wtr.write_u8(self.data[105]).unwrap();
        wtr.write_u8(self.data[106]).unwrap();
        wtr.write_u8(self.data[107]).unwrap();
        wtr.write_u8(self.data[108]).unwrap();
        wtr.write_u8(self.data[109]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GPS2_RAW_DATA {
    pub time_usec: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub dgps_age: u32,
    pub eph: u16,
    pub epv: u16,
    pub vel: u16,
    pub cog: u16,
    pub fix_type: u8,
    pub satellites_visible: u8,
    pub dgps_numch: u8,
}

impl Parsable for GPS2_RAW_DATA {
    fn parse(payload: &[u8]) -> GPS2_RAW_DATA {
        let mut cur = Cursor::new(payload);
        GPS2_RAW_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            alt: cur.read_i32::<LittleEndian>().unwrap(),
            dgps_age: cur.read_u32::<LittleEndian>().unwrap(),
            eph: cur.read_u16::<LittleEndian>().unwrap(),
            epv: cur.read_u16::<LittleEndian>().unwrap(),
            vel: cur.read_u16::<LittleEndian>().unwrap(),
            cog: cur.read_u16::<LittleEndian>().unwrap(),
            fix_type: cur.read_u8().unwrap(),
            satellites_visible: cur.read_u8().unwrap(),
            dgps_numch: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_i32::<LittleEndian>(self.alt).unwrap();
        wtr.write_u32::<LittleEndian>(self.dgps_age).unwrap();
        wtr.write_u16::<LittleEndian>(self.eph).unwrap();
        wtr.write_u16::<LittleEndian>(self.epv).unwrap();
        wtr.write_u16::<LittleEndian>(self.vel).unwrap();
        wtr.write_u16::<LittleEndian>(self.cog).unwrap();
        wtr.write_u8(self.fix_type).unwrap();
        wtr.write_u8(self.satellites_visible).unwrap();
        wtr.write_u8(self.dgps_numch).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct POWER_STATUS_DATA {
    pub Vcc: u16,
    pub Vservo: u16,
    pub flags: u16,
}

impl Parsable for POWER_STATUS_DATA {
    fn parse(payload: &[u8]) -> POWER_STATUS_DATA {
        let mut cur = Cursor::new(payload);
        POWER_STATUS_DATA {
            Vcc: cur.read_u16::<LittleEndian>().unwrap(),
            Vservo: cur.read_u16::<LittleEndian>().unwrap(),
            flags: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.Vcc).unwrap();
        wtr.write_u16::<LittleEndian>(self.Vservo).unwrap();
        wtr.write_u16::<LittleEndian>(self.flags).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SERIAL_CONTROL_DATA {
    pub baudrate: u32,
    pub timeout: u16,
    pub device: u8,
    pub flags: u8,
    pub count: u8,
    pub data: Vec<u8> /* 70 */,
}

impl Parsable for SERIAL_CONTROL_DATA {
    fn parse(payload: &[u8]) -> SERIAL_CONTROL_DATA {
        let mut cur = Cursor::new(payload);
        SERIAL_CONTROL_DATA {
            baudrate: cur.read_u32::<LittleEndian>().unwrap(),
            timeout: cur.read_u16::<LittleEndian>().unwrap(),
            device: cur.read_u8().unwrap(),
            flags: cur.read_u8().unwrap(),
            count: cur.read_u8().unwrap(),
            data: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.baudrate).unwrap();
        wtr.write_u16::<LittleEndian>(self.timeout).unwrap();
        wtr.write_u8(self.device).unwrap();
        wtr.write_u8(self.flags).unwrap();
        wtr.write_u8(self.count).unwrap();
        wtr.write_u8(self.data[0]).unwrap();
        wtr.write_u8(self.data[1]).unwrap();
        wtr.write_u8(self.data[2]).unwrap();
        wtr.write_u8(self.data[3]).unwrap();
        wtr.write_u8(self.data[4]).unwrap();
        wtr.write_u8(self.data[5]).unwrap();
        wtr.write_u8(self.data[6]).unwrap();
        wtr.write_u8(self.data[7]).unwrap();
        wtr.write_u8(self.data[8]).unwrap();
        wtr.write_u8(self.data[9]).unwrap();
        wtr.write_u8(self.data[10]).unwrap();
        wtr.write_u8(self.data[11]).unwrap();
        wtr.write_u8(self.data[12]).unwrap();
        wtr.write_u8(self.data[13]).unwrap();
        wtr.write_u8(self.data[14]).unwrap();
        wtr.write_u8(self.data[15]).unwrap();
        wtr.write_u8(self.data[16]).unwrap();
        wtr.write_u8(self.data[17]).unwrap();
        wtr.write_u8(self.data[18]).unwrap();
        wtr.write_u8(self.data[19]).unwrap();
        wtr.write_u8(self.data[20]).unwrap();
        wtr.write_u8(self.data[21]).unwrap();
        wtr.write_u8(self.data[22]).unwrap();
        wtr.write_u8(self.data[23]).unwrap();
        wtr.write_u8(self.data[24]).unwrap();
        wtr.write_u8(self.data[25]).unwrap();
        wtr.write_u8(self.data[26]).unwrap();
        wtr.write_u8(self.data[27]).unwrap();
        wtr.write_u8(self.data[28]).unwrap();
        wtr.write_u8(self.data[29]).unwrap();
        wtr.write_u8(self.data[30]).unwrap();
        wtr.write_u8(self.data[31]).unwrap();
        wtr.write_u8(self.data[32]).unwrap();
        wtr.write_u8(self.data[33]).unwrap();
        wtr.write_u8(self.data[34]).unwrap();
        wtr.write_u8(self.data[35]).unwrap();
        wtr.write_u8(self.data[36]).unwrap();
        wtr.write_u8(self.data[37]).unwrap();
        wtr.write_u8(self.data[38]).unwrap();
        wtr.write_u8(self.data[39]).unwrap();
        wtr.write_u8(self.data[40]).unwrap();
        wtr.write_u8(self.data[41]).unwrap();
        wtr.write_u8(self.data[42]).unwrap();
        wtr.write_u8(self.data[43]).unwrap();
        wtr.write_u8(self.data[44]).unwrap();
        wtr.write_u8(self.data[45]).unwrap();
        wtr.write_u8(self.data[46]).unwrap();
        wtr.write_u8(self.data[47]).unwrap();
        wtr.write_u8(self.data[48]).unwrap();
        wtr.write_u8(self.data[49]).unwrap();
        wtr.write_u8(self.data[50]).unwrap();
        wtr.write_u8(self.data[51]).unwrap();
        wtr.write_u8(self.data[52]).unwrap();
        wtr.write_u8(self.data[53]).unwrap();
        wtr.write_u8(self.data[54]).unwrap();
        wtr.write_u8(self.data[55]).unwrap();
        wtr.write_u8(self.data[56]).unwrap();
        wtr.write_u8(self.data[57]).unwrap();
        wtr.write_u8(self.data[58]).unwrap();
        wtr.write_u8(self.data[59]).unwrap();
        wtr.write_u8(self.data[60]).unwrap();
        wtr.write_u8(self.data[61]).unwrap();
        wtr.write_u8(self.data[62]).unwrap();
        wtr.write_u8(self.data[63]).unwrap();
        wtr.write_u8(self.data[64]).unwrap();
        wtr.write_u8(self.data[65]).unwrap();
        wtr.write_u8(self.data[66]).unwrap();
        wtr.write_u8(self.data[67]).unwrap();
        wtr.write_u8(self.data[68]).unwrap();
        wtr.write_u8(self.data[69]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GPS_RTK_DATA {
    pub time_last_baseline_ms: u32,
    pub tow: u32,
    pub baseline_a_mm: i32,
    pub baseline_b_mm: i32,
    pub baseline_c_mm: i32,
    pub accuracy: u32,
    pub iar_num_hypotheses: i32,
    pub wn: u16,
    pub rtk_receiver_id: u8,
    pub rtk_health: u8,
    pub rtk_rate: u8,
    pub nsats: u8,
    pub baseline_coords_type: u8,
}

impl Parsable for GPS_RTK_DATA {
    fn parse(payload: &[u8]) -> GPS_RTK_DATA {
        let mut cur = Cursor::new(payload);
        GPS_RTK_DATA {
            time_last_baseline_ms: cur.read_u32::<LittleEndian>().unwrap(),
            tow: cur.read_u32::<LittleEndian>().unwrap(),
            baseline_a_mm: cur.read_i32::<LittleEndian>().unwrap(),
            baseline_b_mm: cur.read_i32::<LittleEndian>().unwrap(),
            baseline_c_mm: cur.read_i32::<LittleEndian>().unwrap(),
            accuracy: cur.read_u32::<LittleEndian>().unwrap(),
            iar_num_hypotheses: cur.read_i32::<LittleEndian>().unwrap(),
            wn: cur.read_u16::<LittleEndian>().unwrap(),
            rtk_receiver_id: cur.read_u8().unwrap(),
            rtk_health: cur.read_u8().unwrap(),
            rtk_rate: cur.read_u8().unwrap(),
            nsats: cur.read_u8().unwrap(),
            baseline_coords_type: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_last_baseline_ms).unwrap();
        wtr.write_u32::<LittleEndian>(self.tow).unwrap();
        wtr.write_i32::<LittleEndian>(self.baseline_a_mm).unwrap();
        wtr.write_i32::<LittleEndian>(self.baseline_b_mm).unwrap();
        wtr.write_i32::<LittleEndian>(self.baseline_c_mm).unwrap();
        wtr.write_u32::<LittleEndian>(self.accuracy).unwrap();
        wtr.write_i32::<LittleEndian>(self.iar_num_hypotheses).unwrap();
        wtr.write_u16::<LittleEndian>(self.wn).unwrap();
        wtr.write_u8(self.rtk_receiver_id).unwrap();
        wtr.write_u8(self.rtk_health).unwrap();
        wtr.write_u8(self.rtk_rate).unwrap();
        wtr.write_u8(self.nsats).unwrap();
        wtr.write_u8(self.baseline_coords_type).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct GPS2_RTK_DATA {
    pub time_last_baseline_ms: u32,
    pub tow: u32,
    pub baseline_a_mm: i32,
    pub baseline_b_mm: i32,
    pub baseline_c_mm: i32,
    pub accuracy: u32,
    pub iar_num_hypotheses: i32,
    pub wn: u16,
    pub rtk_receiver_id: u8,
    pub rtk_health: u8,
    pub rtk_rate: u8,
    pub nsats: u8,
    pub baseline_coords_type: u8,
}

impl Parsable for GPS2_RTK_DATA {
    fn parse(payload: &[u8]) -> GPS2_RTK_DATA {
        let mut cur = Cursor::new(payload);
        GPS2_RTK_DATA {
            time_last_baseline_ms: cur.read_u32::<LittleEndian>().unwrap(),
            tow: cur.read_u32::<LittleEndian>().unwrap(),
            baseline_a_mm: cur.read_i32::<LittleEndian>().unwrap(),
            baseline_b_mm: cur.read_i32::<LittleEndian>().unwrap(),
            baseline_c_mm: cur.read_i32::<LittleEndian>().unwrap(),
            accuracy: cur.read_u32::<LittleEndian>().unwrap(),
            iar_num_hypotheses: cur.read_i32::<LittleEndian>().unwrap(),
            wn: cur.read_u16::<LittleEndian>().unwrap(),
            rtk_receiver_id: cur.read_u8().unwrap(),
            rtk_health: cur.read_u8().unwrap(),
            rtk_rate: cur.read_u8().unwrap(),
            nsats: cur.read_u8().unwrap(),
            baseline_coords_type: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_last_baseline_ms).unwrap();
        wtr.write_u32::<LittleEndian>(self.tow).unwrap();
        wtr.write_i32::<LittleEndian>(self.baseline_a_mm).unwrap();
        wtr.write_i32::<LittleEndian>(self.baseline_b_mm).unwrap();
        wtr.write_i32::<LittleEndian>(self.baseline_c_mm).unwrap();
        wtr.write_u32::<LittleEndian>(self.accuracy).unwrap();
        wtr.write_i32::<LittleEndian>(self.iar_num_hypotheses).unwrap();
        wtr.write_u16::<LittleEndian>(self.wn).unwrap();
        wtr.write_u8(self.rtk_receiver_id).unwrap();
        wtr.write_u8(self.rtk_health).unwrap();
        wtr.write_u8(self.rtk_rate).unwrap();
        wtr.write_u8(self.nsats).unwrap();
        wtr.write_u8(self.baseline_coords_type).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SCALED_IMU3_DATA {
    pub time_boot_ms: u32,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
    pub xgyro: i16,
    pub ygyro: i16,
    pub zgyro: i16,
    pub xmag: i16,
    pub ymag: i16,
    pub zmag: i16,
}

impl Parsable for SCALED_IMU3_DATA {
    fn parse(payload: &[u8]) -> SCALED_IMU3_DATA {
        let mut cur = Cursor::new(payload);
        SCALED_IMU3_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            xacc: cur.read_i16::<LittleEndian>().unwrap(),
            yacc: cur.read_i16::<LittleEndian>().unwrap(),
            zacc: cur.read_i16::<LittleEndian>().unwrap(),
            xgyro: cur.read_i16::<LittleEndian>().unwrap(),
            ygyro: cur.read_i16::<LittleEndian>().unwrap(),
            zgyro: cur.read_i16::<LittleEndian>().unwrap(),
            xmag: cur.read_i16::<LittleEndian>().unwrap(),
            ymag: cur.read_i16::<LittleEndian>().unwrap(),
            zmag: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i16::<LittleEndian>(self.xacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.yacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.zacc).unwrap();
        wtr.write_i16::<LittleEndian>(self.xgyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.ygyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.zgyro).unwrap();
        wtr.write_i16::<LittleEndian>(self.xmag).unwrap();
        wtr.write_i16::<LittleEndian>(self.ymag).unwrap();
        wtr.write_i16::<LittleEndian>(self.zmag).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct DATA_TRANSMISSION_HANDSHAKE_DATA {
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub packets: u16,
    pub mavtype: u8,
    pub payload: u8,
    pub jpg_quality: u8,
}

impl Parsable for DATA_TRANSMISSION_HANDSHAKE_DATA {
    fn parse(payload: &[u8]) -> DATA_TRANSMISSION_HANDSHAKE_DATA {
        let mut cur = Cursor::new(payload);
        DATA_TRANSMISSION_HANDSHAKE_DATA {
            size: cur.read_u32::<LittleEndian>().unwrap(),
            width: cur.read_u16::<LittleEndian>().unwrap(),
            height: cur.read_u16::<LittleEndian>().unwrap(),
            packets: cur.read_u16::<LittleEndian>().unwrap(),
            mavtype: cur.read_u8().unwrap(),
            payload: cur.read_u8().unwrap(),
            jpg_quality: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.size).unwrap();
        wtr.write_u16::<LittleEndian>(self.width).unwrap();
        wtr.write_u16::<LittleEndian>(self.height).unwrap();
        wtr.write_u16::<LittleEndian>(self.packets).unwrap();
        wtr.write_u8(self.mavtype).unwrap();
        wtr.write_u8(self.payload).unwrap();
        wtr.write_u8(self.jpg_quality).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ENCAPSULATED_DATA_DATA {
    pub seqnr: u16,
    pub data: Vec<u8> /* 253 */,
}

impl Parsable for ENCAPSULATED_DATA_DATA {
    fn parse(payload: &[u8]) -> ENCAPSULATED_DATA_DATA {
        let mut cur = Cursor::new(payload);
        ENCAPSULATED_DATA_DATA {
            seqnr: cur.read_u16::<LittleEndian>().unwrap(),
            data: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.seqnr).unwrap();
        wtr.write_u8(self.data[0]).unwrap();
        wtr.write_u8(self.data[1]).unwrap();
        wtr.write_u8(self.data[2]).unwrap();
        wtr.write_u8(self.data[3]).unwrap();
        wtr.write_u8(self.data[4]).unwrap();
        wtr.write_u8(self.data[5]).unwrap();
        wtr.write_u8(self.data[6]).unwrap();
        wtr.write_u8(self.data[7]).unwrap();
        wtr.write_u8(self.data[8]).unwrap();
        wtr.write_u8(self.data[9]).unwrap();
        wtr.write_u8(self.data[10]).unwrap();
        wtr.write_u8(self.data[11]).unwrap();
        wtr.write_u8(self.data[12]).unwrap();
        wtr.write_u8(self.data[13]).unwrap();
        wtr.write_u8(self.data[14]).unwrap();
        wtr.write_u8(self.data[15]).unwrap();
        wtr.write_u8(self.data[16]).unwrap();
        wtr.write_u8(self.data[17]).unwrap();
        wtr.write_u8(self.data[18]).unwrap();
        wtr.write_u8(self.data[19]).unwrap();
        wtr.write_u8(self.data[20]).unwrap();
        wtr.write_u8(self.data[21]).unwrap();
        wtr.write_u8(self.data[22]).unwrap();
        wtr.write_u8(self.data[23]).unwrap();
        wtr.write_u8(self.data[24]).unwrap();
        wtr.write_u8(self.data[25]).unwrap();
        wtr.write_u8(self.data[26]).unwrap();
        wtr.write_u8(self.data[27]).unwrap();
        wtr.write_u8(self.data[28]).unwrap();
        wtr.write_u8(self.data[29]).unwrap();
        wtr.write_u8(self.data[30]).unwrap();
        wtr.write_u8(self.data[31]).unwrap();
        wtr.write_u8(self.data[32]).unwrap();
        wtr.write_u8(self.data[33]).unwrap();
        wtr.write_u8(self.data[34]).unwrap();
        wtr.write_u8(self.data[35]).unwrap();
        wtr.write_u8(self.data[36]).unwrap();
        wtr.write_u8(self.data[37]).unwrap();
        wtr.write_u8(self.data[38]).unwrap();
        wtr.write_u8(self.data[39]).unwrap();
        wtr.write_u8(self.data[40]).unwrap();
        wtr.write_u8(self.data[41]).unwrap();
        wtr.write_u8(self.data[42]).unwrap();
        wtr.write_u8(self.data[43]).unwrap();
        wtr.write_u8(self.data[44]).unwrap();
        wtr.write_u8(self.data[45]).unwrap();
        wtr.write_u8(self.data[46]).unwrap();
        wtr.write_u8(self.data[47]).unwrap();
        wtr.write_u8(self.data[48]).unwrap();
        wtr.write_u8(self.data[49]).unwrap();
        wtr.write_u8(self.data[50]).unwrap();
        wtr.write_u8(self.data[51]).unwrap();
        wtr.write_u8(self.data[52]).unwrap();
        wtr.write_u8(self.data[53]).unwrap();
        wtr.write_u8(self.data[54]).unwrap();
        wtr.write_u8(self.data[55]).unwrap();
        wtr.write_u8(self.data[56]).unwrap();
        wtr.write_u8(self.data[57]).unwrap();
        wtr.write_u8(self.data[58]).unwrap();
        wtr.write_u8(self.data[59]).unwrap();
        wtr.write_u8(self.data[60]).unwrap();
        wtr.write_u8(self.data[61]).unwrap();
        wtr.write_u8(self.data[62]).unwrap();
        wtr.write_u8(self.data[63]).unwrap();
        wtr.write_u8(self.data[64]).unwrap();
        wtr.write_u8(self.data[65]).unwrap();
        wtr.write_u8(self.data[66]).unwrap();
        wtr.write_u8(self.data[67]).unwrap();
        wtr.write_u8(self.data[68]).unwrap();
        wtr.write_u8(self.data[69]).unwrap();
        wtr.write_u8(self.data[70]).unwrap();
        wtr.write_u8(self.data[71]).unwrap();
        wtr.write_u8(self.data[72]).unwrap();
        wtr.write_u8(self.data[73]).unwrap();
        wtr.write_u8(self.data[74]).unwrap();
        wtr.write_u8(self.data[75]).unwrap();
        wtr.write_u8(self.data[76]).unwrap();
        wtr.write_u8(self.data[77]).unwrap();
        wtr.write_u8(self.data[78]).unwrap();
        wtr.write_u8(self.data[79]).unwrap();
        wtr.write_u8(self.data[80]).unwrap();
        wtr.write_u8(self.data[81]).unwrap();
        wtr.write_u8(self.data[82]).unwrap();
        wtr.write_u8(self.data[83]).unwrap();
        wtr.write_u8(self.data[84]).unwrap();
        wtr.write_u8(self.data[85]).unwrap();
        wtr.write_u8(self.data[86]).unwrap();
        wtr.write_u8(self.data[87]).unwrap();
        wtr.write_u8(self.data[88]).unwrap();
        wtr.write_u8(self.data[89]).unwrap();
        wtr.write_u8(self.data[90]).unwrap();
        wtr.write_u8(self.data[91]).unwrap();
        wtr.write_u8(self.data[92]).unwrap();
        wtr.write_u8(self.data[93]).unwrap();
        wtr.write_u8(self.data[94]).unwrap();
        wtr.write_u8(self.data[95]).unwrap();
        wtr.write_u8(self.data[96]).unwrap();
        wtr.write_u8(self.data[97]).unwrap();
        wtr.write_u8(self.data[98]).unwrap();
        wtr.write_u8(self.data[99]).unwrap();
        wtr.write_u8(self.data[100]).unwrap();
        wtr.write_u8(self.data[101]).unwrap();
        wtr.write_u8(self.data[102]).unwrap();
        wtr.write_u8(self.data[103]).unwrap();
        wtr.write_u8(self.data[104]).unwrap();
        wtr.write_u8(self.data[105]).unwrap();
        wtr.write_u8(self.data[106]).unwrap();
        wtr.write_u8(self.data[107]).unwrap();
        wtr.write_u8(self.data[108]).unwrap();
        wtr.write_u8(self.data[109]).unwrap();
        wtr.write_u8(self.data[110]).unwrap();
        wtr.write_u8(self.data[111]).unwrap();
        wtr.write_u8(self.data[112]).unwrap();
        wtr.write_u8(self.data[113]).unwrap();
        wtr.write_u8(self.data[114]).unwrap();
        wtr.write_u8(self.data[115]).unwrap();
        wtr.write_u8(self.data[116]).unwrap();
        wtr.write_u8(self.data[117]).unwrap();
        wtr.write_u8(self.data[118]).unwrap();
        wtr.write_u8(self.data[119]).unwrap();
        wtr.write_u8(self.data[120]).unwrap();
        wtr.write_u8(self.data[121]).unwrap();
        wtr.write_u8(self.data[122]).unwrap();
        wtr.write_u8(self.data[123]).unwrap();
        wtr.write_u8(self.data[124]).unwrap();
        wtr.write_u8(self.data[125]).unwrap();
        wtr.write_u8(self.data[126]).unwrap();
        wtr.write_u8(self.data[127]).unwrap();
        wtr.write_u8(self.data[128]).unwrap();
        wtr.write_u8(self.data[129]).unwrap();
        wtr.write_u8(self.data[130]).unwrap();
        wtr.write_u8(self.data[131]).unwrap();
        wtr.write_u8(self.data[132]).unwrap();
        wtr.write_u8(self.data[133]).unwrap();
        wtr.write_u8(self.data[134]).unwrap();
        wtr.write_u8(self.data[135]).unwrap();
        wtr.write_u8(self.data[136]).unwrap();
        wtr.write_u8(self.data[137]).unwrap();
        wtr.write_u8(self.data[138]).unwrap();
        wtr.write_u8(self.data[139]).unwrap();
        wtr.write_u8(self.data[140]).unwrap();
        wtr.write_u8(self.data[141]).unwrap();
        wtr.write_u8(self.data[142]).unwrap();
        wtr.write_u8(self.data[143]).unwrap();
        wtr.write_u8(self.data[144]).unwrap();
        wtr.write_u8(self.data[145]).unwrap();
        wtr.write_u8(self.data[146]).unwrap();
        wtr.write_u8(self.data[147]).unwrap();
        wtr.write_u8(self.data[148]).unwrap();
        wtr.write_u8(self.data[149]).unwrap();
        wtr.write_u8(self.data[150]).unwrap();
        wtr.write_u8(self.data[151]).unwrap();
        wtr.write_u8(self.data[152]).unwrap();
        wtr.write_u8(self.data[153]).unwrap();
        wtr.write_u8(self.data[154]).unwrap();
        wtr.write_u8(self.data[155]).unwrap();
        wtr.write_u8(self.data[156]).unwrap();
        wtr.write_u8(self.data[157]).unwrap();
        wtr.write_u8(self.data[158]).unwrap();
        wtr.write_u8(self.data[159]).unwrap();
        wtr.write_u8(self.data[160]).unwrap();
        wtr.write_u8(self.data[161]).unwrap();
        wtr.write_u8(self.data[162]).unwrap();
        wtr.write_u8(self.data[163]).unwrap();
        wtr.write_u8(self.data[164]).unwrap();
        wtr.write_u8(self.data[165]).unwrap();
        wtr.write_u8(self.data[166]).unwrap();
        wtr.write_u8(self.data[167]).unwrap();
        wtr.write_u8(self.data[168]).unwrap();
        wtr.write_u8(self.data[169]).unwrap();
        wtr.write_u8(self.data[170]).unwrap();
        wtr.write_u8(self.data[171]).unwrap();
        wtr.write_u8(self.data[172]).unwrap();
        wtr.write_u8(self.data[173]).unwrap();
        wtr.write_u8(self.data[174]).unwrap();
        wtr.write_u8(self.data[175]).unwrap();
        wtr.write_u8(self.data[176]).unwrap();
        wtr.write_u8(self.data[177]).unwrap();
        wtr.write_u8(self.data[178]).unwrap();
        wtr.write_u8(self.data[179]).unwrap();
        wtr.write_u8(self.data[180]).unwrap();
        wtr.write_u8(self.data[181]).unwrap();
        wtr.write_u8(self.data[182]).unwrap();
        wtr.write_u8(self.data[183]).unwrap();
        wtr.write_u8(self.data[184]).unwrap();
        wtr.write_u8(self.data[185]).unwrap();
        wtr.write_u8(self.data[186]).unwrap();
        wtr.write_u8(self.data[187]).unwrap();
        wtr.write_u8(self.data[188]).unwrap();
        wtr.write_u8(self.data[189]).unwrap();
        wtr.write_u8(self.data[190]).unwrap();
        wtr.write_u8(self.data[191]).unwrap();
        wtr.write_u8(self.data[192]).unwrap();
        wtr.write_u8(self.data[193]).unwrap();
        wtr.write_u8(self.data[194]).unwrap();
        wtr.write_u8(self.data[195]).unwrap();
        wtr.write_u8(self.data[196]).unwrap();
        wtr.write_u8(self.data[197]).unwrap();
        wtr.write_u8(self.data[198]).unwrap();
        wtr.write_u8(self.data[199]).unwrap();
        wtr.write_u8(self.data[200]).unwrap();
        wtr.write_u8(self.data[201]).unwrap();
        wtr.write_u8(self.data[202]).unwrap();
        wtr.write_u8(self.data[203]).unwrap();
        wtr.write_u8(self.data[204]).unwrap();
        wtr.write_u8(self.data[205]).unwrap();
        wtr.write_u8(self.data[206]).unwrap();
        wtr.write_u8(self.data[207]).unwrap();
        wtr.write_u8(self.data[208]).unwrap();
        wtr.write_u8(self.data[209]).unwrap();
        wtr.write_u8(self.data[210]).unwrap();
        wtr.write_u8(self.data[211]).unwrap();
        wtr.write_u8(self.data[212]).unwrap();
        wtr.write_u8(self.data[213]).unwrap();
        wtr.write_u8(self.data[214]).unwrap();
        wtr.write_u8(self.data[215]).unwrap();
        wtr.write_u8(self.data[216]).unwrap();
        wtr.write_u8(self.data[217]).unwrap();
        wtr.write_u8(self.data[218]).unwrap();
        wtr.write_u8(self.data[219]).unwrap();
        wtr.write_u8(self.data[220]).unwrap();
        wtr.write_u8(self.data[221]).unwrap();
        wtr.write_u8(self.data[222]).unwrap();
        wtr.write_u8(self.data[223]).unwrap();
        wtr.write_u8(self.data[224]).unwrap();
        wtr.write_u8(self.data[225]).unwrap();
        wtr.write_u8(self.data[226]).unwrap();
        wtr.write_u8(self.data[227]).unwrap();
        wtr.write_u8(self.data[228]).unwrap();
        wtr.write_u8(self.data[229]).unwrap();
        wtr.write_u8(self.data[230]).unwrap();
        wtr.write_u8(self.data[231]).unwrap();
        wtr.write_u8(self.data[232]).unwrap();
        wtr.write_u8(self.data[233]).unwrap();
        wtr.write_u8(self.data[234]).unwrap();
        wtr.write_u8(self.data[235]).unwrap();
        wtr.write_u8(self.data[236]).unwrap();
        wtr.write_u8(self.data[237]).unwrap();
        wtr.write_u8(self.data[238]).unwrap();
        wtr.write_u8(self.data[239]).unwrap();
        wtr.write_u8(self.data[240]).unwrap();
        wtr.write_u8(self.data[241]).unwrap();
        wtr.write_u8(self.data[242]).unwrap();
        wtr.write_u8(self.data[243]).unwrap();
        wtr.write_u8(self.data[244]).unwrap();
        wtr.write_u8(self.data[245]).unwrap();
        wtr.write_u8(self.data[246]).unwrap();
        wtr.write_u8(self.data[247]).unwrap();
        wtr.write_u8(self.data[248]).unwrap();
        wtr.write_u8(self.data[249]).unwrap();
        wtr.write_u8(self.data[250]).unwrap();
        wtr.write_u8(self.data[251]).unwrap();
        wtr.write_u8(self.data[252]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct DISTANCE_SENSOR_DATA {
    pub time_boot_ms: u32,
    pub min_distance: u16,
    pub max_distance: u16,
    pub current_distance: u16,
    pub mavtype: u8,
    pub id: u8,
    pub orientation: u8,
    pub covariance: u8,
}

impl Parsable for DISTANCE_SENSOR_DATA {
    fn parse(payload: &[u8]) -> DISTANCE_SENSOR_DATA {
        let mut cur = Cursor::new(payload);
        DISTANCE_SENSOR_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            min_distance: cur.read_u16::<LittleEndian>().unwrap(),
            max_distance: cur.read_u16::<LittleEndian>().unwrap(),
            current_distance: cur.read_u16::<LittleEndian>().unwrap(),
            mavtype: cur.read_u8().unwrap(),
            id: cur.read_u8().unwrap(),
            orientation: cur.read_u8().unwrap(),
            covariance: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_u16::<LittleEndian>(self.min_distance).unwrap();
        wtr.write_u16::<LittleEndian>(self.max_distance).unwrap();
        wtr.write_u16::<LittleEndian>(self.current_distance).unwrap();
        wtr.write_u8(self.mavtype).unwrap();
        wtr.write_u8(self.id).unwrap();
        wtr.write_u8(self.orientation).unwrap();
        wtr.write_u8(self.covariance).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct TERRAIN_REQUEST_DATA {
    pub mask: u64,
    pub lat: i32,
    pub lon: i32,
    pub grid_spacing: u16,
}

impl Parsable for TERRAIN_REQUEST_DATA {
    fn parse(payload: &[u8]) -> TERRAIN_REQUEST_DATA {
        let mut cur = Cursor::new(payload);
        TERRAIN_REQUEST_DATA {
            mask: cur.read_u64::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            grid_spacing: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.mask).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_u16::<LittleEndian>(self.grid_spacing).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct TERRAIN_DATA_DATA {
    pub lat: i32,
    pub lon: i32,
    pub grid_spacing: u16,
    pub data: Vec<i16> /* 16 */,
    pub gridbit: u8,
}

impl Parsable for TERRAIN_DATA_DATA {
    fn parse(payload: &[u8]) -> TERRAIN_DATA_DATA {
        let mut cur = Cursor::new(payload);
        TERRAIN_DATA_DATA {
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            grid_spacing: cur.read_u16::<LittleEndian>().unwrap(),
            data: vec![
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
                cur.read_i16::<LittleEndian>().unwrap(),
            ],
            gridbit: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_u16::<LittleEndian>(self.grid_spacing).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[0]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[1]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[2]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[3]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[4]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[5]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[6]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[7]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[8]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[9]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[10]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[11]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[12]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[13]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[14]).unwrap();
        wtr.write_i16::<LittleEndian>(self.data[15]).unwrap();
        wtr.write_u8(self.gridbit).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct TERRAIN_CHECK_DATA {
    pub lat: i32,
    pub lon: i32,
}

impl Parsable for TERRAIN_CHECK_DATA {
    fn parse(payload: &[u8]) -> TERRAIN_CHECK_DATA {
        let mut cur = Cursor::new(payload);
        TERRAIN_CHECK_DATA {
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct TERRAIN_REPORT_DATA {
    pub lat: i32,
    pub lon: i32,
    pub terrain_height: f32,
    pub current_height: f32,
    pub spacing: u16,
    pub pending: u16,
    pub loaded: u16,
}

impl Parsable for TERRAIN_REPORT_DATA {
    fn parse(payload: &[u8]) -> TERRAIN_REPORT_DATA {
        let mut cur = Cursor::new(payload);
        TERRAIN_REPORT_DATA {
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            terrain_height: cur.read_f32::<LittleEndian>().unwrap(),
            current_height: cur.read_f32::<LittleEndian>().unwrap(),
            spacing: cur.read_u16::<LittleEndian>().unwrap(),
            pending: cur.read_u16::<LittleEndian>().unwrap(),
            loaded: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_f32::<LittleEndian>(self.terrain_height).unwrap();
        wtr.write_f32::<LittleEndian>(self.current_height).unwrap();
        wtr.write_u16::<LittleEndian>(self.spacing).unwrap();
        wtr.write_u16::<LittleEndian>(self.pending).unwrap();
        wtr.write_u16::<LittleEndian>(self.loaded).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SCALED_PRESSURE2_DATA {
    pub time_boot_ms: u32,
    pub press_abs: f32,
    pub press_diff: f32,
    pub temperature: i16,
}

impl Parsable for SCALED_PRESSURE2_DATA {
    fn parse(payload: &[u8]) -> SCALED_PRESSURE2_DATA {
        let mut cur = Cursor::new(payload);
        SCALED_PRESSURE2_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            press_abs: cur.read_f32::<LittleEndian>().unwrap(),
            press_diff: cur.read_f32::<LittleEndian>().unwrap(),
            temperature: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.press_abs).unwrap();
        wtr.write_f32::<LittleEndian>(self.press_diff).unwrap();
        wtr.write_i16::<LittleEndian>(self.temperature).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ATT_POS_MOCAP_DATA {
    pub time_usec: u64,
    pub q: Vec<f32> /* 4 */,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Parsable for ATT_POS_MOCAP_DATA {
    fn parse(payload: &[u8]) -> ATT_POS_MOCAP_DATA {
        let mut cur = Cursor::new(payload);
        ATT_POS_MOCAP_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            q: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SET_ACTUATOR_CONTROL_TARGET_DATA {
    pub time_usec: u64,
    pub controls: Vec<f32> /* 8 */,
    pub group_mlx: u8,
    pub target_system: u8,
    pub target_component: u8,
}

impl Parsable for SET_ACTUATOR_CONTROL_TARGET_DATA {
    fn parse(payload: &[u8]) -> SET_ACTUATOR_CONTROL_TARGET_DATA {
        let mut cur = Cursor::new(payload);
        SET_ACTUATOR_CONTROL_TARGET_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            controls: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            group_mlx: cur.read_u8().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[4]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[5]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[6]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[7]).unwrap();
        wtr.write_u8(self.group_mlx).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ACTUATOR_CONTROL_TARGET_DATA {
    pub time_usec: u64,
    pub controls: Vec<f32> /* 8 */,
    pub group_mlx: u8,
}

impl Parsable for ACTUATOR_CONTROL_TARGET_DATA {
    fn parse(payload: &[u8]) -> ACTUATOR_CONTROL_TARGET_DATA {
        let mut cur = Cursor::new(payload);
        ACTUATOR_CONTROL_TARGET_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            controls: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            group_mlx: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[4]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[5]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[6]).unwrap();
        wtr.write_f32::<LittleEndian>(self.controls[7]).unwrap();
        wtr.write_u8(self.group_mlx).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ALTITUDE_DATA {
    pub time_usec: u64,
    pub altitude_monotonic: f32,
    pub altitude_amsl: f32,
    pub altitude_local: f32,
    pub altitude_relative: f32,
    pub altitude_terrain: f32,
    pub bottom_clearance: f32,
}

impl Parsable for ALTITUDE_DATA {
    fn parse(payload: &[u8]) -> ALTITUDE_DATA {
        let mut cur = Cursor::new(payload);
        ALTITUDE_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            altitude_monotonic: cur.read_f32::<LittleEndian>().unwrap(),
            altitude_amsl: cur.read_f32::<LittleEndian>().unwrap(),
            altitude_local: cur.read_f32::<LittleEndian>().unwrap(),
            altitude_relative: cur.read_f32::<LittleEndian>().unwrap(),
            altitude_terrain: cur.read_f32::<LittleEndian>().unwrap(),
            bottom_clearance: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.altitude_monotonic).unwrap();
        wtr.write_f32::<LittleEndian>(self.altitude_amsl).unwrap();
        wtr.write_f32::<LittleEndian>(self.altitude_local).unwrap();
        wtr.write_f32::<LittleEndian>(self.altitude_relative).unwrap();
        wtr.write_f32::<LittleEndian>(self.altitude_terrain).unwrap();
        wtr.write_f32::<LittleEndian>(self.bottom_clearance).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct RESOURCE_REQUEST_DATA {
    pub request_id: u8,
    pub uri_type: u8,
    pub uri: Vec<u8> /* 120 */,
    pub transfer_type: u8,
    pub storage: Vec<u8> /* 120 */,
}

impl Parsable for RESOURCE_REQUEST_DATA {
    fn parse(payload: &[u8]) -> RESOURCE_REQUEST_DATA {
        let mut cur = Cursor::new(payload);
        RESOURCE_REQUEST_DATA {
            request_id: cur.read_u8().unwrap(),
            uri_type: cur.read_u8().unwrap(),
            uri: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            transfer_type: cur.read_u8().unwrap(),
            storage: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.request_id).unwrap();
        wtr.write_u8(self.uri_type).unwrap();
        wtr.write_u8(self.uri[0]).unwrap();
        wtr.write_u8(self.uri[1]).unwrap();
        wtr.write_u8(self.uri[2]).unwrap();
        wtr.write_u8(self.uri[3]).unwrap();
        wtr.write_u8(self.uri[4]).unwrap();
        wtr.write_u8(self.uri[5]).unwrap();
        wtr.write_u8(self.uri[6]).unwrap();
        wtr.write_u8(self.uri[7]).unwrap();
        wtr.write_u8(self.uri[8]).unwrap();
        wtr.write_u8(self.uri[9]).unwrap();
        wtr.write_u8(self.uri[10]).unwrap();
        wtr.write_u8(self.uri[11]).unwrap();
        wtr.write_u8(self.uri[12]).unwrap();
        wtr.write_u8(self.uri[13]).unwrap();
        wtr.write_u8(self.uri[14]).unwrap();
        wtr.write_u8(self.uri[15]).unwrap();
        wtr.write_u8(self.uri[16]).unwrap();
        wtr.write_u8(self.uri[17]).unwrap();
        wtr.write_u8(self.uri[18]).unwrap();
        wtr.write_u8(self.uri[19]).unwrap();
        wtr.write_u8(self.uri[20]).unwrap();
        wtr.write_u8(self.uri[21]).unwrap();
        wtr.write_u8(self.uri[22]).unwrap();
        wtr.write_u8(self.uri[23]).unwrap();
        wtr.write_u8(self.uri[24]).unwrap();
        wtr.write_u8(self.uri[25]).unwrap();
        wtr.write_u8(self.uri[26]).unwrap();
        wtr.write_u8(self.uri[27]).unwrap();
        wtr.write_u8(self.uri[28]).unwrap();
        wtr.write_u8(self.uri[29]).unwrap();
        wtr.write_u8(self.uri[30]).unwrap();
        wtr.write_u8(self.uri[31]).unwrap();
        wtr.write_u8(self.uri[32]).unwrap();
        wtr.write_u8(self.uri[33]).unwrap();
        wtr.write_u8(self.uri[34]).unwrap();
        wtr.write_u8(self.uri[35]).unwrap();
        wtr.write_u8(self.uri[36]).unwrap();
        wtr.write_u8(self.uri[37]).unwrap();
        wtr.write_u8(self.uri[38]).unwrap();
        wtr.write_u8(self.uri[39]).unwrap();
        wtr.write_u8(self.uri[40]).unwrap();
        wtr.write_u8(self.uri[41]).unwrap();
        wtr.write_u8(self.uri[42]).unwrap();
        wtr.write_u8(self.uri[43]).unwrap();
        wtr.write_u8(self.uri[44]).unwrap();
        wtr.write_u8(self.uri[45]).unwrap();
        wtr.write_u8(self.uri[46]).unwrap();
        wtr.write_u8(self.uri[47]).unwrap();
        wtr.write_u8(self.uri[48]).unwrap();
        wtr.write_u8(self.uri[49]).unwrap();
        wtr.write_u8(self.uri[50]).unwrap();
        wtr.write_u8(self.uri[51]).unwrap();
        wtr.write_u8(self.uri[52]).unwrap();
        wtr.write_u8(self.uri[53]).unwrap();
        wtr.write_u8(self.uri[54]).unwrap();
        wtr.write_u8(self.uri[55]).unwrap();
        wtr.write_u8(self.uri[56]).unwrap();
        wtr.write_u8(self.uri[57]).unwrap();
        wtr.write_u8(self.uri[58]).unwrap();
        wtr.write_u8(self.uri[59]).unwrap();
        wtr.write_u8(self.uri[60]).unwrap();
        wtr.write_u8(self.uri[61]).unwrap();
        wtr.write_u8(self.uri[62]).unwrap();
        wtr.write_u8(self.uri[63]).unwrap();
        wtr.write_u8(self.uri[64]).unwrap();
        wtr.write_u8(self.uri[65]).unwrap();
        wtr.write_u8(self.uri[66]).unwrap();
        wtr.write_u8(self.uri[67]).unwrap();
        wtr.write_u8(self.uri[68]).unwrap();
        wtr.write_u8(self.uri[69]).unwrap();
        wtr.write_u8(self.uri[70]).unwrap();
        wtr.write_u8(self.uri[71]).unwrap();
        wtr.write_u8(self.uri[72]).unwrap();
        wtr.write_u8(self.uri[73]).unwrap();
        wtr.write_u8(self.uri[74]).unwrap();
        wtr.write_u8(self.uri[75]).unwrap();
        wtr.write_u8(self.uri[76]).unwrap();
        wtr.write_u8(self.uri[77]).unwrap();
        wtr.write_u8(self.uri[78]).unwrap();
        wtr.write_u8(self.uri[79]).unwrap();
        wtr.write_u8(self.uri[80]).unwrap();
        wtr.write_u8(self.uri[81]).unwrap();
        wtr.write_u8(self.uri[82]).unwrap();
        wtr.write_u8(self.uri[83]).unwrap();
        wtr.write_u8(self.uri[84]).unwrap();
        wtr.write_u8(self.uri[85]).unwrap();
        wtr.write_u8(self.uri[86]).unwrap();
        wtr.write_u8(self.uri[87]).unwrap();
        wtr.write_u8(self.uri[88]).unwrap();
        wtr.write_u8(self.uri[89]).unwrap();
        wtr.write_u8(self.uri[90]).unwrap();
        wtr.write_u8(self.uri[91]).unwrap();
        wtr.write_u8(self.uri[92]).unwrap();
        wtr.write_u8(self.uri[93]).unwrap();
        wtr.write_u8(self.uri[94]).unwrap();
        wtr.write_u8(self.uri[95]).unwrap();
        wtr.write_u8(self.uri[96]).unwrap();
        wtr.write_u8(self.uri[97]).unwrap();
        wtr.write_u8(self.uri[98]).unwrap();
        wtr.write_u8(self.uri[99]).unwrap();
        wtr.write_u8(self.uri[100]).unwrap();
        wtr.write_u8(self.uri[101]).unwrap();
        wtr.write_u8(self.uri[102]).unwrap();
        wtr.write_u8(self.uri[103]).unwrap();
        wtr.write_u8(self.uri[104]).unwrap();
        wtr.write_u8(self.uri[105]).unwrap();
        wtr.write_u8(self.uri[106]).unwrap();
        wtr.write_u8(self.uri[107]).unwrap();
        wtr.write_u8(self.uri[108]).unwrap();
        wtr.write_u8(self.uri[109]).unwrap();
        wtr.write_u8(self.uri[110]).unwrap();
        wtr.write_u8(self.uri[111]).unwrap();
        wtr.write_u8(self.uri[112]).unwrap();
        wtr.write_u8(self.uri[113]).unwrap();
        wtr.write_u8(self.uri[114]).unwrap();
        wtr.write_u8(self.uri[115]).unwrap();
        wtr.write_u8(self.uri[116]).unwrap();
        wtr.write_u8(self.uri[117]).unwrap();
        wtr.write_u8(self.uri[118]).unwrap();
        wtr.write_u8(self.uri[119]).unwrap();
        wtr.write_u8(self.transfer_type).unwrap();
        wtr.write_u8(self.storage[0]).unwrap();
        wtr.write_u8(self.storage[1]).unwrap();
        wtr.write_u8(self.storage[2]).unwrap();
        wtr.write_u8(self.storage[3]).unwrap();
        wtr.write_u8(self.storage[4]).unwrap();
        wtr.write_u8(self.storage[5]).unwrap();
        wtr.write_u8(self.storage[6]).unwrap();
        wtr.write_u8(self.storage[7]).unwrap();
        wtr.write_u8(self.storage[8]).unwrap();
        wtr.write_u8(self.storage[9]).unwrap();
        wtr.write_u8(self.storage[10]).unwrap();
        wtr.write_u8(self.storage[11]).unwrap();
        wtr.write_u8(self.storage[12]).unwrap();
        wtr.write_u8(self.storage[13]).unwrap();
        wtr.write_u8(self.storage[14]).unwrap();
        wtr.write_u8(self.storage[15]).unwrap();
        wtr.write_u8(self.storage[16]).unwrap();
        wtr.write_u8(self.storage[17]).unwrap();
        wtr.write_u8(self.storage[18]).unwrap();
        wtr.write_u8(self.storage[19]).unwrap();
        wtr.write_u8(self.storage[20]).unwrap();
        wtr.write_u8(self.storage[21]).unwrap();
        wtr.write_u8(self.storage[22]).unwrap();
        wtr.write_u8(self.storage[23]).unwrap();
        wtr.write_u8(self.storage[24]).unwrap();
        wtr.write_u8(self.storage[25]).unwrap();
        wtr.write_u8(self.storage[26]).unwrap();
        wtr.write_u8(self.storage[27]).unwrap();
        wtr.write_u8(self.storage[28]).unwrap();
        wtr.write_u8(self.storage[29]).unwrap();
        wtr.write_u8(self.storage[30]).unwrap();
        wtr.write_u8(self.storage[31]).unwrap();
        wtr.write_u8(self.storage[32]).unwrap();
        wtr.write_u8(self.storage[33]).unwrap();
        wtr.write_u8(self.storage[34]).unwrap();
        wtr.write_u8(self.storage[35]).unwrap();
        wtr.write_u8(self.storage[36]).unwrap();
        wtr.write_u8(self.storage[37]).unwrap();
        wtr.write_u8(self.storage[38]).unwrap();
        wtr.write_u8(self.storage[39]).unwrap();
        wtr.write_u8(self.storage[40]).unwrap();
        wtr.write_u8(self.storage[41]).unwrap();
        wtr.write_u8(self.storage[42]).unwrap();
        wtr.write_u8(self.storage[43]).unwrap();
        wtr.write_u8(self.storage[44]).unwrap();
        wtr.write_u8(self.storage[45]).unwrap();
        wtr.write_u8(self.storage[46]).unwrap();
        wtr.write_u8(self.storage[47]).unwrap();
        wtr.write_u8(self.storage[48]).unwrap();
        wtr.write_u8(self.storage[49]).unwrap();
        wtr.write_u8(self.storage[50]).unwrap();
        wtr.write_u8(self.storage[51]).unwrap();
        wtr.write_u8(self.storage[52]).unwrap();
        wtr.write_u8(self.storage[53]).unwrap();
        wtr.write_u8(self.storage[54]).unwrap();
        wtr.write_u8(self.storage[55]).unwrap();
        wtr.write_u8(self.storage[56]).unwrap();
        wtr.write_u8(self.storage[57]).unwrap();
        wtr.write_u8(self.storage[58]).unwrap();
        wtr.write_u8(self.storage[59]).unwrap();
        wtr.write_u8(self.storage[60]).unwrap();
        wtr.write_u8(self.storage[61]).unwrap();
        wtr.write_u8(self.storage[62]).unwrap();
        wtr.write_u8(self.storage[63]).unwrap();
        wtr.write_u8(self.storage[64]).unwrap();
        wtr.write_u8(self.storage[65]).unwrap();
        wtr.write_u8(self.storage[66]).unwrap();
        wtr.write_u8(self.storage[67]).unwrap();
        wtr.write_u8(self.storage[68]).unwrap();
        wtr.write_u8(self.storage[69]).unwrap();
        wtr.write_u8(self.storage[70]).unwrap();
        wtr.write_u8(self.storage[71]).unwrap();
        wtr.write_u8(self.storage[72]).unwrap();
        wtr.write_u8(self.storage[73]).unwrap();
        wtr.write_u8(self.storage[74]).unwrap();
        wtr.write_u8(self.storage[75]).unwrap();
        wtr.write_u8(self.storage[76]).unwrap();
        wtr.write_u8(self.storage[77]).unwrap();
        wtr.write_u8(self.storage[78]).unwrap();
        wtr.write_u8(self.storage[79]).unwrap();
        wtr.write_u8(self.storage[80]).unwrap();
        wtr.write_u8(self.storage[81]).unwrap();
        wtr.write_u8(self.storage[82]).unwrap();
        wtr.write_u8(self.storage[83]).unwrap();
        wtr.write_u8(self.storage[84]).unwrap();
        wtr.write_u8(self.storage[85]).unwrap();
        wtr.write_u8(self.storage[86]).unwrap();
        wtr.write_u8(self.storage[87]).unwrap();
        wtr.write_u8(self.storage[88]).unwrap();
        wtr.write_u8(self.storage[89]).unwrap();
        wtr.write_u8(self.storage[90]).unwrap();
        wtr.write_u8(self.storage[91]).unwrap();
        wtr.write_u8(self.storage[92]).unwrap();
        wtr.write_u8(self.storage[93]).unwrap();
        wtr.write_u8(self.storage[94]).unwrap();
        wtr.write_u8(self.storage[95]).unwrap();
        wtr.write_u8(self.storage[96]).unwrap();
        wtr.write_u8(self.storage[97]).unwrap();
        wtr.write_u8(self.storage[98]).unwrap();
        wtr.write_u8(self.storage[99]).unwrap();
        wtr.write_u8(self.storage[100]).unwrap();
        wtr.write_u8(self.storage[101]).unwrap();
        wtr.write_u8(self.storage[102]).unwrap();
        wtr.write_u8(self.storage[103]).unwrap();
        wtr.write_u8(self.storage[104]).unwrap();
        wtr.write_u8(self.storage[105]).unwrap();
        wtr.write_u8(self.storage[106]).unwrap();
        wtr.write_u8(self.storage[107]).unwrap();
        wtr.write_u8(self.storage[108]).unwrap();
        wtr.write_u8(self.storage[109]).unwrap();
        wtr.write_u8(self.storage[110]).unwrap();
        wtr.write_u8(self.storage[111]).unwrap();
        wtr.write_u8(self.storage[112]).unwrap();
        wtr.write_u8(self.storage[113]).unwrap();
        wtr.write_u8(self.storage[114]).unwrap();
        wtr.write_u8(self.storage[115]).unwrap();
        wtr.write_u8(self.storage[116]).unwrap();
        wtr.write_u8(self.storage[117]).unwrap();
        wtr.write_u8(self.storage[118]).unwrap();
        wtr.write_u8(self.storage[119]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SCALED_PRESSURE3_DATA {
    pub time_boot_ms: u32,
    pub press_abs: f32,
    pub press_diff: f32,
    pub temperature: i16,
}

impl Parsable for SCALED_PRESSURE3_DATA {
    fn parse(payload: &[u8]) -> SCALED_PRESSURE3_DATA {
        let mut cur = Cursor::new(payload);
        SCALED_PRESSURE3_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            press_abs: cur.read_f32::<LittleEndian>().unwrap(),
            press_diff: cur.read_f32::<LittleEndian>().unwrap(),
            temperature: cur.read_i16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.press_abs).unwrap();
        wtr.write_f32::<LittleEndian>(self.press_diff).unwrap();
        wtr.write_i16::<LittleEndian>(self.temperature).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct CONTROL_SYSTEM_STATE_DATA {
    pub time_usec: u64,
    pub x_acc: f32,
    pub y_acc: f32,
    pub z_acc: f32,
    pub x_vel: f32,
    pub y_vel: f32,
    pub z_vel: f32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,
    pub airspeed: f32,
    pub vel_variance: Vec<f32> /* 3 */,
    pub pos_variance: Vec<f32> /* 3 */,
    pub q: Vec<f32> /* 4 */,
    pub roll_rate: f32,
    pub pitch_rate: f32,
    pub yaw_rate: f32,
}

impl Parsable for CONTROL_SYSTEM_STATE_DATA {
    fn parse(payload: &[u8]) -> CONTROL_SYSTEM_STATE_DATA {
        let mut cur = Cursor::new(payload);
        CONTROL_SYSTEM_STATE_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            x_acc: cur.read_f32::<LittleEndian>().unwrap(),
            y_acc: cur.read_f32::<LittleEndian>().unwrap(),
            z_acc: cur.read_f32::<LittleEndian>().unwrap(),
            x_vel: cur.read_f32::<LittleEndian>().unwrap(),
            y_vel: cur.read_f32::<LittleEndian>().unwrap(),
            z_vel: cur.read_f32::<LittleEndian>().unwrap(),
            x_pos: cur.read_f32::<LittleEndian>().unwrap(),
            y_pos: cur.read_f32::<LittleEndian>().unwrap(),
            z_pos: cur.read_f32::<LittleEndian>().unwrap(),
            airspeed: cur.read_f32::<LittleEndian>().unwrap(),
            vel_variance: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            pos_variance: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            q: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            roll_rate: cur.read_f32::<LittleEndian>().unwrap(),
            pitch_rate: cur.read_f32::<LittleEndian>().unwrap(),
            yaw_rate: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.x_acc).unwrap();
        wtr.write_f32::<LittleEndian>(self.y_acc).unwrap();
        wtr.write_f32::<LittleEndian>(self.z_acc).unwrap();
        wtr.write_f32::<LittleEndian>(self.x_vel).unwrap();
        wtr.write_f32::<LittleEndian>(self.y_vel).unwrap();
        wtr.write_f32::<LittleEndian>(self.z_vel).unwrap();
        wtr.write_f32::<LittleEndian>(self.x_pos).unwrap();
        wtr.write_f32::<LittleEndian>(self.y_pos).unwrap();
        wtr.write_f32::<LittleEndian>(self.z_pos).unwrap();
        wtr.write_f32::<LittleEndian>(self.airspeed).unwrap();
        wtr.write_f32::<LittleEndian>(self.vel_variance[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.vel_variance[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.vel_variance[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.pos_variance[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.pos_variance[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.pos_variance[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.roll_rate).unwrap();
        wtr.write_f32::<LittleEndian>(self.pitch_rate).unwrap();
        wtr.write_f32::<LittleEndian>(self.yaw_rate).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct BATTERY_STATUS_DATA {
    pub current_consumed: i32,
    pub energy_consumed: i32,
    pub temperature: i16,
    pub voltages: Vec<u16> /* 10 */,
    pub current_battery: i16,
    pub id: u8,
    pub battery_function: u8,
    pub mavtype: u8,
    pub battery_remaining: i8,
}

impl Parsable for BATTERY_STATUS_DATA {
    fn parse(payload: &[u8]) -> BATTERY_STATUS_DATA {
        let mut cur = Cursor::new(payload);
        BATTERY_STATUS_DATA {
            current_consumed: cur.read_i32::<LittleEndian>().unwrap(),
            energy_consumed: cur.read_i32::<LittleEndian>().unwrap(),
            temperature: cur.read_i16::<LittleEndian>().unwrap(),
            voltages: vec![
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
                cur.read_u16::<LittleEndian>().unwrap(),
            ],
            current_battery: cur.read_i16::<LittleEndian>().unwrap(),
            id: cur.read_u8().unwrap(),
            battery_function: cur.read_u8().unwrap(),
            mavtype: cur.read_u8().unwrap(),
            battery_remaining: cur.read_i8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.current_consumed).unwrap();
        wtr.write_i32::<LittleEndian>(self.energy_consumed).unwrap();
        wtr.write_i16::<LittleEndian>(self.temperature).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[0]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[1]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[2]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[3]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[4]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[5]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[6]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[7]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[8]).unwrap();
        wtr.write_u16::<LittleEndian>(self.voltages[9]).unwrap();
        wtr.write_i16::<LittleEndian>(self.current_battery).unwrap();
        wtr.write_u8(self.id).unwrap();
        wtr.write_u8(self.battery_function).unwrap();
        wtr.write_u8(self.mavtype).unwrap();
        wtr.write_i8(self.battery_remaining).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct AUTOPILOT_VERSION_DATA {
    pub capabilities: u64,
    pub uid: u64,
    pub flight_sw_version: u32,
    pub middleware_sw_version: u32,
    pub os_sw_version: u32,
    pub board_version: u32,
    pub vendor_id: u16,
    pub product_id: u16,
    pub flight_custom_version: Vec<u8> /* 8 */,
    pub middleware_custom_version: Vec<u8> /* 8 */,
    pub os_custom_version: Vec<u8> /* 8 */,
}

impl Parsable for AUTOPILOT_VERSION_DATA {
    fn parse(payload: &[u8]) -> AUTOPILOT_VERSION_DATA {
        let mut cur = Cursor::new(payload);
        AUTOPILOT_VERSION_DATA {
            capabilities: cur.read_u64::<LittleEndian>().unwrap(),
            uid: cur.read_u64::<LittleEndian>().unwrap(),
            flight_sw_version: cur.read_u32::<LittleEndian>().unwrap(),
            middleware_sw_version: cur.read_u32::<LittleEndian>().unwrap(),
            os_sw_version: cur.read_u32::<LittleEndian>().unwrap(),
            board_version: cur.read_u32::<LittleEndian>().unwrap(),
            vendor_id: cur.read_u16::<LittleEndian>().unwrap(),
            product_id: cur.read_u16::<LittleEndian>().unwrap(),
            flight_custom_version: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            middleware_custom_version: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            os_custom_version: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.capabilities).unwrap();
        wtr.write_u64::<LittleEndian>(self.uid).unwrap();
        wtr.write_u32::<LittleEndian>(self.flight_sw_version).unwrap();
        wtr.write_u32::<LittleEndian>(self.middleware_sw_version).unwrap();
        wtr.write_u32::<LittleEndian>(self.os_sw_version).unwrap();
        wtr.write_u32::<LittleEndian>(self.board_version).unwrap();
        wtr.write_u16::<LittleEndian>(self.vendor_id).unwrap();
        wtr.write_u16::<LittleEndian>(self.product_id).unwrap();
        wtr.write_u8(self.flight_custom_version[0]).unwrap();
        wtr.write_u8(self.flight_custom_version[1]).unwrap();
        wtr.write_u8(self.flight_custom_version[2]).unwrap();
        wtr.write_u8(self.flight_custom_version[3]).unwrap();
        wtr.write_u8(self.flight_custom_version[4]).unwrap();
        wtr.write_u8(self.flight_custom_version[5]).unwrap();
        wtr.write_u8(self.flight_custom_version[6]).unwrap();
        wtr.write_u8(self.flight_custom_version[7]).unwrap();
        wtr.write_u8(self.middleware_custom_version[0]).unwrap();
        wtr.write_u8(self.middleware_custom_version[1]).unwrap();
        wtr.write_u8(self.middleware_custom_version[2]).unwrap();
        wtr.write_u8(self.middleware_custom_version[3]).unwrap();
        wtr.write_u8(self.middleware_custom_version[4]).unwrap();
        wtr.write_u8(self.middleware_custom_version[5]).unwrap();
        wtr.write_u8(self.middleware_custom_version[6]).unwrap();
        wtr.write_u8(self.middleware_custom_version[7]).unwrap();
        wtr.write_u8(self.os_custom_version[0]).unwrap();
        wtr.write_u8(self.os_custom_version[1]).unwrap();
        wtr.write_u8(self.os_custom_version[2]).unwrap();
        wtr.write_u8(self.os_custom_version[3]).unwrap();
        wtr.write_u8(self.os_custom_version[4]).unwrap();
        wtr.write_u8(self.os_custom_version[5]).unwrap();
        wtr.write_u8(self.os_custom_version[6]).unwrap();
        wtr.write_u8(self.os_custom_version[7]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct LANDING_TARGET_DATA {
    pub time_usec: u64,
    pub angle_x: f32,
    pub angle_y: f32,
    pub distance: f32,
    pub size_x: f32,
    pub size_y: f32,
    pub target_num: u8,
    pub frame: u8,
}

impl Parsable for LANDING_TARGET_DATA {
    fn parse(payload: &[u8]) -> LANDING_TARGET_DATA {
        let mut cur = Cursor::new(payload);
        LANDING_TARGET_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            angle_x: cur.read_f32::<LittleEndian>().unwrap(),
            angle_y: cur.read_f32::<LittleEndian>().unwrap(),
            distance: cur.read_f32::<LittleEndian>().unwrap(),
            size_x: cur.read_f32::<LittleEndian>().unwrap(),
            size_y: cur.read_f32::<LittleEndian>().unwrap(),
            target_num: cur.read_u8().unwrap(),
            frame: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.angle_x).unwrap();
        wtr.write_f32::<LittleEndian>(self.angle_y).unwrap();
        wtr.write_f32::<LittleEndian>(self.distance).unwrap();
        wtr.write_f32::<LittleEndian>(self.size_x).unwrap();
        wtr.write_f32::<LittleEndian>(self.size_y).unwrap();
        wtr.write_u8(self.target_num).unwrap();
        wtr.write_u8(self.frame).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct VIBRATION_DATA {
    pub time_usec: u64,
    pub vibration_x: f32,
    pub vibration_y: f32,
    pub vibration_z: f32,
    pub clipping_0: u32,
    pub clipping_1: u32,
    pub clipping_2: u32,
}

impl Parsable for VIBRATION_DATA {
    fn parse(payload: &[u8]) -> VIBRATION_DATA {
        let mut cur = Cursor::new(payload);
        VIBRATION_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            vibration_x: cur.read_f32::<LittleEndian>().unwrap(),
            vibration_y: cur.read_f32::<LittleEndian>().unwrap(),
            vibration_z: cur.read_f32::<LittleEndian>().unwrap(),
            clipping_0: cur.read_u32::<LittleEndian>().unwrap(),
            clipping_1: cur.read_u32::<LittleEndian>().unwrap(),
            clipping_2: cur.read_u32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.vibration_x).unwrap();
        wtr.write_f32::<LittleEndian>(self.vibration_y).unwrap();
        wtr.write_f32::<LittleEndian>(self.vibration_z).unwrap();
        wtr.write_u32::<LittleEndian>(self.clipping_0).unwrap();
        wtr.write_u32::<LittleEndian>(self.clipping_1).unwrap();
        wtr.write_u32::<LittleEndian>(self.clipping_2).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct HOME_POSITION_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub q: Vec<f32> /* 4 */,
    pub approach_x: f32,
    pub approach_y: f32,
    pub approach_z: f32,
}

impl Parsable for HOME_POSITION_DATA {
    fn parse(payload: &[u8]) -> HOME_POSITION_DATA {
        let mut cur = Cursor::new(payload);
        HOME_POSITION_DATA {
            latitude: cur.read_i32::<LittleEndian>().unwrap(),
            longitude: cur.read_i32::<LittleEndian>().unwrap(),
            altitude: cur.read_i32::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            q: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            approach_x: cur.read_f32::<LittleEndian>().unwrap(),
            approach_y: cur.read_f32::<LittleEndian>().unwrap(),
            approach_z: cur.read_f32::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.latitude).unwrap();
        wtr.write_i32::<LittleEndian>(self.longitude).unwrap();
        wtr.write_i32::<LittleEndian>(self.altitude).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.approach_x).unwrap();
        wtr.write_f32::<LittleEndian>(self.approach_y).unwrap();
        wtr.write_f32::<LittleEndian>(self.approach_z).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct SET_HOME_POSITION_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub q: Vec<f32> /* 4 */,
    pub approach_x: f32,
    pub approach_y: f32,
    pub approach_z: f32,
    pub target_system: u8,
}

impl Parsable for SET_HOME_POSITION_DATA {
    fn parse(payload: &[u8]) -> SET_HOME_POSITION_DATA {
        let mut cur = Cursor::new(payload);
        SET_HOME_POSITION_DATA {
            latitude: cur.read_i32::<LittleEndian>().unwrap(),
            longitude: cur.read_i32::<LittleEndian>().unwrap(),
            altitude: cur.read_i32::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            q: vec![
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
                cur.read_f32::<LittleEndian>().unwrap(),
            ],
            approach_x: cur.read_f32::<LittleEndian>().unwrap(),
            approach_y: cur.read_f32::<LittleEndian>().unwrap(),
            approach_z: cur.read_f32::<LittleEndian>().unwrap(),
            target_system: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.latitude).unwrap();
        wtr.write_i32::<LittleEndian>(self.longitude).unwrap();
        wtr.write_i32::<LittleEndian>(self.altitude).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[0]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[1]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[2]).unwrap();
        wtr.write_f32::<LittleEndian>(self.q[3]).unwrap();
        wtr.write_f32::<LittleEndian>(self.approach_x).unwrap();
        wtr.write_f32::<LittleEndian>(self.approach_y).unwrap();
        wtr.write_f32::<LittleEndian>(self.approach_z).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MESSAGE_INTERVAL_DATA {
    pub interval_us: i32,
    pub message_id: u16,
}

impl Parsable for MESSAGE_INTERVAL_DATA {
    fn parse(payload: &[u8]) -> MESSAGE_INTERVAL_DATA {
        let mut cur = Cursor::new(payload);
        MESSAGE_INTERVAL_DATA {
            interval_us: cur.read_i32::<LittleEndian>().unwrap(),
            message_id: cur.read_u16::<LittleEndian>().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.interval_us).unwrap();
        wtr.write_u16::<LittleEndian>(self.message_id).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct EXTENDED_SYS_STATE_DATA {
    pub vtol_state: u8,
    pub landed_state: u8,
}

impl Parsable for EXTENDED_SYS_STATE_DATA {
    fn parse(payload: &[u8]) -> EXTENDED_SYS_STATE_DATA {
        let mut cur = Cursor::new(payload);
        EXTENDED_SYS_STATE_DATA {
            vtol_state: cur.read_u8().unwrap(),
            landed_state: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.vtol_state).unwrap();
        wtr.write_u8(self.landed_state).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct ADSB_VEHICLE_DATA {
    pub ICAO_address: u32,
    pub lat: i32,
    pub lon: i32,
    pub altitude: f32,
    pub hor_velocity: f32,
    pub ver_velocity: f32,
    pub heading: u16,
    pub flags: u16,
    pub squawk: u16,
    pub altitude_type: u8,
    pub callsign: Vec<u8> /* 9 */,
    pub emitter_type: u8,
    pub tslc: u8,
}

impl Parsable for ADSB_VEHICLE_DATA {
    fn parse(payload: &[u8]) -> ADSB_VEHICLE_DATA {
        let mut cur = Cursor::new(payload);
        ADSB_VEHICLE_DATA {
            ICAO_address: cur.read_u32::<LittleEndian>().unwrap(),
            lat: cur.read_i32::<LittleEndian>().unwrap(),
            lon: cur.read_i32::<LittleEndian>().unwrap(),
            altitude: cur.read_f32::<LittleEndian>().unwrap(),
            hor_velocity: cur.read_f32::<LittleEndian>().unwrap(),
            ver_velocity: cur.read_f32::<LittleEndian>().unwrap(),
            heading: cur.read_u16::<LittleEndian>().unwrap(),
            flags: cur.read_u16::<LittleEndian>().unwrap(),
            squawk: cur.read_u16::<LittleEndian>().unwrap(),
            altitude_type: cur.read_u8().unwrap(),
            callsign: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
            emitter_type: cur.read_u8().unwrap(),
            tslc: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.ICAO_address).unwrap();
        wtr.write_i32::<LittleEndian>(self.lat).unwrap();
        wtr.write_i32::<LittleEndian>(self.lon).unwrap();
        wtr.write_f32::<LittleEndian>(self.altitude).unwrap();
        wtr.write_f32::<LittleEndian>(self.hor_velocity).unwrap();
        wtr.write_f32::<LittleEndian>(self.ver_velocity).unwrap();
        wtr.write_u16::<LittleEndian>(self.heading).unwrap();
        wtr.write_u16::<LittleEndian>(self.flags).unwrap();
        wtr.write_u16::<LittleEndian>(self.squawk).unwrap();
        wtr.write_u8(self.altitude_type).unwrap();
        wtr.write_u8(self.callsign[0]).unwrap();
        wtr.write_u8(self.callsign[1]).unwrap();
        wtr.write_u8(self.callsign[2]).unwrap();
        wtr.write_u8(self.callsign[3]).unwrap();
        wtr.write_u8(self.callsign[4]).unwrap();
        wtr.write_u8(self.callsign[5]).unwrap();
        wtr.write_u8(self.callsign[6]).unwrap();
        wtr.write_u8(self.callsign[7]).unwrap();
        wtr.write_u8(self.callsign[8]).unwrap();
        wtr.write_u8(self.emitter_type).unwrap();
        wtr.write_u8(self.tslc).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct V2_EXTENSION_DATA {
    pub message_type: u16,
    pub target_network: u8,
    pub target_system: u8,
    pub target_component: u8,
    pub payload: Vec<u8> /* 249 */,
}

impl Parsable for V2_EXTENSION_DATA {
    fn parse(payload: &[u8]) -> V2_EXTENSION_DATA {
        let mut cur = Cursor::new(payload);
        V2_EXTENSION_DATA {
            message_type: cur.read_u16::<LittleEndian>().unwrap(),
            target_network: cur.read_u8().unwrap(),
            target_system: cur.read_u8().unwrap(),
            target_component: cur.read_u8().unwrap(),
            payload: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.message_type).unwrap();
        wtr.write_u8(self.target_network).unwrap();
        wtr.write_u8(self.target_system).unwrap();
        wtr.write_u8(self.target_component).unwrap();
        wtr.write_u8(self.payload[0]).unwrap();
        wtr.write_u8(self.payload[1]).unwrap();
        wtr.write_u8(self.payload[2]).unwrap();
        wtr.write_u8(self.payload[3]).unwrap();
        wtr.write_u8(self.payload[4]).unwrap();
        wtr.write_u8(self.payload[5]).unwrap();
        wtr.write_u8(self.payload[6]).unwrap();
        wtr.write_u8(self.payload[7]).unwrap();
        wtr.write_u8(self.payload[8]).unwrap();
        wtr.write_u8(self.payload[9]).unwrap();
        wtr.write_u8(self.payload[10]).unwrap();
        wtr.write_u8(self.payload[11]).unwrap();
        wtr.write_u8(self.payload[12]).unwrap();
        wtr.write_u8(self.payload[13]).unwrap();
        wtr.write_u8(self.payload[14]).unwrap();
        wtr.write_u8(self.payload[15]).unwrap();
        wtr.write_u8(self.payload[16]).unwrap();
        wtr.write_u8(self.payload[17]).unwrap();
        wtr.write_u8(self.payload[18]).unwrap();
        wtr.write_u8(self.payload[19]).unwrap();
        wtr.write_u8(self.payload[20]).unwrap();
        wtr.write_u8(self.payload[21]).unwrap();
        wtr.write_u8(self.payload[22]).unwrap();
        wtr.write_u8(self.payload[23]).unwrap();
        wtr.write_u8(self.payload[24]).unwrap();
        wtr.write_u8(self.payload[25]).unwrap();
        wtr.write_u8(self.payload[26]).unwrap();
        wtr.write_u8(self.payload[27]).unwrap();
        wtr.write_u8(self.payload[28]).unwrap();
        wtr.write_u8(self.payload[29]).unwrap();
        wtr.write_u8(self.payload[30]).unwrap();
        wtr.write_u8(self.payload[31]).unwrap();
        wtr.write_u8(self.payload[32]).unwrap();
        wtr.write_u8(self.payload[33]).unwrap();
        wtr.write_u8(self.payload[34]).unwrap();
        wtr.write_u8(self.payload[35]).unwrap();
        wtr.write_u8(self.payload[36]).unwrap();
        wtr.write_u8(self.payload[37]).unwrap();
        wtr.write_u8(self.payload[38]).unwrap();
        wtr.write_u8(self.payload[39]).unwrap();
        wtr.write_u8(self.payload[40]).unwrap();
        wtr.write_u8(self.payload[41]).unwrap();
        wtr.write_u8(self.payload[42]).unwrap();
        wtr.write_u8(self.payload[43]).unwrap();
        wtr.write_u8(self.payload[44]).unwrap();
        wtr.write_u8(self.payload[45]).unwrap();
        wtr.write_u8(self.payload[46]).unwrap();
        wtr.write_u8(self.payload[47]).unwrap();
        wtr.write_u8(self.payload[48]).unwrap();
        wtr.write_u8(self.payload[49]).unwrap();
        wtr.write_u8(self.payload[50]).unwrap();
        wtr.write_u8(self.payload[51]).unwrap();
        wtr.write_u8(self.payload[52]).unwrap();
        wtr.write_u8(self.payload[53]).unwrap();
        wtr.write_u8(self.payload[54]).unwrap();
        wtr.write_u8(self.payload[55]).unwrap();
        wtr.write_u8(self.payload[56]).unwrap();
        wtr.write_u8(self.payload[57]).unwrap();
        wtr.write_u8(self.payload[58]).unwrap();
        wtr.write_u8(self.payload[59]).unwrap();
        wtr.write_u8(self.payload[60]).unwrap();
        wtr.write_u8(self.payload[61]).unwrap();
        wtr.write_u8(self.payload[62]).unwrap();
        wtr.write_u8(self.payload[63]).unwrap();
        wtr.write_u8(self.payload[64]).unwrap();
        wtr.write_u8(self.payload[65]).unwrap();
        wtr.write_u8(self.payload[66]).unwrap();
        wtr.write_u8(self.payload[67]).unwrap();
        wtr.write_u8(self.payload[68]).unwrap();
        wtr.write_u8(self.payload[69]).unwrap();
        wtr.write_u8(self.payload[70]).unwrap();
        wtr.write_u8(self.payload[71]).unwrap();
        wtr.write_u8(self.payload[72]).unwrap();
        wtr.write_u8(self.payload[73]).unwrap();
        wtr.write_u8(self.payload[74]).unwrap();
        wtr.write_u8(self.payload[75]).unwrap();
        wtr.write_u8(self.payload[76]).unwrap();
        wtr.write_u8(self.payload[77]).unwrap();
        wtr.write_u8(self.payload[78]).unwrap();
        wtr.write_u8(self.payload[79]).unwrap();
        wtr.write_u8(self.payload[80]).unwrap();
        wtr.write_u8(self.payload[81]).unwrap();
        wtr.write_u8(self.payload[82]).unwrap();
        wtr.write_u8(self.payload[83]).unwrap();
        wtr.write_u8(self.payload[84]).unwrap();
        wtr.write_u8(self.payload[85]).unwrap();
        wtr.write_u8(self.payload[86]).unwrap();
        wtr.write_u8(self.payload[87]).unwrap();
        wtr.write_u8(self.payload[88]).unwrap();
        wtr.write_u8(self.payload[89]).unwrap();
        wtr.write_u8(self.payload[90]).unwrap();
        wtr.write_u8(self.payload[91]).unwrap();
        wtr.write_u8(self.payload[92]).unwrap();
        wtr.write_u8(self.payload[93]).unwrap();
        wtr.write_u8(self.payload[94]).unwrap();
        wtr.write_u8(self.payload[95]).unwrap();
        wtr.write_u8(self.payload[96]).unwrap();
        wtr.write_u8(self.payload[97]).unwrap();
        wtr.write_u8(self.payload[98]).unwrap();
        wtr.write_u8(self.payload[99]).unwrap();
        wtr.write_u8(self.payload[100]).unwrap();
        wtr.write_u8(self.payload[101]).unwrap();
        wtr.write_u8(self.payload[102]).unwrap();
        wtr.write_u8(self.payload[103]).unwrap();
        wtr.write_u8(self.payload[104]).unwrap();
        wtr.write_u8(self.payload[105]).unwrap();
        wtr.write_u8(self.payload[106]).unwrap();
        wtr.write_u8(self.payload[107]).unwrap();
        wtr.write_u8(self.payload[108]).unwrap();
        wtr.write_u8(self.payload[109]).unwrap();
        wtr.write_u8(self.payload[110]).unwrap();
        wtr.write_u8(self.payload[111]).unwrap();
        wtr.write_u8(self.payload[112]).unwrap();
        wtr.write_u8(self.payload[113]).unwrap();
        wtr.write_u8(self.payload[114]).unwrap();
        wtr.write_u8(self.payload[115]).unwrap();
        wtr.write_u8(self.payload[116]).unwrap();
        wtr.write_u8(self.payload[117]).unwrap();
        wtr.write_u8(self.payload[118]).unwrap();
        wtr.write_u8(self.payload[119]).unwrap();
        wtr.write_u8(self.payload[120]).unwrap();
        wtr.write_u8(self.payload[121]).unwrap();
        wtr.write_u8(self.payload[122]).unwrap();
        wtr.write_u8(self.payload[123]).unwrap();
        wtr.write_u8(self.payload[124]).unwrap();
        wtr.write_u8(self.payload[125]).unwrap();
        wtr.write_u8(self.payload[126]).unwrap();
        wtr.write_u8(self.payload[127]).unwrap();
        wtr.write_u8(self.payload[128]).unwrap();
        wtr.write_u8(self.payload[129]).unwrap();
        wtr.write_u8(self.payload[130]).unwrap();
        wtr.write_u8(self.payload[131]).unwrap();
        wtr.write_u8(self.payload[132]).unwrap();
        wtr.write_u8(self.payload[133]).unwrap();
        wtr.write_u8(self.payload[134]).unwrap();
        wtr.write_u8(self.payload[135]).unwrap();
        wtr.write_u8(self.payload[136]).unwrap();
        wtr.write_u8(self.payload[137]).unwrap();
        wtr.write_u8(self.payload[138]).unwrap();
        wtr.write_u8(self.payload[139]).unwrap();
        wtr.write_u8(self.payload[140]).unwrap();
        wtr.write_u8(self.payload[141]).unwrap();
        wtr.write_u8(self.payload[142]).unwrap();
        wtr.write_u8(self.payload[143]).unwrap();
        wtr.write_u8(self.payload[144]).unwrap();
        wtr.write_u8(self.payload[145]).unwrap();
        wtr.write_u8(self.payload[146]).unwrap();
        wtr.write_u8(self.payload[147]).unwrap();
        wtr.write_u8(self.payload[148]).unwrap();
        wtr.write_u8(self.payload[149]).unwrap();
        wtr.write_u8(self.payload[150]).unwrap();
        wtr.write_u8(self.payload[151]).unwrap();
        wtr.write_u8(self.payload[152]).unwrap();
        wtr.write_u8(self.payload[153]).unwrap();
        wtr.write_u8(self.payload[154]).unwrap();
        wtr.write_u8(self.payload[155]).unwrap();
        wtr.write_u8(self.payload[156]).unwrap();
        wtr.write_u8(self.payload[157]).unwrap();
        wtr.write_u8(self.payload[158]).unwrap();
        wtr.write_u8(self.payload[159]).unwrap();
        wtr.write_u8(self.payload[160]).unwrap();
        wtr.write_u8(self.payload[161]).unwrap();
        wtr.write_u8(self.payload[162]).unwrap();
        wtr.write_u8(self.payload[163]).unwrap();
        wtr.write_u8(self.payload[164]).unwrap();
        wtr.write_u8(self.payload[165]).unwrap();
        wtr.write_u8(self.payload[166]).unwrap();
        wtr.write_u8(self.payload[167]).unwrap();
        wtr.write_u8(self.payload[168]).unwrap();
        wtr.write_u8(self.payload[169]).unwrap();
        wtr.write_u8(self.payload[170]).unwrap();
        wtr.write_u8(self.payload[171]).unwrap();
        wtr.write_u8(self.payload[172]).unwrap();
        wtr.write_u8(self.payload[173]).unwrap();
        wtr.write_u8(self.payload[174]).unwrap();
        wtr.write_u8(self.payload[175]).unwrap();
        wtr.write_u8(self.payload[176]).unwrap();
        wtr.write_u8(self.payload[177]).unwrap();
        wtr.write_u8(self.payload[178]).unwrap();
        wtr.write_u8(self.payload[179]).unwrap();
        wtr.write_u8(self.payload[180]).unwrap();
        wtr.write_u8(self.payload[181]).unwrap();
        wtr.write_u8(self.payload[182]).unwrap();
        wtr.write_u8(self.payload[183]).unwrap();
        wtr.write_u8(self.payload[184]).unwrap();
        wtr.write_u8(self.payload[185]).unwrap();
        wtr.write_u8(self.payload[186]).unwrap();
        wtr.write_u8(self.payload[187]).unwrap();
        wtr.write_u8(self.payload[188]).unwrap();
        wtr.write_u8(self.payload[189]).unwrap();
        wtr.write_u8(self.payload[190]).unwrap();
        wtr.write_u8(self.payload[191]).unwrap();
        wtr.write_u8(self.payload[192]).unwrap();
        wtr.write_u8(self.payload[193]).unwrap();
        wtr.write_u8(self.payload[194]).unwrap();
        wtr.write_u8(self.payload[195]).unwrap();
        wtr.write_u8(self.payload[196]).unwrap();
        wtr.write_u8(self.payload[197]).unwrap();
        wtr.write_u8(self.payload[198]).unwrap();
        wtr.write_u8(self.payload[199]).unwrap();
        wtr.write_u8(self.payload[200]).unwrap();
        wtr.write_u8(self.payload[201]).unwrap();
        wtr.write_u8(self.payload[202]).unwrap();
        wtr.write_u8(self.payload[203]).unwrap();
        wtr.write_u8(self.payload[204]).unwrap();
        wtr.write_u8(self.payload[205]).unwrap();
        wtr.write_u8(self.payload[206]).unwrap();
        wtr.write_u8(self.payload[207]).unwrap();
        wtr.write_u8(self.payload[208]).unwrap();
        wtr.write_u8(self.payload[209]).unwrap();
        wtr.write_u8(self.payload[210]).unwrap();
        wtr.write_u8(self.payload[211]).unwrap();
        wtr.write_u8(self.payload[212]).unwrap();
        wtr.write_u8(self.payload[213]).unwrap();
        wtr.write_u8(self.payload[214]).unwrap();
        wtr.write_u8(self.payload[215]).unwrap();
        wtr.write_u8(self.payload[216]).unwrap();
        wtr.write_u8(self.payload[217]).unwrap();
        wtr.write_u8(self.payload[218]).unwrap();
        wtr.write_u8(self.payload[219]).unwrap();
        wtr.write_u8(self.payload[220]).unwrap();
        wtr.write_u8(self.payload[221]).unwrap();
        wtr.write_u8(self.payload[222]).unwrap();
        wtr.write_u8(self.payload[223]).unwrap();
        wtr.write_u8(self.payload[224]).unwrap();
        wtr.write_u8(self.payload[225]).unwrap();
        wtr.write_u8(self.payload[226]).unwrap();
        wtr.write_u8(self.payload[227]).unwrap();
        wtr.write_u8(self.payload[228]).unwrap();
        wtr.write_u8(self.payload[229]).unwrap();
        wtr.write_u8(self.payload[230]).unwrap();
        wtr.write_u8(self.payload[231]).unwrap();
        wtr.write_u8(self.payload[232]).unwrap();
        wtr.write_u8(self.payload[233]).unwrap();
        wtr.write_u8(self.payload[234]).unwrap();
        wtr.write_u8(self.payload[235]).unwrap();
        wtr.write_u8(self.payload[236]).unwrap();
        wtr.write_u8(self.payload[237]).unwrap();
        wtr.write_u8(self.payload[238]).unwrap();
        wtr.write_u8(self.payload[239]).unwrap();
        wtr.write_u8(self.payload[240]).unwrap();
        wtr.write_u8(self.payload[241]).unwrap();
        wtr.write_u8(self.payload[242]).unwrap();
        wtr.write_u8(self.payload[243]).unwrap();
        wtr.write_u8(self.payload[244]).unwrap();
        wtr.write_u8(self.payload[245]).unwrap();
        wtr.write_u8(self.payload[246]).unwrap();
        wtr.write_u8(self.payload[247]).unwrap();
        wtr.write_u8(self.payload[248]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct MEMORY_VECT_DATA {
    pub address: u16,
    pub ver: u8,
    pub mavtype: u8,
    pub value: Vec<i8> /* 32 */,
}

impl Parsable for MEMORY_VECT_DATA {
    fn parse(payload: &[u8]) -> MEMORY_VECT_DATA {
        let mut cur = Cursor::new(payload);
        MEMORY_VECT_DATA {
            address: cur.read_u16::<LittleEndian>().unwrap(),
            ver: cur.read_u8().unwrap(),
            mavtype: cur.read_u8().unwrap(),
            value: vec![
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
                cur.read_i8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(self.address).unwrap();
        wtr.write_u8(self.ver).unwrap();
        wtr.write_u8(self.mavtype).unwrap();
        wtr.write_i8(self.value[0]).unwrap();
        wtr.write_i8(self.value[1]).unwrap();
        wtr.write_i8(self.value[2]).unwrap();
        wtr.write_i8(self.value[3]).unwrap();
        wtr.write_i8(self.value[4]).unwrap();
        wtr.write_i8(self.value[5]).unwrap();
        wtr.write_i8(self.value[6]).unwrap();
        wtr.write_i8(self.value[7]).unwrap();
        wtr.write_i8(self.value[8]).unwrap();
        wtr.write_i8(self.value[9]).unwrap();
        wtr.write_i8(self.value[10]).unwrap();
        wtr.write_i8(self.value[11]).unwrap();
        wtr.write_i8(self.value[12]).unwrap();
        wtr.write_i8(self.value[13]).unwrap();
        wtr.write_i8(self.value[14]).unwrap();
        wtr.write_i8(self.value[15]).unwrap();
        wtr.write_i8(self.value[16]).unwrap();
        wtr.write_i8(self.value[17]).unwrap();
        wtr.write_i8(self.value[18]).unwrap();
        wtr.write_i8(self.value[19]).unwrap();
        wtr.write_i8(self.value[20]).unwrap();
        wtr.write_i8(self.value[21]).unwrap();
        wtr.write_i8(self.value[22]).unwrap();
        wtr.write_i8(self.value[23]).unwrap();
        wtr.write_i8(self.value[24]).unwrap();
        wtr.write_i8(self.value[25]).unwrap();
        wtr.write_i8(self.value[26]).unwrap();
        wtr.write_i8(self.value[27]).unwrap();
        wtr.write_i8(self.value[28]).unwrap();
        wtr.write_i8(self.value[29]).unwrap();
        wtr.write_i8(self.value[30]).unwrap();
        wtr.write_i8(self.value[31]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct DEBUG_VECT_DATA {
    pub time_usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub name: Vec<u8> /* 10 */,
}

impl Parsable for DEBUG_VECT_DATA {
    fn parse(payload: &[u8]) -> DEBUG_VECT_DATA {
        let mut cur = Cursor::new(payload);
        DEBUG_VECT_DATA {
            time_usec: cur.read_u64::<LittleEndian>().unwrap(),
            x: cur.read_f32::<LittleEndian>().unwrap(),
            y: cur.read_f32::<LittleEndian>().unwrap(),
            z: cur.read_f32::<LittleEndian>().unwrap(),
            name: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.time_usec).unwrap();
        wtr.write_f32::<LittleEndian>(self.x).unwrap();
        wtr.write_f32::<LittleEndian>(self.y).unwrap();
        wtr.write_f32::<LittleEndian>(self.z).unwrap();
        wtr.write_u8(self.name[0]).unwrap();
        wtr.write_u8(self.name[1]).unwrap();
        wtr.write_u8(self.name[2]).unwrap();
        wtr.write_u8(self.name[3]).unwrap();
        wtr.write_u8(self.name[4]).unwrap();
        wtr.write_u8(self.name[5]).unwrap();
        wtr.write_u8(self.name[6]).unwrap();
        wtr.write_u8(self.name[7]).unwrap();
        wtr.write_u8(self.name[8]).unwrap();
        wtr.write_u8(self.name[9]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct NAMED_VALUE_FLOAT_DATA {
    pub time_boot_ms: u32,
    pub value: f32,
    pub name: Vec<u8> /* 10 */,
}

impl Parsable for NAMED_VALUE_FLOAT_DATA {
    fn parse(payload: &[u8]) -> NAMED_VALUE_FLOAT_DATA {
        let mut cur = Cursor::new(payload);
        NAMED_VALUE_FLOAT_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            value: cur.read_f32::<LittleEndian>().unwrap(),
            name: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.value).unwrap();
        wtr.write_u8(self.name[0]).unwrap();
        wtr.write_u8(self.name[1]).unwrap();
        wtr.write_u8(self.name[2]).unwrap();
        wtr.write_u8(self.name[3]).unwrap();
        wtr.write_u8(self.name[4]).unwrap();
        wtr.write_u8(self.name[5]).unwrap();
        wtr.write_u8(self.name[6]).unwrap();
        wtr.write_u8(self.name[7]).unwrap();
        wtr.write_u8(self.name[8]).unwrap();
        wtr.write_u8(self.name[9]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct NAMED_VALUE_INT_DATA {
    pub time_boot_ms: u32,
    pub value: i32,
    pub name: Vec<u8> /* 10 */,
}

impl Parsable for NAMED_VALUE_INT_DATA {
    fn parse(payload: &[u8]) -> NAMED_VALUE_INT_DATA {
        let mut cur = Cursor::new(payload);
        NAMED_VALUE_INT_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            value: cur.read_i32::<LittleEndian>().unwrap(),
            name: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_i32::<LittleEndian>(self.value).unwrap();
        wtr.write_u8(self.name[0]).unwrap();
        wtr.write_u8(self.name[1]).unwrap();
        wtr.write_u8(self.name[2]).unwrap();
        wtr.write_u8(self.name[3]).unwrap();
        wtr.write_u8(self.name[4]).unwrap();
        wtr.write_u8(self.name[5]).unwrap();
        wtr.write_u8(self.name[6]).unwrap();
        wtr.write_u8(self.name[7]).unwrap();
        wtr.write_u8(self.name[8]).unwrap();
        wtr.write_u8(self.name[9]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct STATUSTEXT_DATA {
    pub severity: u8,
    pub text: Vec<u8> /* 50 */,
}

impl Parsable for STATUSTEXT_DATA {
    fn parse(payload: &[u8]) -> STATUSTEXT_DATA {
        let mut cur = Cursor::new(payload);
        STATUSTEXT_DATA {
            severity: cur.read_u8().unwrap(),
            text: vec![
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
                cur.read_u8().unwrap(),
            ],
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.severity).unwrap();
        wtr.write_u8(self.text[0]).unwrap();
        wtr.write_u8(self.text[1]).unwrap();
        wtr.write_u8(self.text[2]).unwrap();
        wtr.write_u8(self.text[3]).unwrap();
        wtr.write_u8(self.text[4]).unwrap();
        wtr.write_u8(self.text[5]).unwrap();
        wtr.write_u8(self.text[6]).unwrap();
        wtr.write_u8(self.text[7]).unwrap();
        wtr.write_u8(self.text[8]).unwrap();
        wtr.write_u8(self.text[9]).unwrap();
        wtr.write_u8(self.text[10]).unwrap();
        wtr.write_u8(self.text[11]).unwrap();
        wtr.write_u8(self.text[12]).unwrap();
        wtr.write_u8(self.text[13]).unwrap();
        wtr.write_u8(self.text[14]).unwrap();
        wtr.write_u8(self.text[15]).unwrap();
        wtr.write_u8(self.text[16]).unwrap();
        wtr.write_u8(self.text[17]).unwrap();
        wtr.write_u8(self.text[18]).unwrap();
        wtr.write_u8(self.text[19]).unwrap();
        wtr.write_u8(self.text[20]).unwrap();
        wtr.write_u8(self.text[21]).unwrap();
        wtr.write_u8(self.text[22]).unwrap();
        wtr.write_u8(self.text[23]).unwrap();
        wtr.write_u8(self.text[24]).unwrap();
        wtr.write_u8(self.text[25]).unwrap();
        wtr.write_u8(self.text[26]).unwrap();
        wtr.write_u8(self.text[27]).unwrap();
        wtr.write_u8(self.text[28]).unwrap();
        wtr.write_u8(self.text[29]).unwrap();
        wtr.write_u8(self.text[30]).unwrap();
        wtr.write_u8(self.text[31]).unwrap();
        wtr.write_u8(self.text[32]).unwrap();
        wtr.write_u8(self.text[33]).unwrap();
        wtr.write_u8(self.text[34]).unwrap();
        wtr.write_u8(self.text[35]).unwrap();
        wtr.write_u8(self.text[36]).unwrap();
        wtr.write_u8(self.text[37]).unwrap();
        wtr.write_u8(self.text[38]).unwrap();
        wtr.write_u8(self.text[39]).unwrap();
        wtr.write_u8(self.text[40]).unwrap();
        wtr.write_u8(self.text[41]).unwrap();
        wtr.write_u8(self.text[42]).unwrap();
        wtr.write_u8(self.text[43]).unwrap();
        wtr.write_u8(self.text[44]).unwrap();
        wtr.write_u8(self.text[45]).unwrap();
        wtr.write_u8(self.text[46]).unwrap();
        wtr.write_u8(self.text[47]).unwrap();
        wtr.write_u8(self.text[48]).unwrap();
        wtr.write_u8(self.text[49]).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub struct DEBUG_DATA {
    pub time_boot_ms: u32,
    pub value: f32,
    pub ind: u8,
}

impl Parsable for DEBUG_DATA {
    fn parse(payload: &[u8]) -> DEBUG_DATA {
        let mut cur = Cursor::new(payload);
        DEBUG_DATA {
            time_boot_ms: cur.read_u32::<LittleEndian>().unwrap(),
            value: cur.read_f32::<LittleEndian>().unwrap(),
            ind: cur.read_u8().unwrap(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time_boot_ms).unwrap();
        wtr.write_f32::<LittleEndian>(self.value).unwrap();
        wtr.write_u8(self.ind).unwrap();
        wtr
    }
}

#[derive(Clone, Debug)]
pub enum DkMessage {
  HEARTBEAT(HEARTBEAT_DATA),
  SYS_STATUS(SYS_STATUS_DATA),
  SYSTEM_TIME(SYSTEM_TIME_DATA),
  PING(PING_DATA),
  CHANGE_OPERATOR_CONTROL(CHANGE_OPERATOR_CONTROL_DATA),
  CHANGE_OPERATOR_CONTROL_ACK(CHANGE_OPERATOR_CONTROL_ACK_DATA),
  AUTH_KEY(AUTH_KEY_DATA),
  SET_MODE(SET_MODE_DATA),
  PARAM_REQUEST_READ(PARAM_REQUEST_READ_DATA),
  PARAM_REQUEST_LIST(PARAM_REQUEST_LIST_DATA),
  PARAM_VALUE(PARAM_VALUE_DATA),
  PARAM_SET(PARAM_SET_DATA),
  GPS_RAW_INT(GPS_RAW_INT_DATA),
  GPS_STATUS(GPS_STATUS_DATA),
  SCALED_IMU(SCALED_IMU_DATA),
  RAW_IMU(RAW_IMU_DATA),
  RAW_PRESSURE(RAW_PRESSURE_DATA),
  SCALED_PRESSURE(SCALED_PRESSURE_DATA),
  ATTITUDE(ATTITUDE_DATA),
  ATTITUDE_QUATERNION(ATTITUDE_QUATERNION_DATA),
  LOCAL_POSITION_NED(LOCAL_POSITION_NED_DATA),
  GLOBAL_POSITION_INT(GLOBAL_POSITION_INT_DATA),
  RC_CHANNELS_SCALED(RC_CHANNELS_SCALED_DATA),
  RC_CHANNELS_RAW(RC_CHANNELS_RAW_DATA),
  SERVO_OUTPUT_RAW(SERVO_OUTPUT_RAW_DATA),
  MISSION_REQUEST_PARTIAL_LIST(MISSION_REQUEST_PARTIAL_LIST_DATA),
  MISSION_WRITE_PARTIAL_LIST(MISSION_WRITE_PARTIAL_LIST_DATA),
  MISSION_ITEM(MISSION_ITEM_DATA),
  MISSION_REQUEST(MISSION_REQUEST_DATA),
  MISSION_SET_CURRENT(MISSION_SET_CURRENT_DATA),
  MISSION_CURRENT(MISSION_CURRENT_DATA),
  MISSION_REQUEST_LIST(MISSION_REQUEST_LIST_DATA),
  MISSION_COUNT(MISSION_COUNT_DATA),
  MISSION_CLEAR_ALL(MISSION_CLEAR_ALL_DATA),
  MISSION_ITEM_REACHED(MISSION_ITEM_REACHED_DATA),
  MISSION_ACK(MISSION_ACK_DATA),
  SET_GPS_GLOBAL_ORIGIN(SET_GPS_GLOBAL_ORIGIN_DATA),
  GPS_GLOBAL_ORIGIN(GPS_GLOBAL_ORIGIN_DATA),
  PARAM_MAP_RC(PARAM_MAP_RC_DATA),
  SAFETY_SET_ALLOWED_AREA(SAFETY_SET_ALLOWED_AREA_DATA),
  SAFETY_ALLOWED_AREA(SAFETY_ALLOWED_AREA_DATA),
  ATTITUDE_QUATERNION_COV(ATTITUDE_QUATERNION_COV_DATA),
  NAV_CONTROLLER_OUTPUT(NAV_CONTROLLER_OUTPUT_DATA),
  GLOBAL_POSITION_INT_COV(GLOBAL_POSITION_INT_COV_DATA),
  LOCAL_POSITION_NED_COV(LOCAL_POSITION_NED_COV_DATA),
  RC_CHANNELS(RC_CHANNELS_DATA),
  REQUEST_DATA_STREAM(REQUEST_DATA_STREAM_DATA),
  DATA_STREAM(DATA_STREAM_DATA),
  MANUAL_CONTROL(MANUAL_CONTROL_DATA),
  RC_CHANNELS_OVERRIDE(RC_CHANNELS_OVERRIDE_DATA),
  MISSION_ITEM_INT(MISSION_ITEM_INT_DATA),
  VFR_HUD(VFR_HUD_DATA),
  COMMAND_INT(COMMAND_INT_DATA),
  COMMAND_LONG(COMMAND_LONG_DATA),
  COMMAND_ACK(COMMAND_ACK_DATA),
  MANUAL_SETPOINT(MANUAL_SETPOINT_DATA),
  SET_ATTITUDE_TARGET(SET_ATTITUDE_TARGET_DATA),
  ATTITUDE_TARGET(ATTITUDE_TARGET_DATA),
  SET_POSITION_TARGET_LOCAL_NED(SET_POSITION_TARGET_LOCAL_NED_DATA),
  POSITION_TARGET_LOCAL_NED(POSITION_TARGET_LOCAL_NED_DATA),
  SET_POSITION_TARGET_GLOBAL_INT(SET_POSITION_TARGET_GLOBAL_INT_DATA),
  POSITION_TARGET_GLOBAL_INT(POSITION_TARGET_GLOBAL_INT_DATA),
  LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA),
  HIL_STATE(HIL_STATE_DATA),
  HIL_CONTROLS(HIL_CONTROLS_DATA),
  HIL_RC_INPUTS_RAW(HIL_RC_INPUTS_RAW_DATA),
  OPTICAL_FLOW(OPTICAL_FLOW_DATA),
  GLOBAL_VISION_POSITION_ESTIMATE(GLOBAL_VISION_POSITION_ESTIMATE_DATA),
  VISION_POSITION_ESTIMATE(VISION_POSITION_ESTIMATE_DATA),
  VISION_SPEED_ESTIMATE(VISION_SPEED_ESTIMATE_DATA),
  VICON_POSITION_ESTIMATE(VICON_POSITION_ESTIMATE_DATA),
  HIGHRES_IMU(HIGHRES_IMU_DATA),
  OPTICAL_FLOW_RAD(OPTICAL_FLOW_RAD_DATA),
  HIL_SENSOR(HIL_SENSOR_DATA),
  SIM_STATE(SIM_STATE_DATA),
  RADIO_STATUS(RADIO_STATUS_DATA),
  FILE_TRANSFER_PROTOCOL(FILE_TRANSFER_PROTOCOL_DATA),
  TIMESYNC(TIMESYNC_DATA),
  CAMERA_TRIGGER(CAMERA_TRIGGER_DATA),
  HIL_GPS(HIL_GPS_DATA),
  HIL_OPTICAL_FLOW(HIL_OPTICAL_FLOW_DATA),
  HIL_STATE_QUATERNION(HIL_STATE_QUATERNION_DATA),
  SCALED_IMU2(SCALED_IMU2_DATA),
  LOG_REQUEST_LIST(LOG_REQUEST_LIST_DATA),
  LOG_ENTRY(LOG_ENTRY_DATA),
  LOG_REQUEST_DATA(LOG_REQUEST_DATA_DATA),
  LOG_DATA(LOG_DATA_DATA),
  LOG_ERASE(LOG_ERASE_DATA),
  LOG_REQUEST_END(LOG_REQUEST_END_DATA),
  GPS_INJECT_DATA(GPS_INJECT_DATA_DATA),
  GPS2_RAW(GPS2_RAW_DATA),
  POWER_STATUS(POWER_STATUS_DATA),
  SERIAL_CONTROL(SERIAL_CONTROL_DATA),
  GPS_RTK(GPS_RTK_DATA),
  GPS2_RTK(GPS2_RTK_DATA),
  SCALED_IMU3(SCALED_IMU3_DATA),
  DATA_TRANSMISSION_HANDSHAKE(DATA_TRANSMISSION_HANDSHAKE_DATA),
  ENCAPSULATED_DATA(ENCAPSULATED_DATA_DATA),
  DISTANCE_SENSOR(DISTANCE_SENSOR_DATA),
  TERRAIN_REQUEST(TERRAIN_REQUEST_DATA),
  TERRAIN_DATA(TERRAIN_DATA_DATA),
  TERRAIN_CHECK(TERRAIN_CHECK_DATA),
  TERRAIN_REPORT(TERRAIN_REPORT_DATA),
  SCALED_PRESSURE2(SCALED_PRESSURE2_DATA),
  ATT_POS_MOCAP(ATT_POS_MOCAP_DATA),
  SET_ACTUATOR_CONTROL_TARGET(SET_ACTUATOR_CONTROL_TARGET_DATA),
  ACTUATOR_CONTROL_TARGET(ACTUATOR_CONTROL_TARGET_DATA),
  ALTITUDE(ALTITUDE_DATA),
  RESOURCE_REQUEST(RESOURCE_REQUEST_DATA),
  SCALED_PRESSURE3(SCALED_PRESSURE3_DATA),
  CONTROL_SYSTEM_STATE(CONTROL_SYSTEM_STATE_DATA),
  BATTERY_STATUS(BATTERY_STATUS_DATA),
  AUTOPILOT_VERSION(AUTOPILOT_VERSION_DATA),
  LANDING_TARGET(LANDING_TARGET_DATA),
  VIBRATION(VIBRATION_DATA),
  HOME_POSITION(HOME_POSITION_DATA),
  SET_HOME_POSITION(SET_HOME_POSITION_DATA),
  MESSAGE_INTERVAL(MESSAGE_INTERVAL_DATA),
  EXTENDED_SYS_STATE(EXTENDED_SYS_STATE_DATA),
  ADSB_VEHICLE(ADSB_VEHICLE_DATA),
  V2_EXTENSION(V2_EXTENSION_DATA),
  MEMORY_VECT(MEMORY_VECT_DATA),
  DEBUG_VECT(DEBUG_VECT_DATA),
  NAMED_VALUE_FLOAT(NAMED_VALUE_FLOAT_DATA),
  NAMED_VALUE_INT(NAMED_VALUE_INT_DATA),
  STATUSTEXT(STATUSTEXT_DATA),
  DEBUG(DEBUG_DATA),
}

impl DkMessage {
    pub fn parse(id: u8, payload: &[u8]) -> Option<DkMessage> {
        match id {
            0 => Some(DkMessage::HEARTBEAT(HEARTBEAT_DATA::parse(payload))),
            1 => Some(DkMessage::SYS_STATUS(SYS_STATUS_DATA::parse(payload))),
            2 => Some(DkMessage::SYSTEM_TIME(SYSTEM_TIME_DATA::parse(payload))),
            4 => Some(DkMessage::PING(PING_DATA::parse(payload))),
            5 => Some(DkMessage::CHANGE_OPERATOR_CONTROL(CHANGE_OPERATOR_CONTROL_DATA::parse(payload))),
            6 => Some(DkMessage::CHANGE_OPERATOR_CONTROL_ACK(CHANGE_OPERATOR_CONTROL_ACK_DATA::parse(payload))),
            7 => Some(DkMessage::AUTH_KEY(AUTH_KEY_DATA::parse(payload))),
            11 => Some(DkMessage::SET_MODE(SET_MODE_DATA::parse(payload))),
            20 => Some(DkMessage::PARAM_REQUEST_READ(PARAM_REQUEST_READ_DATA::parse(payload))),
            21 => Some(DkMessage::PARAM_REQUEST_LIST(PARAM_REQUEST_LIST_DATA::parse(payload))),
            22 => Some(DkMessage::PARAM_VALUE(PARAM_VALUE_DATA::parse(payload))),
            23 => Some(DkMessage::PARAM_SET(PARAM_SET_DATA::parse(payload))),
            24 => Some(DkMessage::GPS_RAW_INT(GPS_RAW_INT_DATA::parse(payload))),
            25 => Some(DkMessage::GPS_STATUS(GPS_STATUS_DATA::parse(payload))),
            26 => Some(DkMessage::SCALED_IMU(SCALED_IMU_DATA::parse(payload))),
            27 => Some(DkMessage::RAW_IMU(RAW_IMU_DATA::parse(payload))),
            28 => Some(DkMessage::RAW_PRESSURE(RAW_PRESSURE_DATA::parse(payload))),
            29 => Some(DkMessage::SCALED_PRESSURE(SCALED_PRESSURE_DATA::parse(payload))),
            30 => Some(DkMessage::ATTITUDE(ATTITUDE_DATA::parse(payload))),
            31 => Some(DkMessage::ATTITUDE_QUATERNION(ATTITUDE_QUATERNION_DATA::parse(payload))),
            32 => Some(DkMessage::LOCAL_POSITION_NED(LOCAL_POSITION_NED_DATA::parse(payload))),
            33 => Some(DkMessage::GLOBAL_POSITION_INT(GLOBAL_POSITION_INT_DATA::parse(payload))),
            34 => Some(DkMessage::RC_CHANNELS_SCALED(RC_CHANNELS_SCALED_DATA::parse(payload))),
            35 => Some(DkMessage::RC_CHANNELS_RAW(RC_CHANNELS_RAW_DATA::parse(payload))),
            36 => Some(DkMessage::SERVO_OUTPUT_RAW(SERVO_OUTPUT_RAW_DATA::parse(payload))),
            37 => Some(DkMessage::MISSION_REQUEST_PARTIAL_LIST(MISSION_REQUEST_PARTIAL_LIST_DATA::parse(payload))),
            38 => Some(DkMessage::MISSION_WRITE_PARTIAL_LIST(MISSION_WRITE_PARTIAL_LIST_DATA::parse(payload))),
            39 => Some(DkMessage::MISSION_ITEM(MISSION_ITEM_DATA::parse(payload))),
            40 => Some(DkMessage::MISSION_REQUEST(MISSION_REQUEST_DATA::parse(payload))),
            41 => Some(DkMessage::MISSION_SET_CURRENT(MISSION_SET_CURRENT_DATA::parse(payload))),
            42 => Some(DkMessage::MISSION_CURRENT(MISSION_CURRENT_DATA::parse(payload))),
            43 => Some(DkMessage::MISSION_REQUEST_LIST(MISSION_REQUEST_LIST_DATA::parse(payload))),
            44 => Some(DkMessage::MISSION_COUNT(MISSION_COUNT_DATA::parse(payload))),
            45 => Some(DkMessage::MISSION_CLEAR_ALL(MISSION_CLEAR_ALL_DATA::parse(payload))),
            46 => Some(DkMessage::MISSION_ITEM_REACHED(MISSION_ITEM_REACHED_DATA::parse(payload))),
            47 => Some(DkMessage::MISSION_ACK(MISSION_ACK_DATA::parse(payload))),
            48 => Some(DkMessage::SET_GPS_GLOBAL_ORIGIN(SET_GPS_GLOBAL_ORIGIN_DATA::parse(payload))),
            49 => Some(DkMessage::GPS_GLOBAL_ORIGIN(GPS_GLOBAL_ORIGIN_DATA::parse(payload))),
            50 => Some(DkMessage::PARAM_MAP_RC(PARAM_MAP_RC_DATA::parse(payload))),
            54 => Some(DkMessage::SAFETY_SET_ALLOWED_AREA(SAFETY_SET_ALLOWED_AREA_DATA::parse(payload))),
            55 => Some(DkMessage::SAFETY_ALLOWED_AREA(SAFETY_ALLOWED_AREA_DATA::parse(payload))),
            61 => Some(DkMessage::ATTITUDE_QUATERNION_COV(ATTITUDE_QUATERNION_COV_DATA::parse(payload))),
            62 => Some(DkMessage::NAV_CONTROLLER_OUTPUT(NAV_CONTROLLER_OUTPUT_DATA::parse(payload))),
            63 => Some(DkMessage::GLOBAL_POSITION_INT_COV(GLOBAL_POSITION_INT_COV_DATA::parse(payload))),
            64 => Some(DkMessage::LOCAL_POSITION_NED_COV(LOCAL_POSITION_NED_COV_DATA::parse(payload))),
            65 => Some(DkMessage::RC_CHANNELS(RC_CHANNELS_DATA::parse(payload))),
            66 => Some(DkMessage::REQUEST_DATA_STREAM(REQUEST_DATA_STREAM_DATA::parse(payload))),
            67 => Some(DkMessage::DATA_STREAM(DATA_STREAM_DATA::parse(payload))),
            69 => Some(DkMessage::MANUAL_CONTROL(MANUAL_CONTROL_DATA::parse(payload))),
            70 => Some(DkMessage::RC_CHANNELS_OVERRIDE(RC_CHANNELS_OVERRIDE_DATA::parse(payload))),
            73 => Some(DkMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA::parse(payload))),
            74 => Some(DkMessage::VFR_HUD(VFR_HUD_DATA::parse(payload))),
            75 => Some(DkMessage::COMMAND_INT(COMMAND_INT_DATA::parse(payload))),
            76 => Some(DkMessage::COMMAND_LONG(COMMAND_LONG_DATA::parse(payload))),
            77 => Some(DkMessage::COMMAND_ACK(COMMAND_ACK_DATA::parse(payload))),
            81 => Some(DkMessage::MANUAL_SETPOINT(MANUAL_SETPOINT_DATA::parse(payload))),
            82 => Some(DkMessage::SET_ATTITUDE_TARGET(SET_ATTITUDE_TARGET_DATA::parse(payload))),
            83 => Some(DkMessage::ATTITUDE_TARGET(ATTITUDE_TARGET_DATA::parse(payload))),
            84 => Some(DkMessage::SET_POSITION_TARGET_LOCAL_NED(SET_POSITION_TARGET_LOCAL_NED_DATA::parse(payload))),
            85 => Some(DkMessage::POSITION_TARGET_LOCAL_NED(POSITION_TARGET_LOCAL_NED_DATA::parse(payload))),
            86 => Some(DkMessage::SET_POSITION_TARGET_GLOBAL_INT(SET_POSITION_TARGET_GLOBAL_INT_DATA::parse(payload))),
            87 => Some(DkMessage::POSITION_TARGET_GLOBAL_INT(POSITION_TARGET_GLOBAL_INT_DATA::parse(payload))),
            89 => Some(DkMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA::parse(payload))),
            90 => Some(DkMessage::HIL_STATE(HIL_STATE_DATA::parse(payload))),
            91 => Some(DkMessage::HIL_CONTROLS(HIL_CONTROLS_DATA::parse(payload))),
            92 => Some(DkMessage::HIL_RC_INPUTS_RAW(HIL_RC_INPUTS_RAW_DATA::parse(payload))),
            100 => Some(DkMessage::OPTICAL_FLOW(OPTICAL_FLOW_DATA::parse(payload))),
            101 => Some(DkMessage::GLOBAL_VISION_POSITION_ESTIMATE(GLOBAL_VISION_POSITION_ESTIMATE_DATA::parse(payload))),
            102 => Some(DkMessage::VISION_POSITION_ESTIMATE(VISION_POSITION_ESTIMATE_DATA::parse(payload))),
            103 => Some(DkMessage::VISION_SPEED_ESTIMATE(VISION_SPEED_ESTIMATE_DATA::parse(payload))),
            104 => Some(DkMessage::VICON_POSITION_ESTIMATE(VICON_POSITION_ESTIMATE_DATA::parse(payload))),
            105 => Some(DkMessage::HIGHRES_IMU(HIGHRES_IMU_DATA::parse(payload))),
            106 => Some(DkMessage::OPTICAL_FLOW_RAD(OPTICAL_FLOW_RAD_DATA::parse(payload))),
            107 => Some(DkMessage::HIL_SENSOR(HIL_SENSOR_DATA::parse(payload))),
            108 => Some(DkMessage::SIM_STATE(SIM_STATE_DATA::parse(payload))),
            109 => Some(DkMessage::RADIO_STATUS(RADIO_STATUS_DATA::parse(payload))),
            110 => Some(DkMessage::FILE_TRANSFER_PROTOCOL(FILE_TRANSFER_PROTOCOL_DATA::parse(payload))),
            111 => Some(DkMessage::TIMESYNC(TIMESYNC_DATA::parse(payload))),
            112 => Some(DkMessage::CAMERA_TRIGGER(CAMERA_TRIGGER_DATA::parse(payload))),
            113 => Some(DkMessage::HIL_GPS(HIL_GPS_DATA::parse(payload))),
            114 => Some(DkMessage::HIL_OPTICAL_FLOW(HIL_OPTICAL_FLOW_DATA::parse(payload))),
            115 => Some(DkMessage::HIL_STATE_QUATERNION(HIL_STATE_QUATERNION_DATA::parse(payload))),
            116 => Some(DkMessage::SCALED_IMU2(SCALED_IMU2_DATA::parse(payload))),
            117 => Some(DkMessage::LOG_REQUEST_LIST(LOG_REQUEST_LIST_DATA::parse(payload))),
            118 => Some(DkMessage::LOG_ENTRY(LOG_ENTRY_DATA::parse(payload))),
            119 => Some(DkMessage::LOG_REQUEST_DATA(LOG_REQUEST_DATA_DATA::parse(payload))),
            120 => Some(DkMessage::LOG_DATA(LOG_DATA_DATA::parse(payload))),
            121 => Some(DkMessage::LOG_ERASE(LOG_ERASE_DATA::parse(payload))),
            122 => Some(DkMessage::LOG_REQUEST_END(LOG_REQUEST_END_DATA::parse(payload))),
            123 => Some(DkMessage::GPS_INJECT_DATA(GPS_INJECT_DATA_DATA::parse(payload))),
            124 => Some(DkMessage::GPS2_RAW(GPS2_RAW_DATA::parse(payload))),
            125 => Some(DkMessage::POWER_STATUS(POWER_STATUS_DATA::parse(payload))),
            126 => Some(DkMessage::SERIAL_CONTROL(SERIAL_CONTROL_DATA::parse(payload))),
            127 => Some(DkMessage::GPS_RTK(GPS_RTK_DATA::parse(payload))),
            128 => Some(DkMessage::GPS2_RTK(GPS2_RTK_DATA::parse(payload))),
            129 => Some(DkMessage::SCALED_IMU3(SCALED_IMU3_DATA::parse(payload))),
            130 => Some(DkMessage::DATA_TRANSMISSION_HANDSHAKE(DATA_TRANSMISSION_HANDSHAKE_DATA::parse(payload))),
            131 => Some(DkMessage::ENCAPSULATED_DATA(ENCAPSULATED_DATA_DATA::parse(payload))),
            132 => Some(DkMessage::DISTANCE_SENSOR(DISTANCE_SENSOR_DATA::parse(payload))),
            133 => Some(DkMessage::TERRAIN_REQUEST(TERRAIN_REQUEST_DATA::parse(payload))),
            134 => Some(DkMessage::TERRAIN_DATA(TERRAIN_DATA_DATA::parse(payload))),
            135 => Some(DkMessage::TERRAIN_CHECK(TERRAIN_CHECK_DATA::parse(payload))),
            136 => Some(DkMessage::TERRAIN_REPORT(TERRAIN_REPORT_DATA::parse(payload))),
            137 => Some(DkMessage::SCALED_PRESSURE2(SCALED_PRESSURE2_DATA::parse(payload))),
            138 => Some(DkMessage::ATT_POS_MOCAP(ATT_POS_MOCAP_DATA::parse(payload))),
            139 => Some(DkMessage::SET_ACTUATOR_CONTROL_TARGET(SET_ACTUATOR_CONTROL_TARGET_DATA::parse(payload))),
            140 => Some(DkMessage::ACTUATOR_CONTROL_TARGET(ACTUATOR_CONTROL_TARGET_DATA::parse(payload))),
            141 => Some(DkMessage::ALTITUDE(ALTITUDE_DATA::parse(payload))),
            142 => Some(DkMessage::RESOURCE_REQUEST(RESOURCE_REQUEST_DATA::parse(payload))),
            143 => Some(DkMessage::SCALED_PRESSURE3(SCALED_PRESSURE3_DATA::parse(payload))),
            146 => Some(DkMessage::CONTROL_SYSTEM_STATE(CONTROL_SYSTEM_STATE_DATA::parse(payload))),
            147 => Some(DkMessage::BATTERY_STATUS(BATTERY_STATUS_DATA::parse(payload))),
            148 => Some(DkMessage::AUTOPILOT_VERSION(AUTOPILOT_VERSION_DATA::parse(payload))),
            149 => Some(DkMessage::LANDING_TARGET(LANDING_TARGET_DATA::parse(payload))),
            241 => Some(DkMessage::VIBRATION(VIBRATION_DATA::parse(payload))),
            242 => Some(DkMessage::HOME_POSITION(HOME_POSITION_DATA::parse(payload))),
            243 => Some(DkMessage::SET_HOME_POSITION(SET_HOME_POSITION_DATA::parse(payload))),
            244 => Some(DkMessage::MESSAGE_INTERVAL(MESSAGE_INTERVAL_DATA::parse(payload))),
            245 => Some(DkMessage::EXTENDED_SYS_STATE(EXTENDED_SYS_STATE_DATA::parse(payload))),
            246 => Some(DkMessage::ADSB_VEHICLE(ADSB_VEHICLE_DATA::parse(payload))),
            248 => Some(DkMessage::V2_EXTENSION(V2_EXTENSION_DATA::parse(payload))),
            249 => Some(DkMessage::MEMORY_VECT(MEMORY_VECT_DATA::parse(payload))),
            250 => Some(DkMessage::DEBUG_VECT(DEBUG_VECT_DATA::parse(payload))),
            251 => Some(DkMessage::NAMED_VALUE_FLOAT(NAMED_VALUE_FLOAT_DATA::parse(payload))),
            252 => Some(DkMessage::NAMED_VALUE_INT(NAMED_VALUE_INT_DATA::parse(payload))),
            253 => Some(DkMessage::STATUSTEXT(STATUSTEXT_DATA::parse(payload))),
            254 => Some(DkMessage::DEBUG(DEBUG_DATA::parse(payload))),
            _ => None,
        }
    }

    pub fn message_id(&self) -> u8 {
        match self {
            &DkMessage::HEARTBEAT(..) => 0,
            &DkMessage::SYS_STATUS(..) => 1,
            &DkMessage::SYSTEM_TIME(..) => 2,
            &DkMessage::PING(..) => 4,
            &DkMessage::CHANGE_OPERATOR_CONTROL(..) => 5,
            &DkMessage::CHANGE_OPERATOR_CONTROL_ACK(..) => 6,
            &DkMessage::AUTH_KEY(..) => 7,
            &DkMessage::SET_MODE(..) => 11,
            &DkMessage::PARAM_REQUEST_READ(..) => 20,
            &DkMessage::PARAM_REQUEST_LIST(..) => 21,
            &DkMessage::PARAM_VALUE(..) => 22,
            &DkMessage::PARAM_SET(..) => 23,
            &DkMessage::GPS_RAW_INT(..) => 24,
            &DkMessage::GPS_STATUS(..) => 25,
            &DkMessage::SCALED_IMU(..) => 26,
            &DkMessage::RAW_IMU(..) => 27,
            &DkMessage::RAW_PRESSURE(..) => 28,
            &DkMessage::SCALED_PRESSURE(..) => 29,
            &DkMessage::ATTITUDE(..) => 30,
            &DkMessage::ATTITUDE_QUATERNION(..) => 31,
            &DkMessage::LOCAL_POSITION_NED(..) => 32,
            &DkMessage::GLOBAL_POSITION_INT(..) => 33,
            &DkMessage::RC_CHANNELS_SCALED(..) => 34,
            &DkMessage::RC_CHANNELS_RAW(..) => 35,
            &DkMessage::SERVO_OUTPUT_RAW(..) => 36,
            &DkMessage::MISSION_REQUEST_PARTIAL_LIST(..) => 37,
            &DkMessage::MISSION_WRITE_PARTIAL_LIST(..) => 38,
            &DkMessage::MISSION_ITEM(..) => 39,
            &DkMessage::MISSION_REQUEST(..) => 40,
            &DkMessage::MISSION_SET_CURRENT(..) => 41,
            &DkMessage::MISSION_CURRENT(..) => 42,
            &DkMessage::MISSION_REQUEST_LIST(..) => 43,
            &DkMessage::MISSION_COUNT(..) => 44,
            &DkMessage::MISSION_CLEAR_ALL(..) => 45,
            &DkMessage::MISSION_ITEM_REACHED(..) => 46,
            &DkMessage::MISSION_ACK(..) => 47,
            &DkMessage::SET_GPS_GLOBAL_ORIGIN(..) => 48,
            &DkMessage::GPS_GLOBAL_ORIGIN(..) => 49,
            &DkMessage::PARAM_MAP_RC(..) => 50,
            &DkMessage::SAFETY_SET_ALLOWED_AREA(..) => 54,
            &DkMessage::SAFETY_ALLOWED_AREA(..) => 55,
            &DkMessage::ATTITUDE_QUATERNION_COV(..) => 61,
            &DkMessage::NAV_CONTROLLER_OUTPUT(..) => 62,
            &DkMessage::GLOBAL_POSITION_INT_COV(..) => 63,
            &DkMessage::LOCAL_POSITION_NED_COV(..) => 64,
            &DkMessage::RC_CHANNELS(..) => 65,
            &DkMessage::REQUEST_DATA_STREAM(..) => 66,
            &DkMessage::DATA_STREAM(..) => 67,
            &DkMessage::MANUAL_CONTROL(..) => 69,
            &DkMessage::RC_CHANNELS_OVERRIDE(..) => 70,
            &DkMessage::MISSION_ITEM_INT(..) => 73,
            &DkMessage::VFR_HUD(..) => 74,
            &DkMessage::COMMAND_INT(..) => 75,
            &DkMessage::COMMAND_LONG(..) => 76,
            &DkMessage::COMMAND_ACK(..) => 77,
            &DkMessage::MANUAL_SETPOINT(..) => 81,
            &DkMessage::SET_ATTITUDE_TARGET(..) => 82,
            &DkMessage::ATTITUDE_TARGET(..) => 83,
            &DkMessage::SET_POSITION_TARGET_LOCAL_NED(..) => 84,
            &DkMessage::POSITION_TARGET_LOCAL_NED(..) => 85,
            &DkMessage::SET_POSITION_TARGET_GLOBAL_INT(..) => 86,
            &DkMessage::POSITION_TARGET_GLOBAL_INT(..) => 87,
            &DkMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(..) => 89,
            &DkMessage::HIL_STATE(..) => 90,
            &DkMessage::HIL_CONTROLS(..) => 91,
            &DkMessage::HIL_RC_INPUTS_RAW(..) => 92,
            &DkMessage::OPTICAL_FLOW(..) => 100,
            &DkMessage::GLOBAL_VISION_POSITION_ESTIMATE(..) => 101,
            &DkMessage::VISION_POSITION_ESTIMATE(..) => 102,
            &DkMessage::VISION_SPEED_ESTIMATE(..) => 103,
            &DkMessage::VICON_POSITION_ESTIMATE(..) => 104,
            &DkMessage::HIGHRES_IMU(..) => 105,
            &DkMessage::OPTICAL_FLOW_RAD(..) => 106,
            &DkMessage::HIL_SENSOR(..) => 107,
            &DkMessage::SIM_STATE(..) => 108,
            &DkMessage::RADIO_STATUS(..) => 109,
            &DkMessage::FILE_TRANSFER_PROTOCOL(..) => 110,
            &DkMessage::TIMESYNC(..) => 111,
            &DkMessage::CAMERA_TRIGGER(..) => 112,
            &DkMessage::HIL_GPS(..) => 113,
            &DkMessage::HIL_OPTICAL_FLOW(..) => 114,
            &DkMessage::HIL_STATE_QUATERNION(..) => 115,
            &DkMessage::SCALED_IMU2(..) => 116,
            &DkMessage::LOG_REQUEST_LIST(..) => 117,
            &DkMessage::LOG_ENTRY(..) => 118,
            &DkMessage::LOG_REQUEST_DATA(..) => 119,
            &DkMessage::LOG_DATA(..) => 120,
            &DkMessage::LOG_ERASE(..) => 121,
            &DkMessage::LOG_REQUEST_END(..) => 122,
            &DkMessage::GPS_INJECT_DATA(..) => 123,
            &DkMessage::GPS2_RAW(..) => 124,
            &DkMessage::POWER_STATUS(..) => 125,
            &DkMessage::SERIAL_CONTROL(..) => 126,
            &DkMessage::GPS_RTK(..) => 127,
            &DkMessage::GPS2_RTK(..) => 128,
            &DkMessage::SCALED_IMU3(..) => 129,
            &DkMessage::DATA_TRANSMISSION_HANDSHAKE(..) => 130,
            &DkMessage::ENCAPSULATED_DATA(..) => 131,
            &DkMessage::DISTANCE_SENSOR(..) => 132,
            &DkMessage::TERRAIN_REQUEST(..) => 133,
            &DkMessage::TERRAIN_DATA(..) => 134,
            &DkMessage::TERRAIN_CHECK(..) => 135,
            &DkMessage::TERRAIN_REPORT(..) => 136,
            &DkMessage::SCALED_PRESSURE2(..) => 137,
            &DkMessage::ATT_POS_MOCAP(..) => 138,
            &DkMessage::SET_ACTUATOR_CONTROL_TARGET(..) => 139,
            &DkMessage::ACTUATOR_CONTROL_TARGET(..) => 140,
            &DkMessage::ALTITUDE(..) => 141,
            &DkMessage::RESOURCE_REQUEST(..) => 142,
            &DkMessage::SCALED_PRESSURE3(..) => 143,
            &DkMessage::CONTROL_SYSTEM_STATE(..) => 146,
            &DkMessage::BATTERY_STATUS(..) => 147,
            &DkMessage::AUTOPILOT_VERSION(..) => 148,
            &DkMessage::LANDING_TARGET(..) => 149,
            &DkMessage::VIBRATION(..) => 241,
            &DkMessage::HOME_POSITION(..) => 242,
            &DkMessage::SET_HOME_POSITION(..) => 243,
            &DkMessage::MESSAGE_INTERVAL(..) => 244,
            &DkMessage::EXTENDED_SYS_STATE(..) => 245,
            &DkMessage::ADSB_VEHICLE(..) => 246,
            &DkMessage::V2_EXTENSION(..) => 248,
            &DkMessage::MEMORY_VECT(..) => 249,
            &DkMessage::DEBUG_VECT(..) => 250,
            &DkMessage::NAMED_VALUE_FLOAT(..) => 251,
            &DkMessage::NAMED_VALUE_INT(..) => 252,
            &DkMessage::STATUSTEXT(..) => 253,
            &DkMessage::DEBUG(..) => 254,
        }
    }

    pub fn extra_crc(id: u8) -> u8 {
        match id {
            0 => 50,
            1 => 124,
            2 => 137,
            4 => 237,
            5 => 217,
            6 => 104,
            7 => 119,
            11 => 89,
            20 => 214,
            21 => 159,
            22 => 220,
            23 => 168,
            24 => 24,
            25 => 23,
            26 => 170,
            27 => 144,
            28 => 67,
            29 => 115,
            30 => 39,
            31 => 246,
            32 => 185,
            33 => 104,
            34 => 237,
            35 => 244,
            36 => 222,
            37 => 212,
            38 => 9,
            39 => 254,
            40 => 230,
            41 => 28,
            42 => 28,
            43 => 132,
            44 => 221,
            45 => 232,
            46 => 11,
            47 => 153,
            48 => 41,
            49 => 39,
            50 => 78,
            54 => 15,
            55 => 3,
            61 => 153,
            62 => 183,
            63 => 51,
            64 => 59,
            65 => 118,
            66 => 148,
            67 => 21,
            69 => 243,
            70 => 124,
            73 => 38,
            74 => 20,
            75 => 158,
            76 => 152,
            77 => 143,
            81 => 106,
            82 => 49,
            83 => 22,
            84 => 143,
            85 => 140,
            86 => 5,
            87 => 150,
            89 => 231,
            90 => 183,
            91 => 63,
            92 => 54,
            100 => 175,
            101 => 102,
            102 => 158,
            103 => 208,
            104 => 56,
            105 => 93,
            106 => 138,
            107 => 108,
            108 => 32,
            109 => 185,
            110 => 84,
            111 => 34,
            112 => 174,
            113 => 124,
            114 => 237,
            115 => 4,
            116 => 76,
            117 => 128,
            118 => 56,
            119 => 116,
            120 => 134,
            121 => 237,
            122 => 203,
            123 => 250,
            124 => 87,
            125 => 203,
            126 => 220,
            127 => 25,
            128 => 226,
            129 => 46,
            130 => 29,
            131 => 223,
            132 => 85,
            133 => 6,
            134 => 229,
            135 => 203,
            136 => 1,
            137 => 195,
            138 => 109,
            139 => 168,
            140 => 181,
            141 => 47,
            142 => 72,
            143 => 131,
            146 => 103,
            147 => 154,
            148 => 178,
            149 => 200,
            241 => 90,
            242 => 104,
            243 => 85,
            244 => 95,
            245 => 130,
            246 => 210,
            248 => 8,
            249 => 204,
            250 => 49,
            251 => 170,
            252 => 44,
            253 => 83,
            254 => 46,
            _ => 0,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self {
            &DkMessage::HEARTBEAT(ref body) => body.serialize(),
            &DkMessage::SYS_STATUS(ref body) => body.serialize(),
            &DkMessage::SYSTEM_TIME(ref body) => body.serialize(),
            &DkMessage::PING(ref body) => body.serialize(),
            &DkMessage::CHANGE_OPERATOR_CONTROL(ref body) => body.serialize(),
            &DkMessage::CHANGE_OPERATOR_CONTROL_ACK(ref body) => body.serialize(),
            &DkMessage::AUTH_KEY(ref body) => body.serialize(),
            &DkMessage::SET_MODE(ref body) => body.serialize(),
            &DkMessage::PARAM_REQUEST_READ(ref body) => body.serialize(),
            &DkMessage::PARAM_REQUEST_LIST(ref body) => body.serialize(),
            &DkMessage::PARAM_VALUE(ref body) => body.serialize(),
            &DkMessage::PARAM_SET(ref body) => body.serialize(),
            &DkMessage::GPS_RAW_INT(ref body) => body.serialize(),
            &DkMessage::GPS_STATUS(ref body) => body.serialize(),
            &DkMessage::SCALED_IMU(ref body) => body.serialize(),
            &DkMessage::RAW_IMU(ref body) => body.serialize(),
            &DkMessage::RAW_PRESSURE(ref body) => body.serialize(),
            &DkMessage::SCALED_PRESSURE(ref body) => body.serialize(),
            &DkMessage::ATTITUDE(ref body) => body.serialize(),
            &DkMessage::ATTITUDE_QUATERNION(ref body) => body.serialize(),
            &DkMessage::LOCAL_POSITION_NED(ref body) => body.serialize(),
            &DkMessage::GLOBAL_POSITION_INT(ref body) => body.serialize(),
            &DkMessage::RC_CHANNELS_SCALED(ref body) => body.serialize(),
            &DkMessage::RC_CHANNELS_RAW(ref body) => body.serialize(),
            &DkMessage::SERVO_OUTPUT_RAW(ref body) => body.serialize(),
            &DkMessage::MISSION_REQUEST_PARTIAL_LIST(ref body) => body.serialize(),
            &DkMessage::MISSION_WRITE_PARTIAL_LIST(ref body) => body.serialize(),
            &DkMessage::MISSION_ITEM(ref body) => body.serialize(),
            &DkMessage::MISSION_REQUEST(ref body) => body.serialize(),
            &DkMessage::MISSION_SET_CURRENT(ref body) => body.serialize(),
            &DkMessage::MISSION_CURRENT(ref body) => body.serialize(),
            &DkMessage::MISSION_REQUEST_LIST(ref body) => body.serialize(),
            &DkMessage::MISSION_COUNT(ref body) => body.serialize(),
            &DkMessage::MISSION_CLEAR_ALL(ref body) => body.serialize(),
            &DkMessage::MISSION_ITEM_REACHED(ref body) => body.serialize(),
            &DkMessage::MISSION_ACK(ref body) => body.serialize(),
            &DkMessage::SET_GPS_GLOBAL_ORIGIN(ref body) => body.serialize(),
            &DkMessage::GPS_GLOBAL_ORIGIN(ref body) => body.serialize(),
            &DkMessage::PARAM_MAP_RC(ref body) => body.serialize(),
            &DkMessage::SAFETY_SET_ALLOWED_AREA(ref body) => body.serialize(),
            &DkMessage::SAFETY_ALLOWED_AREA(ref body) => body.serialize(),
            &DkMessage::ATTITUDE_QUATERNION_COV(ref body) => body.serialize(),
            &DkMessage::NAV_CONTROLLER_OUTPUT(ref body) => body.serialize(),
            &DkMessage::GLOBAL_POSITION_INT_COV(ref body) => body.serialize(),
            &DkMessage::LOCAL_POSITION_NED_COV(ref body) => body.serialize(),
            &DkMessage::RC_CHANNELS(ref body) => body.serialize(),
            &DkMessage::REQUEST_DATA_STREAM(ref body) => body.serialize(),
            &DkMessage::DATA_STREAM(ref body) => body.serialize(),
            &DkMessage::MANUAL_CONTROL(ref body) => body.serialize(),
            &DkMessage::RC_CHANNELS_OVERRIDE(ref body) => body.serialize(),
            &DkMessage::MISSION_ITEM_INT(ref body) => body.serialize(),
            &DkMessage::VFR_HUD(ref body) => body.serialize(),
            &DkMessage::COMMAND_INT(ref body) => body.serialize(),
            &DkMessage::COMMAND_LONG(ref body) => body.serialize(),
            &DkMessage::COMMAND_ACK(ref body) => body.serialize(),
            &DkMessage::MANUAL_SETPOINT(ref body) => body.serialize(),
            &DkMessage::SET_ATTITUDE_TARGET(ref body) => body.serialize(),
            &DkMessage::ATTITUDE_TARGET(ref body) => body.serialize(),
            &DkMessage::SET_POSITION_TARGET_LOCAL_NED(ref body) => body.serialize(),
            &DkMessage::POSITION_TARGET_LOCAL_NED(ref body) => body.serialize(),
            &DkMessage::SET_POSITION_TARGET_GLOBAL_INT(ref body) => body.serialize(),
            &DkMessage::POSITION_TARGET_GLOBAL_INT(ref body) => body.serialize(),
            &DkMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(ref body) => body.serialize(),
            &DkMessage::HIL_STATE(ref body) => body.serialize(),
            &DkMessage::HIL_CONTROLS(ref body) => body.serialize(),
            &DkMessage::HIL_RC_INPUTS_RAW(ref body) => body.serialize(),
            &DkMessage::OPTICAL_FLOW(ref body) => body.serialize(),
            &DkMessage::GLOBAL_VISION_POSITION_ESTIMATE(ref body) => body.serialize(),
            &DkMessage::VISION_POSITION_ESTIMATE(ref body) => body.serialize(),
            &DkMessage::VISION_SPEED_ESTIMATE(ref body) => body.serialize(),
            &DkMessage::VICON_POSITION_ESTIMATE(ref body) => body.serialize(),
            &DkMessage::HIGHRES_IMU(ref body) => body.serialize(),
            &DkMessage::OPTICAL_FLOW_RAD(ref body) => body.serialize(),
            &DkMessage::HIL_SENSOR(ref body) => body.serialize(),
            &DkMessage::SIM_STATE(ref body) => body.serialize(),
            &DkMessage::RADIO_STATUS(ref body) => body.serialize(),
            &DkMessage::FILE_TRANSFER_PROTOCOL(ref body) => body.serialize(),
            &DkMessage::TIMESYNC(ref body) => body.serialize(),
            &DkMessage::CAMERA_TRIGGER(ref body) => body.serialize(),
            &DkMessage::HIL_GPS(ref body) => body.serialize(),
            &DkMessage::HIL_OPTICAL_FLOW(ref body) => body.serialize(),
            &DkMessage::HIL_STATE_QUATERNION(ref body) => body.serialize(),
            &DkMessage::SCALED_IMU2(ref body) => body.serialize(),
            &DkMessage::LOG_REQUEST_LIST(ref body) => body.serialize(),
            &DkMessage::LOG_ENTRY(ref body) => body.serialize(),
            &DkMessage::LOG_REQUEST_DATA(ref body) => body.serialize(),
            &DkMessage::LOG_DATA(ref body) => body.serialize(),
            &DkMessage::LOG_ERASE(ref body) => body.serialize(),
            &DkMessage::LOG_REQUEST_END(ref body) => body.serialize(),
            &DkMessage::GPS_INJECT_DATA(ref body) => body.serialize(),
            &DkMessage::GPS2_RAW(ref body) => body.serialize(),
            &DkMessage::POWER_STATUS(ref body) => body.serialize(),
            &DkMessage::SERIAL_CONTROL(ref body) => body.serialize(),
            &DkMessage::GPS_RTK(ref body) => body.serialize(),
            &DkMessage::GPS2_RTK(ref body) => body.serialize(),
            &DkMessage::SCALED_IMU3(ref body) => body.serialize(),
            &DkMessage::DATA_TRANSMISSION_HANDSHAKE(ref body) => body.serialize(),
            &DkMessage::ENCAPSULATED_DATA(ref body) => body.serialize(),
            &DkMessage::DISTANCE_SENSOR(ref body) => body.serialize(),
            &DkMessage::TERRAIN_REQUEST(ref body) => body.serialize(),
            &DkMessage::TERRAIN_DATA(ref body) => body.serialize(),
            &DkMessage::TERRAIN_CHECK(ref body) => body.serialize(),
            &DkMessage::TERRAIN_REPORT(ref body) => body.serialize(),
            &DkMessage::SCALED_PRESSURE2(ref body) => body.serialize(),
            &DkMessage::ATT_POS_MOCAP(ref body) => body.serialize(),
            &DkMessage::SET_ACTUATOR_CONTROL_TARGET(ref body) => body.serialize(),
            &DkMessage::ACTUATOR_CONTROL_TARGET(ref body) => body.serialize(),
            &DkMessage::ALTITUDE(ref body) => body.serialize(),
            &DkMessage::RESOURCE_REQUEST(ref body) => body.serialize(),
            &DkMessage::SCALED_PRESSURE3(ref body) => body.serialize(),
            &DkMessage::CONTROL_SYSTEM_STATE(ref body) => body.serialize(),
            &DkMessage::BATTERY_STATUS(ref body) => body.serialize(),
            &DkMessage::AUTOPILOT_VERSION(ref body) => body.serialize(),
            &DkMessage::LANDING_TARGET(ref body) => body.serialize(),
            &DkMessage::VIBRATION(ref body) => body.serialize(),
            &DkMessage::HOME_POSITION(ref body) => body.serialize(),
            &DkMessage::SET_HOME_POSITION(ref body) => body.serialize(),
            &DkMessage::MESSAGE_INTERVAL(ref body) => body.serialize(),
            &DkMessage::EXTENDED_SYS_STATE(ref body) => body.serialize(),
            &DkMessage::ADSB_VEHICLE(ref body) => body.serialize(),
            &DkMessage::V2_EXTENSION(ref body) => body.serialize(),
            &DkMessage::MEMORY_VECT(ref body) => body.serialize(),
            &DkMessage::DEBUG_VECT(ref body) => body.serialize(),
            &DkMessage::NAMED_VALUE_FLOAT(ref body) => body.serialize(),
            &DkMessage::NAMED_VALUE_INT(ref body) => body.serialize(),
            &DkMessage::STATUSTEXT(ref body) => body.serialize(),
            &DkMessage::DEBUG(ref body) => body.serialize(),
        }
    }
}

