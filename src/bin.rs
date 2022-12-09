#[macro_use]
extern crate log;
extern crate simplelog;

use e1_20::*;

use simplelog::*;

// OK, so we need to pass a function that can accept a string of RDM data and then return the response.
// Response should actually be an Option so we can do Some or None

fn fake_rdm(data: &[u8]) -> Option<Vec<u8>> {
    // println!("fake_rdm: {:?}",data);

    let uid_list: Vec<Uid> = vec![
        Uid::new(0x3638, 0x08101AD8),
        Uid::new(0x646F, 0x000E8E22),
        Uid::new(0x646F, 0x000FB190),
        Uid::new(0x3638, 0x27106D31),
        Uid::new(0x3638, 0x0B101307),
        Uid::new(0x646F, 0x000FB118),
        Uid::new(0x6574, 0x1B69D0FE),
        Uid::new(0x646F, 0x000FA98D),
        Uid::new(0x3638, 0x4110280F),
        Uid::new(0x3638, 0x0B101323),
    ];

    let packet = Pkt::deserialize(data.to_vec()).unwrap();

    if packet.pid == DISC_UNIQUE_BRANCH {
        let min = Uid::from_bytes(&packet.pd[0..6]);
        let max = Uid::from_bytes(&packet.pd[6..12]);

        let mut uid_found: Vec<Uid> = Vec::new();

        for uid in uid_list {
            if (uid <= max) && (uid >= min) {
                // println!("Checking between {} and {} and found {}",min,max,uid);
                uid_found.push(uid);
            }
        }

        if uid_found.len() == 0 {
            return None;
        } else if uid_found.len() == 1 {
            debug!("Found one: {}", uid_found[0]);
            let uid_buffer = uid_found[0].uid_serialize();
            let mut buffer: [u8; 24] = [0xFE; 24];
            buffer[7] = 0xAA;

            buffer[8] = uid_buffer[0] | 0xAA;
            buffer[9] = uid_buffer[0] | 0x55;

            buffer[10] = uid_buffer[1] | 0xAA;
            buffer[11] = uid_buffer[1] | 0x55;

            buffer[12] = uid_buffer[2] | 0xAA;
            buffer[13] = uid_buffer[2] | 0x55;

            buffer[14] = uid_buffer[3] | 0xAA;
            buffer[15] = uid_buffer[3] | 0x55;

            buffer[16] = uid_buffer[4] | 0xAA;
            buffer[17] = uid_buffer[4] | 0x55;

            buffer[18] = uid_buffer[5] | 0xAA;
            buffer[19] = uid_buffer[5] | 0x55;

            let mut crc: u16 = 0;

            for byte in &buffer[8..20] {
                crc = crc.overflowing_add(*byte as u16).0;
            }

            let crc_buffer = crc.to_be_bytes();

            buffer[20] = crc_buffer[0] | 0xAA;
            buffer[21] = crc_buffer[0] | 0x55;

            buffer[22] = crc_buffer[1] | 0xAA;
            buffer[23] = crc_buffer[1] | 0x55;

            return Some(buffer.to_vec());
        } else {
            return Some(data.to_vec());
        }
    }

    if packet.pid == DISC_MUTE {
        for uid in uid_list {
            if packet.destination == uid {
                debug!("Got DISC_MUTE for {}", uid);
                let mut response = Pkt::new();
                response.source = uid;
                response.destination = packet.source;
                response.tn = packet.tn;
                response.port_or_response_type = RESPONSE_TYPE_ACK;
                response.cc = DISCOVERY_COMMAND_RESPONSE;
                response.pid = DISC_MUTE;
                response.set_message_length(); // sets message length from PDL
                response.set_checksum(); // sets checksum from the whole packet.

                let response_data = response.serialize();

                return Some(response_data);
            }
        }
    }

    return None;
}

fn main() {
    // Statements here are executed when the compiled binary is called

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ), // Filter to debug, output to termial
    ])
    .unwrap();

    // Print text to the console
    println!("Hello World!");

    let my_uid = Uid::new(0x044E, 0x01);

    println!("{:?}", do_discovery_algo(fake_rdm, &my_uid, false, false));
}
