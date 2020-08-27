// This is an example of three-party Diffie-Hellman key exchange
// Requires two rounds

extern crate rabe_bn;
extern crate rand;

use rabe_bn::*;
use rand::Rng;
use rabe_bn::Gt;

fn main() {
    let mut rng = rand::thread_rng();

    // Construct private keys
    let alice_sk:Gt = rng.gen();
    println!("alice_sk: {}", alice_sk)
}
