use futures::unsync::oneshot::Sender;

use std::mem;
use std::time::{Duration, Instant};

use crate::timer::Timer;

/// A washer implemented as a state machine.
#[derive(Debug)]
pub struct Washer {
    state: WasherState,
}

/// The states a washer can be in.
#[derive(Debug)]
pub enum WasherState {
    Running {
        program: Program,
        start_time: Instant,
        cancel_timer: Sender<()>,
    },
    Finished,
    Idle,
}

impl Washer {
    /// Creates a new `Washer` in the `WasherState::Idle` state.
    pub fn new() -> Washer {
        Washer { state: WasherState::Idle }
    }

    /// Starts the `Washer`, changing its `state` to `WasherState::Running`. 
    /// Start time will be set to current time and a `Timer` will be created.
    /// 
    /// # Panics
    /// When `state` is not `WasherState::Idle`.
    pub fn start(&mut self, program: &Program) -> Timer {
        let now = Instant::now();
        let (timer, cancel_timer) = Timer::new(now + program.duration);
        match self.state {
            WasherState::Idle => self.state = WasherState::Running {
                program: program.clone(),
                start_time: now,
                cancel_timer,
            },
            _ => panic!("Can call start on an 'Idle' Washer only"),
        }
        timer
    }

    /// Stops the `Washer` prematurely, changing its `state` to
    /// `WasherState::Idle`.
    /// 
    /// # Panics
    /// When `state` is not `WasherState::Running`
    pub fn stop(&mut self) {
        let state = mem::replace(&mut self.state, WasherState::Idle);
        match state {
            WasherState::Running { cancel_timer, .. } => {
                if let Err(_) = cancel_timer.send(()) {
                    eprintln!{"Receiver was dropped."};
                }
                self.state = WasherState::Idle
            },
            _ => panic!("Can call stop on a 'Running' Washer only"),
        }
    }

    /// Stops the `Washer`. `state` is changed to
    /// `WasherState::Finished`.
    /// Call this when time was up.
    ///
    /// # Panics
    /// When `state` is not `WasherState::Running`
    pub fn finish(&mut self) {
        match self.state {
            WasherState::Running { .. } => self.state = WasherState::Finished,
            _ => panic!("Can call finish on a 'Running' Washer only"),
        }
    }

    /// Return remaining time.
    ///
    /// # Panics
    /// When `state` is not `WasherState::Running`
    pub fn remaining_time(&self) -> Duration {
        match &self.state {
            WasherState::Running { start_time, .. } => 
                    Instant::now() - *start_time,
            _ => panic!("Can get time of a 'Running' Washer only"),
        }
        
    }

    /// Change `state` to `WasherState::Idle`. To be called when Washer
    /// was emptied.
    ///
    /// # Panics
    /// When `state` is not `WasherState::Finished`
    pub fn empty(&mut self) {
        match self.state {
            WasherState::Finished { .. } => self.state = WasherState::Idle,
            _ => panic!("Can call empty on a 'Finished' Washer only"),
        }
    }

    pub fn state(&self) -> &WasherState {
        &self.state
    }
}

/// A program a washer can execute.
#[derive(Debug, Clone)]
pub struct Program {
    name: String,
    duration: Duration,
}

impl Program {
    /// Creates a new `Program`.
    pub fn new(name: String, duration: Duration) -> Program {
        Program {
            name,
            duration,
        }
    }
}
