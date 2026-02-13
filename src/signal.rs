use crate::error::DaemonError;
use tokio::sync::mpsc;

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Shutdown,
    ReloadConfig,
}

pub struct SignalHandler {
    rx: mpsc::UnboundedReceiver<Signal>,
}

impl SignalHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        
        #[cfg(unix)]
        {
            use signal_hook::consts::SIGTERM;
            use signal_hook::consts::SIGINT;
            use signal_hook::consts::SIGHUP;
            use signal_hook::iterator::Signals;
            use std::thread;
            
            if let Ok(mut signals) = Signals::new(&[SIGTERM, SIGINT, SIGHUP]) {
                let tx_clone = tx.clone();
                thread::spawn(move || {
                    for sig in signals.forever() {
                        let signal = match sig as i32 {
                            SIGTERM | SIGINT => Signal::Shutdown,
                            SIGHUP => Signal::ReloadConfig,
                            _ => continue,
                        };
                        let _ = tx_clone.send(signal);
                    }
                });
            }
        }
        
        Self { rx }
    }
    
    pub async fn recv(&mut self) -> Option<Signal> {
        self.rx.recv().await
    }
}
