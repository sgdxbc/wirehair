fn main() -> Result<(), wirehair::Error> {
    const PACKET_SIZE: u32 = 1400;
    const MESSAGE_BYTES: u64 = 1000 * 1000 + 333;
    // const PACKET_SIZE: u32 = 1 << 22;
    // const MESSAGE_BYTES: u64 = 1 << 28;

    let message = vec![1; MESSAGE_BYTES as _];
    // let start = std::time::Instant::now();
    let encoder = wirehair::Encoder::new(message.clone(), PACKET_SIZE)?;
    // println!("{:?}", start.elapsed());
    let mut decoder = wirehair::Decoder::new(MESSAGE_BYTES, PACKET_SIZE)?;
    // println!("{:?}", start.elapsed());
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
