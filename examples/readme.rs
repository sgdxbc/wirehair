fn main() -> Result<(), wirehair::Error> {
    const PACKET_SIZE: u32 = 1400;
    const MESSAGE_BYTES: u64 = 1000 * 1000 + 333;

    let message = vec![1; MESSAGE_BYTES as _];
    let encoder = wirehair::Encoder::new(&message, PACKET_SIZE)?;
    let mut decoder = wirehair::Decoder::new(MESSAGE_BYTES, PACKET_SIZE)?;
    let mut needed = 0;
    for block_id in 0.. {
        if block_id % 10 == 0 {
            continue;
        }
        needed += 1;
        let packet = encoder.encode(block_id)?;
        if decoder.decode(block_id, &packet)? {
            break;
        }
    }
    let decoded = decoder.recover()?;

    println!("needed {needed}");
    println!("decoded == message {}", decoded == message);
    Ok(())
}
