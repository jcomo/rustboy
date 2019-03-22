use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

/// A clock is used as a physical timing mechanism for the GameBoy.
/// Each tick gets the number of cycles passed and can slow down or
/// speed up emulation depending on the implementation.
pub trait Clock {
    fn tick(&mut self, cycles: u8);
}

/// A clock intended to emulate a slower CPU clock. Every few milliseconds,
/// it reconciles the acttual elapsed time with the expected elapsed time and
/// sleeps if necessary to sync up the difference.
pub struct WallClock {
    start: Instant,
    cycles: u64,
    ns_per_cycle: u64,
    cycles_per_tick: u64,
}

impl WallClock {
    pub fn new(hz: u64) -> WallClock {
        WallClock {
            start: Instant::now(),
            cycles: 0,
            ns_per_cycle: 1_000_000_000 / hz,
            // Attempt to sync every 5 ms
            cycles_per_tick: hz / 5_000,
        }
    }

    pub fn z80() -> WallClock {
        WallClock::new(1_048_000) // 1.048 MHz
    }

    fn needs_sync(&self) -> bool {
        self.cycles >= self.cycles_per_tick
    }

    fn sync(&self) {
        let elapsed = self.start.elapsed();
        let duration = self.expected_duration();
        duration.checked_sub(elapsed).map(|delta| {
            sleep(delta);
        });
    }

    fn reset(&mut self) {
        self.start = Instant::now();
        self.cycles = self.cycles % self.cycles_per_tick;
    }

    fn expected_duration(&self) -> Duration {
        let nanos = self.cycles * self.ns_per_cycle;
        Duration::from_nanos(nanos)
    }
}

impl Clock for WallClock {
    fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as u64;

        if self.needs_sync() {
            self.sync();
            self.reset();
        }
    }
}

/// A clock that runs as fast as the host
pub struct NoClock;

impl Clock for NoClock {
    fn tick(&mut self, cycles: u8) {
        // Do nothing
    }
}
