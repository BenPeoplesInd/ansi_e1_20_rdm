use e1_20::*;

// OK, so we need to pass a function that can accept a string of RDM data and then return the response.   
// Response should actually be an Option so we can do Some or None 

fn fake_rdm(data: &[u8]) -> Option<&[u8]> {

    println!("fake_rdm: {:?}",data);

    // let uid_list : Vec<Uid> = vec![Uid::new(0x3638,0x08101AD8),
    // Uid::new(0x646F,0x000E8E22),
    // Uid::new(0x646F,0x000FB190),
    // Uid::new(0x3638,0x27106D31),
    // Uid::new(0x3638,0x0B101307),
    // Uid::new(0x646F,0x000FB118),
    // Uid::new(0x6574,0x1B69D0FE),
    // Uid::new(0x646F,0x000FA98D),
    // Uid::new(0x3638,0x4110280F),
    // Uid::new(0x3638,0x0B101323)];

    // let packet = Pkt::deserialize(data.to_vec()).unwrap();

    // if packet.pid == DISC_UNIQUE_BRANCH {
    //     let min = Uid::from_bytes(&packet.pd[0..6]);
    //     let max = Uid::from_bytes(&packet.pd[6..12]);

    //     let mut uid_found : Vec<Uid> = Vec::new();

    //     for uid in uid_list {
    //         if (uid <= max) && (uid >= min) {
    //             // println!("Checking between {} and {} and found {}",min,max,uid);
    //             uid_found.push(uid);
    //         }
    //     }

    //     if uid_found.len() == 0 {
    //         return None;
    //     } else if uid_found.len() == 1 {
    //         return None;
    //     } else {
    //         return Some(data);
    //     }


    // }


    

    return None;
}


fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");

    println!("{:?}", do_discovery_algo(fake_rdm));

   
}