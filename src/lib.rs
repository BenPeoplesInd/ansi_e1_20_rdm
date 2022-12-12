#![allow(dead_code)]
#![feature(int_abs_diff)]
#[macro_use]
extern crate log;
extern crate simplelog;

use std::fmt;
use core::cmp::Ordering;
use core::cmp::min;
use std::str;
use std::u32;
use std::u16;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut testpkt = Pkt::new();

        testpkt.message_length = 25;
        testpkt.destination = Uid::new(0x1234, 0x56789ABC);
        testpkt.source = Uid::new(0xCBA9, 0x87654321);
        testpkt.port_or_response_type = 1;
        testpkt.cc = GET_COMMAND;
        testpkt.pid = STATUS_MESSAGES;
        testpkt.pdl = 1;
        testpkt.pd = vec![0x55];
    
        testpkt.set_checksum();
    
        println!("{:?}",testpkt);
    
        assert_eq!(true,testpkt.test_checksum());
    
        let buf = testpkt.serialize();
    
        println!("{:X?}",buf);
    
        let testpkt2 = Pkt::deserialize(buf);
    
        println!("{:?}",testpkt2);
    
        let testpkt2 = testpkt2.unwrap();

        assert_eq!(true,testpkt2.test_checksum());

        assert_eq!(testpkt.checksum,testpkt2.checksum); // Verify that the two checksums actually match

    }

}

// A big ol table of pub constants
pub const SC_RDM : u8                       = 0xCC;
pub const SC_SUB_MESSAGE : u8               = 0x01;

pub const SUB_DEVICE_ALL_CALL : u16         = 0xFFFF;

// Table A-1 : Command Class Defines
pub const DISCOVERY_COMMAND : u8            = 0x10;
pub const DISCOVERY_COMMAND_RESPONSE : u8   = 0x11;
pub const GET_COMMAND : u8                  = 0x20;
pub const GET_COMMAND_RESPONSE : u8         = 0x21;
pub const SET_COMMAND : u8                  = 0x30;
pub const SET_COMMAND_RESPONSE : u8         = 0x31;

// Table A-2: Response Type Defines 
pub const RESPONSE_TYPE_ACK : u8            = 0x00;
pub const RESPONSE_TYPE_ACK_TIMER : u8      = 0x01;
pub const RESPONSE_TYPE_NACK_REASON : u8    = 0x02;
pub const RESPONSE_TYPE_ACK_OVERFLOW : u8   = 0x03;

// Table A-3: Table A-3: RDM Categories/Parameter ID Defines
// These are almost exclusively lifted from here: https://github.com/ETCLabs/ETCDmxTool/blob/33f9aafcb7f0f78f59fc8ad3441878762202330a/src/rdm/estardm.h
// Copyright notice:
/*****************************************************************/
/* Entertainment Services Technology Association (ESTA)          */
/* ANSI E1.20 Remote Device Management (RDM) over DMX512 Networks*/
/*****************************************************************/
/*                                                               */
/*                          RDM.h                                */
/*                                                               */
/*****************************************************************/
/* Appendix A Defines for the RDM Protocol.                      */
/* Publish date: 3/31/2006                                       */
/*****************************************************************/
/* Compiled by: Scott M. Blair   8/18/2006                       */
/* Updated 10/11/2011: Adding E1.20-2010 and E1.37-1 defines.    */
/*****************************************************************/
/* For updates see: http://www.rdmprotocol.org                   */
/*****************************************************************/
/* Copyright 2006,2011 Litespeed Design                          */
/*****************************************************************/
/* Permission to use, copy, modify, and distribute this software */
/* is freely granted, provided that this notice is preserved.    */
/*****************************************************************/
// TODO: Fix formatting here so it looks more rusty.
/********************************************************/
/* Table A-3: RDM Parameter ID's (Slots 21-22)          */
/********************************************************/
// Category - Network Management 
pub const DISC_UNIQUE_BRANCH                           : u16 = 0x0001;
pub const DISC_MUTE                                    : u16 = 0x0002;
pub const DISC_UN_MUTE                                 : u16 = 0x0003;
pub const PROXIED_DEVICES                              : u16 = 0x0010;
pub const PROXIED_DEVICE_COUNT                         : u16 = 0x0011;
pub const COMMS_STATUS                                 : u16 = 0x0015;

// Category - Status Collection    
pub const QUEUED_MESSAGE                               : u16 = 0x0020; /* See Table A-4                                              */
pub const STATUS_MESSAGES                              : u16 = 0x0030; /* See Table A-4                                              */
pub const STATUS_ID_DESCRIPTION                        : u16 = 0x0031;
pub const CLEAR_STATUS_ID                              : u16 = 0x0032;
pub const SUB_DEVICE_STATUS_REPORT_THRESHOLD           : u16 = 0x0033; /* See Table A-4                                              */

// Category - RDM Information     
pub const SUPPORTED_PARAMETERS                         : u16 = 0x0050; /* Support required only if supporting Parameters beyond the minimum required set.*/
pub const PARAMETER_DESCRIPTION                        : u16 = 0x0051; /* Support required for Manufacturer-Specific PIDs exposed in SUPPORTED_PARAMETERS message */

// Category - Product Information  
pub const DEVICE_INFO                                  : u16 = 0x0060;
pub const PRODUCT_DETAIL_ID_LIST                       : u16 = 0x0070;
pub const DEVICE_MODEL_DESCRIPTION                     : u16 = 0x0080;
pub const MANUFACTURER_LABEL                           : u16 = 0x0081;
pub const DEVICE_LABEL                                 : u16 = 0x0082;
pub const FACTORY_DEFAULTS                             : u16 = 0x0090;
pub const LANGUAGE_CAPABILITIES                        : u16 = 0x00A0;
pub const LANGUAGE                                     : u16 = 0x00B0;
pub const SOFTWARE_VERSION_LABEL                       : u16 = 0x00C0;
pub const BOOT_SOFTWARE_VERSION_ID                     : u16 = 0x00C1;
pub const BOOT_SOFTWARE_VERSION_LABEL                  : u16 = 0x00C2;

// Category - DMX512 Setup         
pub const DMX_PERSONALITY                              : u16 = 0x00E0;
pub const DMX_PERSONALITY_DESCRIPTION                  : u16 = 0x00E1;
pub const DMX_START_ADDRESS                            : u16 = 0x00F0; /* Support required if device uses a DMX512 Slot.             */
pub const SLOT_INFO                                    : u16 = 0x0120;
pub const SLOT_DESCRIPTION                             : u16 = 0x0121;
pub const DEFAULT_SLOT_VALUE                           : u16 = 0x0122;
pub const DMX_BLOCK_ADDRESS                          : u16 = 0x0140; /* Defined in ANSI E1.37-1 document                           */
pub const DMX_FAIL_MODE                              : u16 = 0x0141; /* Defined in ANSI E1.37-1 document                           */
pub const DMX_STARTUP_MODE                           : u16 = 0x0142; /* Defined in ANSI E1.37-1 document                           */


// Category - Sensors              
pub const SENSOR_DEFINITION                            : u16 = 0x0200;
pub const SENSOR_VALUE                                 : u16 = 0x0201;
pub const RECORD_SENSORS                               : u16 = 0x0202;

// Category - Dimmer Settings      
pub const DIMMER_INFO                                : u16 = 0x0340;
pub const MINIMUM_LEVEL                              : u16 = 0x0341;
pub const MAXIMUM_LEVEL                              : u16 = 0x0342;
pub const CURVE                                      : u16 = 0x0343;
pub const CURVE_DESCRIPTION                          : u16 = 0x0344; /* Support required if CURVE is supported                     */
pub const OUTPUT_RESPONSE_TIME                       : u16 = 0x0345;
pub const OUTPUT_RESPONSE_TIME_DESCRIPTION           : u16 = 0x0346; /* Support required if OUTPUT_RESPONSE_TIME is supported      */
pub const MODULATION_FREQUENCY                       : u16 = 0x0347;
pub const MODULATION_FREQUENCY_DESCRIPTION           : u16 = 0x0348; /* Support required if MODULATION_FREQUENCY is supported      */

// Category - Power/Lamp Settings  
pub const DEVICE_HOURS                                 : u16 = 0x0400;
pub const LAMP_HOURS                                   : u16 = 0x0401;
pub const LAMP_STRIKES                                 : u16 = 0x0402;
pub const LAMP_STATE                                   : u16 = 0x0403; /* See Table A-8                                              */
pub const LAMP_ON_MODE                                 : u16 = 0x0404; /* See Table A-9                                              */
pub const DEVICE_POWER_CYCLES                          : u16 = 0x0405;
pub const BURN_IN									  : u16 = 0x0440; /* Defined in ANSI E1.37-1                                    */

// Category - Display Settings     
pub const DISPLAY_INVERT                               : u16 = 0x0500;
pub const DISPLAY_LEVEL                                : u16 = 0x0501;

// Category - Configuration        
pub const PAN_INVERT                                   : u16 = 0x0600;
pub const TILT_INVERT                                  : u16 = 0x0601;
pub const PAN_TILT_SWAP                                : u16 = 0x0602;
pub const REAL_TIME_CLOCK                              : u16 = 0x0603;
pub const LOCK_PIN                                   : u16 = 0x0640; /* Defined in ANSI E1.37-1                                    */
pub const LOCK_STATE                                 : u16 = 0x0641; /* Defined in ANSI E1.37-1                                    */
pub const LOCK_STATE_DESCRIPTION                     : u16 = 0x0642; /* Support required if MODULATION_FREQUENCY is supported      */

// Category - Control              
pub const IDENTIFY_DEVICE                              : u16 = 0x1000;
pub const RESET_DEVICE                                 : u16 = 0x1001;
pub const POWER_STATE                                  : u16 = 0x1010; /* See Table A-11                                              */
pub const PERFORM_SELFTEST                             : u16 = 0x1020; /* See Table A-10                                              */
pub const SELF_TEST_DESCRIPTION                        : u16 = 0x1021;
pub const CAPTURE_PRESET                               : u16 = 0x1030;
pub const PRESET_PLAYBACK                              : u16 = 0x1031; /* See Table A-7                                               */
pub const IDENTIFY_MODE                              : u16 = 0x1040; /* Defined in ANSI E1.37-1                                     */
pub const PRESET_INFO                                : u16 = 0x1041; /* Defined in ANSI E1.37-1                                     */
pub const PRESET_STATUS                              : u16 = 0x1042; /* Defined in ANSI E1.37-1                                     */
pub const PRESET_MERGEMODE                           : u16 = 0x1043; /* See E1.37-1 Table A-3                                       */
pub const POWER_ON_SELF_TEST                         : u16 = 0x1044; /* Defined in ANSI E1.37-1                                     */


/// UID Struct
/// This contains the manufacturer and device ids
/// Implementations will include the ability to format as MMMM:DDDDDDDD as well as strings of bytes
/// 
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Uid {
    pub mfg : u16,
    pub dev : u32
}

impl Uid {

    pub fn uid_serialize(self) -> [u8; 6] {
        let mfg : [u8; 2] = self.mfg.to_be_bytes();
        let dev : [u8; 4] = self.dev.to_be_bytes();

        let mut ret : [u8; 6] = [0; 6];

        ret[0..2].clone_from_slice(&mfg);
        ret[2..].clone_from_slice(&dev);

        return ret;
    }

    pub fn from_bytes(data: &[u8]) -> Uid {

        let mut mfg_array = [0u8; 2];
        let mut dev_array = [0u8; 4];

        mfg_array.clone_from_slice(&data[0..2]);
        dev_array.clone_from_slice(&data[2..6]);


        Uid { mfg: u16::from_be_bytes(mfg_array),
            dev: u32::from_be_bytes(dev_array)}
    }

    /// Given a UID in the standard format like 044E:123456 it'll generate a UID
    pub fn from_string(data: String) -> Uid {
        Uid { 
            mfg: u16::from_str_radix(&data[0..4],16).unwrap_or(0x00),  
            dev: u32::from_str_radix(&data[5..], 16).unwrap_or(0x00)            
        }
    }

    pub fn new(mfg : u16, dev : u32) -> Uid {
        Uid { mfg , dev }
    }

    pub fn set_mfg(mut self, mfg_id: u16) -> () {
        self.mfg = mfg_id;
    }

    pub fn set_dev(mut self, dev_id: u32) -> () {
        self.dev = dev_id;
    }

    pub fn get_mfg(self) -> u16 {
        self.mfg
    }

    pub fn get_dev(self) -> u32 {
        self.dev
    }

    pub fn get_as_64(self) -> u64 {
        ((self.mfg as u64) << 32) + (self.dev as u64)
    }

    pub fn set_from_64(mut self, val: u64) -> () {
        println!("Val: {} mfg: {} dev: {}",val,(val >> 32) as u16,val as u32);
        self.mfg = (val >> 32) as u16;
        self.dev = val as u32;
        println!("mfg: {} dev: {}",self.mfg,self.dev);
    }

    pub fn new_from_64(val: u64) -> Uid {
        Uid {
            mfg : (val >> 32) as u16,
            dev : val as u32
        }
    }

    pub fn get_midpoint(self, top: &Uid) -> Uid {
        if self == *top {
            return self;
        }
        // if self.mfg == top.mfg {
        //     return Uid::new(self.mfg,self.dev.abs_diff(top.dev)/2 + min(self.dev,top.dev));
        // } 
        // else {
        //     return Uid::new(self.mfg.abs_diff(top.mfg)/2 + min(self.mfg,top.mfg),self.dev);
        // }

        let bot_64 : u64 = self.get_as_64();
        let top_64 : u64 = top.get_as_64();

        // println!("top_64: {} bot_64: {}",top_64,bot_64);

        let mid_64 : u64 = (top_64.abs_diff(bot_64) / 2 ) + min(top_64,bot_64);

        // println!("Midpoint: {}",mid_64);




        // let bot_64 : u64 = self.get_as_64();
        // let top_64 : u64 = top.get_as_64();

        // // let retval = Uid::new(0,0);

        // println!("Halfway between {} and {}",top_64,bot_64);

        // if top_64 > bot_64 {
        //     // println!("Result is {}", ((top_64 - bot_64) / 2 + bot_64));
        //     return Uid::new_from_64((top_64 - bot_64) / 2 + bot_64);
        // } 
        // if bot_64 > top_64 {
        //     return Uid::new_from_64((bot_64 - top_64) / 2 + top_64);
        // }
        
        Uid::new_from_64(mid_64)

    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04X}:{:08X}", self.mfg, self.dev)
    }
}

impl fmt::Debug for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}:{:08X}", self.mfg, self.dev)
    }
}

/// Packet Structure
/// This includes all fields (including ones can calculate)
/// This allows us to use it as a container format for incoming packets to validate them
#[derive(Clone)]
pub struct Pkt {
    pub start : u8,
    pub substart : u8,
    pub message_length : u8,
    pub destination : Uid,
    pub source : Uid,
    pub tn : u8,
    pub port_or_response_type : u8,
    pub message_count : u8,
    pub subdevice : u16,
    pub cc : u8, // Command Class
    pub pid : u16, // PID
    pub pdl : u8, // PDL
    pub pd : Vec<u8>,
    pub checksum : u16
}

impl fmt::Debug for Pkt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X} {:02X}\nLength: {}\nDest: {}\nSource: {}\nTN: {:02X} RT: {:02X} MC: {:02X}\nSD: {}\nCC: {:02X} PID: {:04X} PDL: {}\nPD: {:?}\nChecksum: {:04X}", 
        self.start, self.substart, 
        self.message_length,
        self.destination,
        self.source,
        self.tn, self.port_or_response_type, self.message_count,
        self.subdevice,
        self.cc, self.pid, self.pdl,
        self.pd,
        self.checksum
    )
    }
}

impl Pkt {
    pub fn new() -> Pkt {
        Pkt { start: SC_RDM, substart: SC_SUB_MESSAGE, message_length: 0, destination: Uid::new(0,0), source: Uid::new(0,0), tn: 0, 
            port_or_response_type: 0, message_count: 0, subdevice: 0, cc: 0, pid: 0, pdl: 0, pd: Vec::new(), checksum: 0 }
    }

    pub fn set_checksum(&mut self) -> u16 {
        let mut checksum : u16 = 0;

        // let mut temp_pkt = self.clone();

        self.checksum = 0;

        let data = self.serialize();

        for byte in data {
            checksum = checksum.overflowing_add(byte as u16).0;
        }

        self.checksum = checksum;

        return checksum;

    }

    pub fn test_checksum(&self) -> bool {
        let mut checksum : u16 = 0;
        let mut test_packet = self.clone();
        test_packet.checksum = 0;

        let data = test_packet.serialize();

        for byte in data {
            checksum = checksum.overflowing_add(byte as u16).0;
        }

        self.checksum == checksum

    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.push(self.start);
        data.push(self.substart);
        data.push(self.message_length);
        data.extend(self.destination.uid_serialize());
        data.extend(self.source.uid_serialize());
        data.push(self.tn);
        data.push(self.port_or_response_type);
        data.push(self.message_count);
        data.extend(self.subdevice.to_be_bytes());
        data.push(self.cc);
        data.extend(self.pid.to_be_bytes());
        data.push(self.pdl);
        data.extend(self.pd.as_slice());
        data.extend(self.checksum.to_be_bytes());

        return data;
    }

    pub fn deserialize(data: Vec<u8>) -> Option<Pkt> {
        
        // debug!("deserialize({:?})",data);

        let mut ret = Pkt::new();

        if data.len() < 14 {
            error!("Not enough data, need at least 14 bytes, have {}",data.len());
            return None;
            
        }

        ret.start = data[0];
        ret.substart = data[1];
        ret.message_length = data[2];

        if data.len() < (ret.message_length as usize) + 2 {
            error!("data.len was incorrect got {} wanted {}",data.len(),ret.message_length+2);
            return None; // May want some more useful errors later
        }

        ret.destination = Uid::from_bytes(&data[3..9]);
        ret.source = Uid::from_bytes(&data[9..15]);

        ret.tn = data[15];
        ret.port_or_response_type = data[16];
        ret.message_count = data[17];
        ret.subdevice = u16::from_be_bytes(data[18..20].try_into().unwrap());
        ret.cc = data[20];
        ret.pid = u16::from_be_bytes(data[21..23].try_into().unwrap());
        ret.pdl = data[23];
        if ret.pdl > 0 {
            ret.pd.extend(&data[24..((ret.pdl+24) as usize)]);
        }
        ret.checksum = u16::from_be_bytes(data[(ret.message_length as usize)..((ret.message_length+2) as usize)].try_into().unwrap());

        return Some(ret);
    }

    pub fn set_message_length(&mut self) -> u8 {
        self.message_length = self.pdl + 24;
        self.message_length
    }

}

/// DiscoveryResponse packet data
/// None == no response received
/// One == a single UID was received with a valid checksum
/// Some == Data was received, but it was invalid 
pub enum DiscoveryResponse {
    None,
    One(Uid),
    Some
}

impl PartialEq for Uid {
    fn eq(&self, other: &Self) -> bool {
        self.dev == other.dev && self.mfg == other.mfg
    }
}

#[derive(Debug)]
pub struct DeviceLabelPD {
    pub device_label : String
}

impl DeviceLabelPD {
    pub fn serialize(&self) -> Vec<u8> {
        let mut rv = Vec::new();

        rv.extend(self.device_label.as_bytes());

        return rv;

    }

    pub fn deserialize(data: Vec<u8>) -> Option<DeviceLabelPD> {

        let mut pd = DeviceLabelPD::new();

        if data.len() == 0 {
            return Some(pd);
        }

        let mut last_index = data.len();

        for i in 0..data.len() {
            if data[i] == 0x00 { // if it's a null terminated string, truncate here.
                last_index = i;
                break;
            }
        }
        
        pd.device_label = str::from_utf8(&data[0..last_index]).unwrap_or("").to_string();
        
        return Some(pd);
    }

    pub fn new() -> DeviceLabelPD {
        DeviceLabelPD { device_label: "".to_string() }
    }
}

#[derive(Debug)]
pub struct DeviceInfoPD {
    pub rdm_protocol_version : u16,
    pub device_model_id : u16,
    pub product_category : u16,
    pub software_version_id : u32,
    pub dmx_footprint: u16,
    pub dmx_personality : u16,
    pub dmx_start_address : u16,
    pub sub_device_count : u16,
    pub sensor_count : u8
}

impl DeviceInfoPD {
    pub fn serialize(&self) -> Vec<u8> {
        let mut rv = Vec::new();

        rv.extend(self.rdm_protocol_version.to_be_bytes());
        rv.extend(self.device_model_id.to_be_bytes());
        rv.extend(self.product_category.to_be_bytes());
        rv.extend(self.software_version_id.to_be_bytes());
        rv.extend(self.dmx_footprint.to_be_bytes());
        rv.extend(self.dmx_personality.to_be_bytes());
        rv.extend(self.dmx_start_address.to_be_bytes());
        rv.extend(self.sub_device_count.to_be_bytes());

        rv.push(self.sensor_count);
        
        return rv;
    }

    pub fn deserialize(data : Vec<u8>) -> Option<DeviceInfoPD> {
        if data.len() < 0x13 {
            return None;
        }

        let mut pd = DeviceInfoPD::new();

        pd.rdm_protocol_version = u16::from_be_bytes(data[0..2].try_into().unwrap());
        pd.device_model_id = u16::from_be_bytes(data[2..4].try_into().unwrap());
        pd.product_category = u16::from_be_bytes(data[4..6].try_into().unwrap());
        pd.software_version_id = u32::from_be_bytes(data[6..10].try_into().unwrap());
        pd.dmx_footprint = u16::from_be_bytes(data[10..12].try_into().unwrap());
        pd.dmx_personality = u16::from_be_bytes(data[12..14].try_into().unwrap());
        pd.dmx_start_address = u16::from_be_bytes(data[14..16].try_into().unwrap());
        pd.sub_device_count = u16::from_be_bytes(data[16..18].try_into().unwrap());

        pd.sensor_count = data[18];

        return Some(pd);

    }

    pub fn new() -> DeviceInfoPD {
        DeviceInfoPD { 
            rdm_protocol_version: 0, 
            device_model_id: 0, 
            product_category: 0, 
            software_version_id: 0, 
            dmx_footprint: 0, 
            dmx_personality: 0, 
            dmx_start_address: 0, 
            sub_device_count: 0, 
            sensor_count: 0 
        }   

    }

}

#[derive(Debug)]
pub struct SensorDefinitionPD {
    pub id : u8,
    pub sensor_type : u8,
    pub unit : u8,
    pub prefix : u8,
    pub range_min : i16,
    pub range_max : i16,
    pub normal_min : i16,
    pub normal_max : i16,
    pub recorded_value : u8,
    pub description : String
}

impl SensorDefinitionPD {
    pub fn serialize(&self) -> Vec<u8> {
        let mut rv = Vec::new();

        rv.push(self.id);
        rv.push(self.sensor_type);
        rv.push(self.unit);
        rv.push(self.prefix);

        rv.extend(self.range_min.to_be_bytes());
        rv.extend(self.range_max.to_be_bytes());
        rv.extend(self.normal_min.to_be_bytes());
        rv.extend(self.normal_max.to_be_bytes());

        rv.push(self.recorded_value);
        
        rv.extend(self.description.as_bytes());

        return rv;
    }

    pub fn deserialize(data : Vec<u8>) -> Option<SensorDefinitionPD> {
        if data.len() < 0x0D {
            return None;
        }

        let mut pd = SensorDefinitionPD::new();

        pd.id = data[0];
        pd.sensor_type = data[1];
        pd.unit = data[2];
        pd.prefix = data[3];

        pd.range_min = i16::from_be_bytes(data[4..6].try_into().unwrap());
        pd.range_max = i16::from_be_bytes(data[6..8].try_into().unwrap());
        pd.normal_min = i16::from_be_bytes(data[8..10].try_into().unwrap());
        pd.normal_max = i16::from_be_bytes(data[10..12].try_into().unwrap());
        
        pd.recorded_value = data[12];

        if data.len() > 0x0D {

            let mut last_index = data.len();

            for i in 13..data.len() {
                if data[i] == 0x00 { // if it's a null terminated string, truncate here.
                    last_index = i;
                    break;
                }
            }

            if last_index > 13 {
                pd.description = str::from_utf8(&data[13..last_index]).unwrap_or("").to_string();
            }
        }

        return Some(pd);

    }

    pub fn new() -> SensorDefinitionPD {
        SensorDefinitionPD { 
            id: 0, 
            sensor_type: 0, 
            unit: 0, 
            prefix: 0, 
            range_min: 0, 
            range_max: 0, 
            normal_min: 0, 
            normal_max: 0, 
            recorded_value: 0, 
            description: "".to_string() }

    }


}

#[derive(Debug)]
pub struct SensorValuePD {
    pub id : u8,
    pub present : i16,
    pub lowest : i16,
    pub highest : i16,
    pub recorded: i16
}

impl SensorValuePD {
    pub fn new() -> SensorValuePD {
        SensorValuePD { id: 0, present: 0, lowest: 0, highest: 0, recorded: 0 }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut rv = Vec::new();

        rv.push(self.id);
        rv.extend(self.present.to_be_bytes());
        rv.extend(self.lowest.to_be_bytes());
        rv.extend(self.highest.to_be_bytes());
        rv.extend(self.recorded.to_be_bytes());

        return rv;
    }

    pub fn deserialize(data : Vec<u8>) -> Option<SensorValuePD> {
        if data.len() < 9 {
            return None;
        }

        let mut pd = SensorValuePD::new();

        pd.id = data[0];

        pd.present = i16::from_be_bytes(data[1..3].try_into().unwrap());
        pd.lowest = i16::from_be_bytes(data[3..5].try_into().unwrap());
        pd.highest = i16::from_be_bytes(data[5..7].try_into().unwrap());
        pd.recorded = i16::from_be_bytes(data[7..9].try_into().unwrap());

        return Some(pd);

    }


}

/********************************************************/
/* Table A-12: Sensor Type Defines                      */
/********************************************************/

pub const SENS_TEMPERATURE                            : u8  = 0x00;
pub const SENS_VOLTAGE                                : u8  = 0x01;
pub const SENS_CURRENT                                : u8  = 0x02;
pub const SENS_FREQUENCY                              : u8  = 0x03;
pub const SENS_RESISTANCE                             : u8  = 0x04;  /* Eg: Cable resistance                                         */
pub const SENS_POWER                                  : u8  = 0x05;
pub const SENS_MASS                                   : u8  = 0x06;   /* Eg: Truss load Cell                                          */
pub const SENS_LENGTH                                 : u8  = 0x07;
pub const SENS_AREA                                   : u8  = 0x08;
pub const SENS_VOLUME                                 : u8  = 0x09 ;  /* Eg: Smoke Fluid                                              */
pub const SENS_DENSITY                                : u8  = 0x0A;
pub const SENS_VELOCITY                               : u8  = 0x0B;
pub const SENS_ACCELERATION                           : u8  = 0x0C;
pub const SENS_FORCE                                  : u8  = 0x0D;
pub const SENS_ENERGY                                 : u8  = 0x0E;
pub const SENS_PRESSURE                               : u8  = 0x0F;
pub const SENS_TIME                                   : u8  = 0x10;
pub const SENS_ANGLE                                  : u8  = 0x11;
pub const SENS_POSITION_X                             : u8  = 0x12 ;  /* E.g.: Lamp position on Truss                                 */
pub const SENS_POSITION_Y                             : u8  = 0x13;
pub const SENS_POSITION_Z                             : u8  = 0x14;
pub const SENS_ANGULAR_VELOCITY                       : u8  = 0x15 ;  /* E.g.: Wind speed                                             */
pub const SENS_LUMINOUS_INTENSITY                     : u8  = 0x16;
pub const SENS_LUMINOUS_FLUX                          : u8  = 0x17;
pub const SENS_ILLUMINANCE                            : u8  = 0x18;
pub const SENS_CHROMINANCE_RED                        : u8  = 0x19;
pub const SENS_CHROMINANCE_GREEN                      : u8  = 0x1A;
pub const SENS_CHROMINANCE_BLUE                       : u8  = 0x1B;
pub const SENS_CONTACTS                               : u8  = 0x1C ;  /* E.g.: Switch inputs.                                         */
pub const SENS_MEMORY                                 : u8  = 0x1D  ; /* E.g.: ROM Size                                               */
pub const SENS_ITEMS                                  : u8  = 0x1E   ;/* E.g.: Scroller gel frames.                                   */
pub const SENS_HUMIDITY                               : u8  = 0x1F;
pub const SENS_COUNTER_16BIT                          : u8  = 0x20;
pub const SENS_OTHER                                  : u8  = 0x7F;
/* Manufacturer-Specific Sensors                         : u8  = 0x;80-
                                                         : u8  = 0xF;F                                                                   */

/********************************************************/
/* Table A-13: Sensor Unit Defines                      */
/********************************************************/

pub const UNITS_NONE                                  : u8  = 0x00;   /* CONTACTS                                                     */
pub const UNITS_CENTIGRADE                            : u8  = 0x01;   /* TEMPERATURE	                                                */
pub const UNITS_VOLTS_DC                              : u8  = 0x02;   /* VOLTAGE		                                                */
pub const UNITS_VOLTS_AC_PEAK                         : u8  = 0x03;   /* VOLTAGE                                                      */
pub const UNITS_VOLTS_AC_RMS                          : u8  = 0x04;   /* VOLTAGE                                                      */
pub const UNITS_AMPERE_DC                             : u8  = 0x05;   /* CURRENT	                                                    */
pub const UNITS_AMPERE_AC_PEAK                        : u8  = 0x06;   /* CURRENT	                                                    */
pub const UNITS_AMPERE_AC_RMS                         : u8  = 0x07;   /* CURRENT                                                      */
pub const UNITS_HERTZ                                 : u8  = 0x08;   /* FREQUENCY / ANG_VEL                                          */
pub const UNITS_OHM                                   : u8  = 0x09;   /* RESISTANCE			                                        */
pub const UNITS_WATT                                  : u8  = 0x0A;   /* POWER					                                    */
pub const UNITS_KILOGRAM                              : u8  = 0x0B;   /* MASS                                                         */
pub const UNITS_METERS                                : u8  = 0x0C;   /* LENGTH / POSITION		                                    */
pub const UNITS_METERS_SQUARED                        : u8  = 0x0D;   /* AREA					                                        */
pub const UNITS_METERS_CUBED                          : u8  = 0x0E;   /* VOLUME                                                       */
pub const UNITS_KILOGRAMMES_PER_METER_CUBED           : u8  = 0x0F;   /* DENSITY                                                      */
pub const UNITS_METERS_PER_SECOND                     : u8  = 0x10;   /* VELOCITY		                                                */
pub const UNITS_METERS_PER_SECOND_SQUARED             : u8  = 0x11;   /* ACCELERATION	                                                */
pub const UNITS_NEWTON                                : u8  = 0x12;   /* FORCE                                                        */
pub const UNITS_JOULE                                 : u8  = 0x13;   /* ENERGY		                                                */
pub const UNITS_PASCAL                                : u8  = 0x14;   /* PRESSURE		                                                */
pub const UNITS_SECOND                                : u8  = 0x15;   /* TIME                                                         */
pub const UNITS_DEGREE                                : u8  = 0x16;   /* ANGLE			                                            */
pub const UNITS_STERADIAN                             : u8  = 0x17;   /* ANGLE			                                            */
pub const UNITS_CANDELA                               : u8  = 0x18;   /* LUMINOUS_INTENSITY                                           */
pub const UNITS_LUMEN                                 : u8  = 0x19;   /* LUMINOUS_FLUX		                                        */
pub const UNITS_LUX                                   : u8  = 0x1A;   /* ILLUMINANCE		                                            */
pub const UNITS_IRE                                   : u8  = 0x1B;   /* CHROMINANCE                                                  */
pub const UNITS_BYTE                                  : u8  = 0x1C;   /* MEMORY	                                                    */
/* Manufacturer-Specific Units                           : u8  = 0x80-
 				                                         : u8  = 0xFF				                                                    */


/********************************************************/
/* Table A-14: Sensor Unit Prefix Defines               */
/********************************************************/

pub const PREFIX_NONE                                 : u8  = 0x00;   /* Multiply by 1                                                */
pub const PREFIX_DECI                                 : u8  = 0x01;   /* Multiply by 10-1	                                            */
pub const PREFIX_CENTI                                : u8  = 0x02;   /* Multiply by 10-2	                                            */
pub const PREFIX_MILLI                                : u8  = 0x03;   /* Multiply by 10-3	                                            */
pub const PREFIX_MICRO                                : u8  = 0x04;   /* Multiply by 10-6	                                            */
pub const PREFIX_NANO                                 : u8  = 0x05;   /* Multiply by 10-9	                                            */
pub const PREFIX_PICO                                 : u8  = 0x06;   /* Multiply by 10-12	                                        */
pub const PREFIX_FEMPTO                               : u8  = 0x07;   /* Multiply by 10-15	                                        */
pub const PREFIX_ATTO                                 : u8  = 0x08;   /* Multiply by 10-18	                                        */
pub const PREFIX_ZEPTO                                : u8  = 0x09;   /* Multiply by 10-21	                                        */
pub const PREFIX_YOCTO                                : u8  = 0x0A;   /* Multiply by 10-24	                                        */
pub const PREFIX_DECA                                 : u8  = 0x11;   /* Multiply by 10+1	                                            */
pub const PREFIX_HECTO                                : u8  = 0x12;   /* Multiply by 10+2	                                            */
pub const PREFIX_KILO                                 : u8  = 0x13;   /* Multiply by 10+3	                                            */
pub const PREFIX_MEGA                                 : u8  = 0x14;   /* Multiply by 10+6	                                            */
pub const PREFIX_GIGA                                 : u8  = 0x15;   /* Multiply by 10+9	                                            */
pub const PREFIX_TERRA                                : u8  = 0x16;   /* Multiply by 10+12	                                        */
pub const PREFIX_PETA                                 : u8  = 0x17;   /* Multiply by 10+15	                                        */
pub const PREFIX_EXA                                  : u8  = 0x18;   /* Multiply by 10+18	                                        */
pub const PREFIX_ZETTA                                : u8  = 0x19;   /* Multiply by 10+21	                                        */
pub const PREFIX_YOTTA                                : u8  = 0x1A;   /* Multiply by 10+24	                                        */



// Just compare them as 64s
impl PartialOrd for Uid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.mfg == other.mfg { // if the mfg ids match, return the comparison between devices
            return self.dev.partial_cmp(&other.dev);
        } else {
            return self.mfg.partial_cmp(&other.mfg); // otherwise just compare devices
        }
    }
}

fn do_discovery_node(f: fn(&[u8]) -> Option<Vec<u8>>,my_uid: &Uid, min: &Uid, max: &Uid, tn: &mut u8) -> DiscoveryResponse {

    let mut output_pkt = Pkt::new();

    output_pkt.destination = Uid::new(0xFFFF,0xFFFF_FFFF);
    output_pkt.source = my_uid.clone();
    
    *tn = tn.overflowing_add(1).0;

    output_pkt.tn = *tn;

    output_pkt.port_or_response_type = 0x01;

    output_pkt.cc = DISCOVERY_COMMAND;

    output_pkt.pid = DISC_UNIQUE_BRANCH;

    output_pkt.pdl = 0x0C;

    output_pkt.pd.extend(min.uid_serialize());
    output_pkt.pd.extend(max.uid_serialize());

    output_pkt.set_message_length(); // sets message length from PDL
    output_pkt.set_checksum(); // sets checksum from the whole packet.

    match f(output_pkt.serialize().as_slice()) {
        None => {
            return DiscoveryResponse::None; // No response means no response
        }
        Some(data) => {
            let mut device_uid = Uid::new(0,0);
            let mut checksum : u16 = 0;
            let mut checksum_calculated : u16 = 0;

            if data.len() == 0 {
                return DiscoveryResponse::None;
            }

            if data.len() >= 16 {

                let mut preamble_ptr : usize = 0;

                for byte in &data {
                    preamble_ptr += 1;
                    if *byte == 0xFE {
                        continue;
                    } 
                    if *byte == 0xAA {
                        break;
                    }
                }

                // Make sure we have enough bytes after the preamble, if not, we have a Some.
                if data.len() < 16+preamble_ptr {
                    return DiscoveryResponse::Some;
                }

                device_uid.mfg = ((data[preamble_ptr] as u16 & data[preamble_ptr+1] as u16) << 8) + (data[preamble_ptr+2] as u16 & data[preamble_ptr+3] as u16);
                device_uid.dev =    ((data[preamble_ptr+4] as u32 & data[preamble_ptr+5] as u32) << 24) 
                                    + ((data[preamble_ptr+6] as u32 & data[preamble_ptr+7] as u32) << 16) 
                                    + ((data[preamble_ptr+8] as u32 & data[preamble_ptr+9] as u32) << 8) 
                                    + (data[preamble_ptr+10] as u32 & data[preamble_ptr+11] as u32);

                checksum = ((data[preamble_ptr+12] as u16 & data[preamble_ptr+13] as u16) << 8) + (data[preamble_ptr+14] as u16 & data[preamble_ptr+15] as u16);

                for i in preamble_ptr..(preamble_ptr+12) {
                    checksum_calculated = checksum_calculated.overflowing_add(data[i] as u16).0;
                }
                
                // If the checksum validates, we have a device, if not we return a Some.
                if checksum == checksum_calculated {
                    return DiscoveryResponse::One(device_uid);
                } else {
                    return DiscoveryResponse::Some;
                }
            } else {
                return DiscoveryResponse::Some; // We got something, but don't know what it is.
            }
        }

    }



}

/// Runs the discovery algorithm.
/// 0. Optionally: Unmute all (out of scope)
/// 1. Do allcall discovery.   If no response, then return empty Vec If response, go to 2
/// 2. Check left-hand side of tree and gather UIDs
/// 3. Check right-hand side of tree and gather UIDs
/// 4. Concatenate and return
pub fn do_discovery_algo(f: fn(&[u8]) -> Option<Vec<u8>>, my_uid: &Uid, do_mute: bool, repeat_disc: bool) -> Vec<Uid> {

    let mut tn : u8 = 0;

    if do_mute {
        let mut output_pkt = Pkt::new();

        output_pkt.destination = Uid::new(0xFFFF,0xFFFF_FFFF);
        output_pkt.source = my_uid.clone();
        
        tn = tn.overflowing_add(1).0;

        output_pkt.tn = tn;

        output_pkt.port_or_response_type = 0x01;

        output_pkt.cc = DISCOVERY_COMMAND;

        output_pkt.pid = DISC_UN_MUTE;

        output_pkt.pdl = 0x00;

        output_pkt.set_message_length(); // sets message length from PDL
        output_pkt.set_checksum(); // sets checksum from the whole packet.

        f(output_pkt.serialize().as_slice()); // send global unmute

    }

    let min : Uid = Uid::new(0,0); 
    let max : Uid = Uid::new(0x7FFF, 0xFFFF_FFFF);

    let mut tod = do_discovery_recursion(f, &my_uid, &min, &max, &mut tn,repeat_disc);

    if repeat_disc && tod.len() == 0 {
        tod = do_discovery_recursion(f, &my_uid, &min, &max, &mut tn,repeat_disc);
    }

    if repeat_disc && tod.len() == 0 {
        tod = do_discovery_recursion(f, &my_uid, &min, &max, &mut tn,repeat_disc);
    }

    return tod;
}

/// Sends a mute message and then returns true if it got an ACK or false if it didn't
fn send_mute_message(f: fn(&[u8]) -> Option<Vec<u8>>, my_uid: &Uid, uid: &Uid, tn : &mut u8) -> bool {
    let mut output_pkt = Pkt::new();

    output_pkt.destination = *uid;
    output_pkt.source = my_uid.clone();
    
    *tn = tn.overflowing_add(1).0;

    output_pkt.tn = *tn;

    output_pkt.port_or_response_type = 0x01;

    output_pkt.cc = DISCOVERY_COMMAND;

    output_pkt.pid = DISC_MUTE;

    output_pkt.pdl = 0x00;

    output_pkt.set_message_length(); // sets message length from PDL
    output_pkt.set_checksum(); // sets checksum from the whole packet.

    match f(output_pkt.serialize().as_slice()) {
        None => return false,
        Some(data) => {
            let response_pkt = Pkt::deserialize(data);

            match response_pkt {
                None => return false,
                Some(data_pkt) => {
                    if data_pkt.test_checksum() {
                        if data_pkt.port_or_response_type == RESPONSE_TYPE_ACK 
                        && data_pkt.source == *uid 
                        && data_pkt.cc == DISCOVERY_COMMAND_RESPONSE 
                        && data_pkt.pid == DISC_MUTE {
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }


        }
    }

}

fn do_discovery_recursion(f: fn(&[u8]) -> Option<Vec<u8>>, my_uid: &Uid, min: &Uid, max: &Uid, tn : &mut u8, repeat_disc: bool) -> Vec<Uid> {
    let mut tod : Vec<Uid> = Vec::new();

    debug!("do_discovery_recursion({},{})",min,max);

    match do_discovery_node(f,&my_uid,min, max, tn) {
        DiscoveryResponse::None => { 
            return tod; // nothing in this branch, go back up.
        },
        DiscoveryResponse::One(found_uid) => {
            debug!("do_discovery_recursion: Found {}, muting it.",found_uid);
            if send_mute_message(f,&my_uid,&found_uid,tn) {
                tod.push(found_uid);
                return tod; // only one thing here, return it.
            }
         },
        DiscoveryResponse::Some => { 
            // println!("Found some responders, digging deeper");
            // need to dig deeper, so don't return.
        }
    }

    let mid = min.get_midpoint(max);

    // println!("Midpoint is {}", mid);

    // Do the left branch.
    let left_branch = do_discovery_recursion(f,&my_uid, min,&mid, tn, repeat_disc);

    if repeat_disc && left_branch.len() == 0 {
        let left_branch = do_discovery_recursion(f,&my_uid, min,&mid, tn, repeat_disc);
        if left_branch.len() == 0 {
            let left_branch = do_discovery_recursion(f,&my_uid, min,&mid, tn, repeat_disc);
            tod.extend(left_branch);
        } else {
            tod.extend(left_branch);
        }
    } else {
        tod.extend(left_branch); // extend tod with the contents of left branch here even if we don't repeat discovery
    }

    // now do the right branch
    let right_branch = do_discovery_recursion(f,&my_uid, &mid,max, tn, repeat_disc);
    if repeat_disc && right_branch.len() == 0 {
        let right_branch = do_discovery_recursion(f,&my_uid, &mid,max, tn, repeat_disc);
        if right_branch.len() == 0 {
            let right_branch = do_discovery_recursion(f,&my_uid, &mid,max, tn, repeat_disc);
            tod.extend(right_branch);
        } else {
            tod.extend(right_branch);
        }
    } else {
        tod.extend(right_branch);
    }

   
    
    return tod;

}

