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

use wirehair::Encoder;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let message = vec![0xa5; 1 << 30];
    let start = Instant::now();
    let encoder = Arc::new(Encoder::new(&message, 1 << 20)?);
    println!("{:?}", start.elapsed());
    let block_id = Arc::new(AtomicU32::new(0));
    let (sender, reciever) = channel();
    for (encoder, block_id, sender) in
        repeat((encoder, block_id, sender)).take(available_parallelism()?.get())
    {
        spawn(move || loop {
            let id = block_id.fetch_add(1, SeqCst);
            if id >= 1 << 18 {
                break Ok::<_, wirehair::Error>(());
            }
            let start = Instant::now();
            encoder.encode(id)?;
            sender.send(start.elapsed()).unwrap()
        });
    }
    for (i, time) in reciever.iter().enumerate() {
        if i % (1 << 12) == 0 {
            println!("{time:?}")
        }
    }
    Ok(())
}
