#![allow(dead_code)]

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
pub struct Uid {
    pub mfg : u16,
    pub dev : u32
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
    pub mdb : Vec<u8>,
    pub checksum : u16
}

// Next steps here is to decide on a format for mdb.
// I think more apropriate will be an ENUM of variuos structus
// There's a few common MDB formats that can be encoded as enums
// I think we *either* have a generic PD with CC/PID/PDL or we 
// have enums for each packet type we support
// I'm not sure which approach is better.