#![allow(dead_code)]
#![feature(int_abs_diff)]

use std::fmt;
use core::cmp::Ordering;
use core::cmp::min;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// A big ol table of constants
const SC_RDM : u8                       = 0xCC;
const SC_SUB_MESSAGE : u8               = 0x01;

const SUB_DEVICE_ALL_CALL : u16         = 0xFFFF;

// Table A-1 : Command Class Defines
const DISCOVERY_COMMAND : u8            = 0x10;
const DISCOVERY_COMMAND_RESPONSE : u8   = 0x11;
const GET_COMMAND : u8                  = 0x20;
const GET_COMMAND_RESPONSE : u8         = 0x21;
const SET_COMMAND : u8                  = 0x30;
const SET_COMMAND_RESPONSE : u8         = 0x31;

// Table A-2: Response Type Defines 
const RESPONSE_TYPE_ACK : u8            = 0x00;
const RESPONSE_TYPE_ACK_TIMER : u8      = 0x01;
const RESPONSE_TYPE_NACK_REASON : u8    = 0x02;
const RESPONSE_TYPE_ACK_OVERFLOW : u8   = 0x03;

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
const DISC_UNIQUE_BRANCH                           : u16 = 0x0001;
const DISC_MUTE                                    : u16 = 0x0002;
const DISC_UN_MUTE                                 : u16 = 0x0003;
const PROXIED_DEVICES                              : u16 = 0x0010;
const PROXIED_DEVICE_COUNT                         : u16 = 0x0011;
const COMMS_STATUS                                 : u16 = 0x0015;

// Category - Status Collection    
const QUEUED_MESSAGE                               : u16 = 0x0020; /* See Table A-4                                              */
const STATUS_MESSAGES                              : u16 = 0x0030; /* See Table A-4                                              */
const STATUS_ID_DESCRIPTION                        : u16 = 0x0031;
const CLEAR_STATUS_ID                              : u16 = 0x0032;
const SUB_DEVICE_STATUS_REPORT_THRESHOLD           : u16 = 0x0033; /* See Table A-4                                              */

// Category - RDM Information     
const SUPPORTED_PARAMETERS                         : u16 = 0x0050; /* Support required only if supporting Parameters beyond the minimum required set.*/
const PARAMETER_DESCRIPTION                        : u16 = 0x0051; /* Support required for Manufacturer-Specific PIDs exposed in SUPPORTED_PARAMETERS message */

// Category - Product Information  
const DEVICE_INFO                                  : u16 = 0x0060;
const PRODUCT_DETAIL_ID_LIST                       : u16 = 0x0070;
const DEVICE_MODEL_DESCRIPTION                     : u16 = 0x0080;
const MANUFACTURER_LABEL                           : u16 = 0x0081;
const DEVICE_LABEL                                 : u16 = 0x0082;
const FACTORY_DEFAULTS                             : u16 = 0x0090;
const LANGUAGE_CAPABILITIES                        : u16 = 0x00A0;
const LANGUAGE                                     : u16 = 0x00B0;
const SOFTWARE_VERSION_LABEL                       : u16 = 0x00C0;
const BOOT_SOFTWARE_VERSION_ID                     : u16 = 0x00C1;
const BOOT_SOFTWARE_VERSION_LABEL                  : u16 = 0x00C2;

// Category - DMX512 Setup         
const DMX_PERSONALITY                              : u16 = 0x00E0;
const DMX_PERSONALITY_DESCRIPTION                  : u16 = 0x00E1;
const DMX_START_ADDRESS                            : u16 = 0x00F0; /* Support required if device uses a DMX512 Slot.             */
const SLOT_INFO                                    : u16 = 0x0120;
const SLOT_DESCRIPTION                             : u16 = 0x0121;
const DEFAULT_SLOT_VALUE                           : u16 = 0x0122;
const DMX_BLOCK_ADDRESS                          : u16 = 0x0140; /* Defined in ANSI E1.37-1 document                           */
const DMX_FAIL_MODE                              : u16 = 0x0141; /* Defined in ANSI E1.37-1 document                           */
const DMX_STARTUP_MODE                           : u16 = 0x0142; /* Defined in ANSI E1.37-1 document                           */


// Category - Sensors              
const SENSOR_DEFINITION                            : u16 = 0x0200;
const SENSOR_VALUE                                 : u16 = 0x0201;
const RECORD_SENSORS                               : u16 = 0x0202;

// Category - Dimmer Settings      
const DIMMER_INFO                                : u16 = 0x0340;
const MINIMUM_LEVEL                              : u16 = 0x0341;
const MAXIMUM_LEVEL                              : u16 = 0x0342;
const CURVE                                      : u16 = 0x0343;
const CURVE_DESCRIPTION                          : u16 = 0x0344; /* Support required if CURVE is supported                     */
const OUTPUT_RESPONSE_TIME                       : u16 = 0x0345;
const OUTPUT_RESPONSE_TIME_DESCRIPTION           : u16 = 0x0346; /* Support required if OUTPUT_RESPONSE_TIME is supported      */
const MODULATION_FREQUENCY                       : u16 = 0x0347;
const MODULATION_FREQUENCY_DESCRIPTION           : u16 = 0x0348; /* Support required if MODULATION_FREQUENCY is supported      */

// Category - Power/Lamp Settings  
const DEVICE_HOURS                                 : u16 = 0x0400;
const LAMP_HOURS                                   : u16 = 0x0401;
const LAMP_STRIKES                                 : u16 = 0x0402;
const LAMP_STATE                                   : u16 = 0x0403; /* See Table A-8                                              */
const LAMP_ON_MODE                                 : u16 = 0x0404; /* See Table A-9                                              */
const DEVICE_POWER_CYCLES                          : u16 = 0x0405;
const BURN_IN									  : u16 = 0x0440; /* Defined in ANSI E1.37-1                                    */

// Category - Display Settings     
const DISPLAY_INVERT                               : u16 = 0x0500;
const DISPLAY_LEVEL                                : u16 = 0x0501;

// Category - Configuration        
const PAN_INVERT                                   : u16 = 0x0600;
const TILT_INVERT                                  : u16 = 0x0601;
const PAN_TILT_SWAP                                : u16 = 0x0602;
const REAL_TIME_CLOCK                              : u16 = 0x0603;
const LOCK_PIN                                   : u16 = 0x0640; /* Defined in ANSI E1.37-1                                    */
const LOCK_STATE                                 : u16 = 0x0641; /* Defined in ANSI E1.37-1                                    */
const LOCK_STATE_DESCRIPTION                     : u16 = 0x0642; /* Support required if MODULATION_FREQUENCY is supported      */

// Category - Control              
const IDENTIFY_DEVICE                              : u16 = 0x1000;
const RESET_DEVICE                                 : u16 = 0x1001;
const POWER_STATE                                  : u16 = 0x1010; /* See Table A-11                                              */
const PERFORM_SELFTEST                             : u16 = 0x1020; /* See Table A-10                                              */
const SELF_TEST_DESCRIPTION                        : u16 = 0x1021;
const CAPTURE_PRESET                               : u16 = 0x1030;
const PRESET_PLAYBACK                              : u16 = 0x1031; /* See Table A-7                                               */
const IDENTIFY_MODE                              : u16 = 0x1040; /* Defined in ANSI E1.37-1                                     */
const PRESET_INFO                                : u16 = 0x1041; /* Defined in ANSI E1.37-1                                     */
const PRESET_STATUS                              : u16 = 0x1042; /* Defined in ANSI E1.37-1                                     */
const PRESET_MERGEMODE                           : u16 = 0x1043; /* See E1.37-1 Table A-3                                       */
const POWER_ON_SELF_TEST                         : u16 = 0x1044; /* Defined in ANSI E1.37-1                                     */


/// UID Struct
/// This contains the manufacturer and device ids
/// Implementations will include the ability to format as MMMM:DDDDDDDD as well as strings of bytes
/// 
#[derive(Copy, Clone)]
pub struct Uid {
    pub mfg : u16,
    pub dev : u32
}

impl Uid {

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
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
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

fn do_discovery_node(f: fn(&[u8]) -> Option<&[u8]>,min: &Uid, max: &Uid) -> DiscoveryResponse {

    let mut buffer : [u8;256] = [0;256];

    f(&buffer[..]);

    /*
    139) "3638:08101AD8"
    140) "646F:000E8E22"
    141) "646F:000FB190"
    142) "3638:27106D31"
    143) "3638:0B101307"
    144) "646F:000FB118"
    145) "6574:1B69D0FE"
    146) "646F:000FA98D"
    147) "3638:4110280F"
    148) "3638:0B101323"
    */

    let uid_list : Vec<Uid> = vec![Uid::new(0x3638,0x08101AD8),
    Uid::new(0x646F,0x000E8E22),
    Uid::new(0x646F,0x000FB190),
    Uid::new(0x3638,0x27106D31),
    Uid::new(0x3638,0x0B101307),
    Uid::new(0x646F,0x000FB118),
    Uid::new(0x6574,0x1B69D0FE),
    Uid::new(0x646F,0x000FA98D),
    Uid::new(0x3638,0x4110280F),
    Uid::new(0x3638,0x0B101323)];

    let mut uid_found : Vec<Uid> = Vec::new();


    for uid in uid_list {
        if (uid <= *max) && (uid >= *min) {
            // println!("Checking between {} and {} and found {}",min,max,uid);
            uid_found.push(uid);
        }
    }

    if uid_found.len() == 0 {
        DiscoveryResponse::None
    } else if uid_found.len() == 1 {
        DiscoveryResponse::One(uid_found[0])
    } else {
        DiscoveryResponse::Some
    }


}

/// Runs the discovery algorithm.
/// 0. Optionally: Unmute all (out of scope)
/// 1. Do allcall discovery.   If no response, then return empty Vec If response, go to 2
/// 2. Check left-hand side of tree and gather UIDs
/// 3. Check right-hand side of tree and gather UIDs
/// 4. Concatenate and return
pub fn do_discovery_algo(f: fn(&[u8]) -> Option<&[u8]>) -> Vec<Uid> {

    let min : Uid = Uid::new(0,0); 
    let max : Uid = Uid::new(0x7FFF, 0xFFFF_FFFF);

    let tod = do_discovery_recursion(f, &min, &max);

    return tod;
}

fn do_discovery_recursion(f: fn(&[u8]) -> Option<&[u8]>, min: &Uid, max: &Uid) -> Vec<Uid> {
    let mut tod : Vec<Uid> = Vec::new();

    println!("do_discovery_recursion({},{})",min,max);

    match do_discovery_node(f,min, max) {
        DiscoveryResponse::None => { 
            return tod; // nothing in this branch, go back up.
        },
        DiscoveryResponse::One(found_uid) => {
            println!("Found {}, muting it.",found_uid);
            // FIXME: add mute message here
            tod.push(found_uid);
            return tod; // only one thing here, return it.
         },
        DiscoveryResponse::Some => { 
            // println!("Found some responders, digging deeper");
            // need to dig deeper, so don't return.
        }
    }

    let mid = min.get_midpoint(max);

    // println!("Midpoint is {}", mid);

    tod.append(do_discovery_recursion(f,min,&mid).as_mut());
    tod.append(do_discovery_recursion(f,&mid,max).as_mut());
    
    return tod;

}