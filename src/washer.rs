use std::time::{Duration, Instant};

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
    },
    Finished,
    Idle,
}

impl Washer {
    /// Create a new `Washer` in the `WasherState::Idle` state.
    pub fn new() -> Washer {
        Washer { state: WasherState::Idle }
    }

    /// Start the `Washer`, changing its `state` to `WasherState::Running`. 
    /// Start time will be set to current time.
    /// 
    /// # Panics
    /// When `state` is not `WasherState::Idle`.
    pub fn start(&mut self, program: Program) {
        match self.state {
            WasherState::Idle => self.state = WasherState::Running {
                program,
                start_time: Instant::now(),
            },
            _ => panic!("Can call start on an 'Idle' Washer only"),
        }
    }

    /// Stop the `Washer` immediately, changing its `state` to
    /// `WasherState::Idle`.
    /// Call this when the washer program was prematurely aborted.
    /// 
    /// # Panics
    /// When `state` is not `WasherState::Running`
    pub fn stop(&mut self) {
        match self.state {
            WasherState::Running { .. } => self.state = WasherState::Idle,
            _ => panic!("Can call stop on a 'Running' Washer only"),
        }
    }

    /// Stop the `Washer` when time is up. `state` is changed to
    /// `WasherState::Finished`.
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
}

/// A program a washer can execute.
#[derive(Debug)]
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
