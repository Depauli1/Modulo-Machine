use num_bigint::BigUint;
use num_traits::FromPrimitive;
use std::io;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn calculate_modulus(x: BigUint, p: BigUint) -> BigUint {
    x % p
}

fn main() {
    let x = Arc::new(Mutex::new(BigUint::from_str("123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890").unwrap())); // replace "..." with your 300-bit number
    let p = Arc::new(Mutex::new(
        BigUint::from_str(
            "104899928942039473597645237135751317405745389583683433800060134911610808289117",
        )
        .unwrap(),
    ));
    let o = Arc::new(Mutex::new(BigUint::from_u8(0).unwrap()));

    let x_clone = Arc::clone(&x);
    let p_clone = Arc::clone(&p);
    let o_clone = Arc::clone(&o);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1)); // this simulates a clock signal every second
            let mut o_lock = o_clone.lock().unwrap();
            let x_lock = x_clone.lock().unwrap();
            let p_lock = p_clone.lock().unwrap();
            *o_lock = calculate_modulus(x_lock.clone(), p_lock.clone());
        }
    });

    loop {
        if reset_signal_received() {
            let mut x_lock = x.lock().unwrap();
            let mut p_lock = p.lock().unwrap();
            *x_lock = BigUint::from_str("987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210").unwrap(); 
            *p_lock = BigUint::from_str(
                "104899928942039473597645237135751317405745389583683433800060134911610808289117",
            )
            .unwrap(); 
        }
    }
}

fn reset_signal_received() -> bool {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim() == "reset" {
                return true;
            }
        }
        Err(_) => {}
    }
    false
}
