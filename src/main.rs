use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve,
    },
    traits::AsBytes,
};

/**
 * Function that takes the vec array of bytes and converts it to a hex string (GPT generated)
 */
fn bytes_to_hex_string(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    let private_key: u64 = 0x6C616D6264617370;

    // get the generator point
    let curve = BLS12381Curve::generator();

    // to get the public key we multiply the generator point by the private key
    let public_key = curve.operate_with_self(private_key);

    // get our x and y
    let x = &public_key.x();
    let y = &public_key.y();

    // answer time
    println!("x: {:?}", bytes_to_hex_string(x.as_bytes()));
    println!("y: {:?}", bytes_to_hex_string(y.as_bytes()));
}
