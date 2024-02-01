use std::{
    error::Error,
    iter::repeat,
    sync::{
        atomic::{AtomicU32, Ordering::SeqCst},
        mpsc::channel,
        Arc,
    },
    thread::{available_parallelism, spawn},
    time::Instant,
};

use wirehair::{Decoder, Encoder};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let message = vec![0xa5; 1 << 30];
    let start = Instant::now();
    let encoder = Arc::new(Encoder::new(message.clone(), 1 << 20)?);
    // drop(message);
    println!("{:?}", start.elapsed());

    let mut decoder = wirehair::Decoder::new(message.len() as _, 1 << 20)?;
    for i in 0.. {
        let block = encoder.encode(i)?;
        if decoder.decode(i, block)? {
            break;
        }
    }
    let encoder = Arc::new(Encoder::try_from(decoder)?);

    let block_id = Arc::new(AtomicU32::new(117418));
    let (sender, reciever) = channel();
    for (encoder, block_id, sender) in repeat((encoder, block_id, sender))
        .take(available_parallelism()?.get() - 1)
        // .take(1)
    {
        spawn(move || loop {
            let id = block_id.fetch_add(1, SeqCst);
            if id >= 117418 + (1 << 16) {
                break Ok::<_, wirehair::Error>(());
            }
            let start = Instant::now();
            let block = encoder.encode(id)?;
            sender.send((start.elapsed(), id, block)).unwrap()
        });
    }
    let mut decoder = Decoder::new(message.len() as _, 1 << 20)?;
    let mut decoded = false;
    for (i, (time, id, block)) in reciever.iter().enumerate() {
        if i % (1 << 6) != 0 {
            continue;
        }
        println!("{time:?}");
        if !decoded {
            decoded = decoder.decode(id, block)?;
            if decoded {
                println!("(decoded)")
            }
        }
    }
    Ok(())
}
