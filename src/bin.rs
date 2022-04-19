use e1_20;

// OK, so we need to pass a function that can accept a string of RDM data and then return the response.   
// Response should actually be an Option so we can do Some or None 

fn fake_rdm(data: &[u8]) -> Option<&[u8]> {

    println!("{:?}",data);

    return None;
}


fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");

    println!("{:?}", e1_20::do_discovery_algo(fake_rdm));

}