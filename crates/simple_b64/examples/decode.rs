fn main() {
    // It can decode base64 messages that use padding:
    let message = "QWxsIGhhaWwgdGhlIG1pZ2h0eSBjcmFiIQ==";
    let decoded_message = simple_b64::decode(message).unwrap();
    println!("{decoded_message}");
    assert_eq!("All hail the mighty crab!", decoded_message);

    // It also can decode base64 messages with the padding omitted:
    let message = "QWxsIGhhaWwgdGhlIG1pZ2h0eSBjcmFiIQ";
    let decoded_message = simple_b64::decode(message).unwrap();
    println!("{decoded_message}");
    assert_eq!("All hail the mighty crab!", &decoded_message);

    // But it will always need a full and valid packet of base64. Otherwise it will fail.
}
