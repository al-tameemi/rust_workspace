use bity::Bity;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Bity, Deserialize)]
struct Tester {
    #[bit_order(little)]
    #[byte_order(big)]
    field: i32,
    field_2: i32,
}

fn main() {
    let _tester = Tester {
        field: 0,
        field_2: 0,
    };
}
