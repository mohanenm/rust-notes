const START_MISSILES: i32 = 8; 
const READY_AMOUNT: i32 = 2; 
fn main() {
    let mut missiles = START_MISSILES; 
    let ready = READY_MISSILES;
    println!("Firing {} of my {} missiles...", ready, missiles);
    
    let missiles = missiles - ready; 
    println!("{} missiles left", missiles);

}

