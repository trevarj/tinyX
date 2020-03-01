use futures::{pin_mut, select, FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::spawn_local;

use crate::TUI;

pub struct SpellChecker {
    delay: Duration,
    tui: TUI,
    // Only available when there's a spell check timer running.
    snd_stop: Option<mpsc::Sender<()>>,
}

impl SpellChecker {
    pub fn new(delay: Duration, tui: TUI) -> SpellChecker {
        SpellChecker {
            delay,
            tui,
            snd_stop: None,
        }
    }

    /// Start (or restart) the timer that spell checks TUI's current tab's `text_field` contents on
    /// timeout.
    pub fn reset(&mut self) {
        self.stop();

        let tui = self.tui.clone();
        let delay = self.delay;
        let (snd_stop, rcv_stop) = mpsc::channel(1);
        self.snd_stop = Some(snd_stop);

        spawn_local(async move {
            let mut rcv_stop_fused = rcv_stop.fuse();
            loop {
                let timer = tokio::time::delay_for(delay);

                let mut timer_fused = timer.fuse();

                select! {
                    () = timer_fused => {
                        tui.spell_check();
                    }
                    _ = rcv_stop_fused.next() => {
                        return;
                    }
                }
            }
        });
    }

    /// Stop the spell checker timer.
    pub fn stop(&mut self) {
        if let Some(snd_stop) = self.snd_stop.as_mut() {
            let _ = snd_stop.try_send(());
        }
    }
}
