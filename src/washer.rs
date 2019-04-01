use futures::unsync::oneshot::Sender;
use telegram_bot_fork::UserId;

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
        user: UserId,
    },
    Finished,
    Idle,
}

impl Washer {
    /// Creates a new `Washer` in the `WasherState::Idle` state.
    pub fn new() -> Washer {
        Washer {
            state: WasherState::Idle,
        }
    }

    /// Starts the `Washer`, changing its `state` to `WasherState::Running`.
    /// Start time will be set to current time and a `Timer` will be created.
    ///
    /// # Panics
    /// When `state` is not `WasherState::Idle`.
    pub fn start(&mut self, program: &Program, user: UserId) -> Timer {
        let now = Instant::now();
        let (timer, cancel_timer) = Timer::new(now + program.duration);
        match self.state {
            WasherState::Idle => {
                self.state = WasherState::Running {
                    program: program.clone(),
                    start_time: now,
                    cancel_timer,
                    user,
                }
            }
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
                if cancel_timer.send(()).is_err() {
                    eprintln! {"Receiver was dropped."};
                }
                self.state = WasherState::Idle
            }
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

    /// Returns remaining time or `None` when timer has finished.
    ///
    /// # Panics
    /// When `state` is not `WasherState::Running`
    pub fn remaining_time(&self) -> Option<Duration> {
        match &self.state {
            WasherState::Running { cancel_timer, .. } if cancel_timer.is_canceled() => None,
            WasherState::Running {
                start_time,
                program,
                ..
            } => Some(*start_time + program.duration - Instant::now()),
            _ => panic!("Can get time of a 'Running' Washer only"),
        }
    }

    /// Changes `state` to `WasherState::Idle`. To be called when the washer
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

    /// Returns the `WasherState` the `Washer` is in.
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
        Program { name, duration }
    }
}
