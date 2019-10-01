use signal_hook::{iterator::Signals, SIGINT};
use std::{error::Error, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    Ok(())
}
