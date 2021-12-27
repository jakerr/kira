//! Precise timing for audio events.

pub(crate) mod clocks;
mod handle;
mod time;

pub use handle::*;
pub use time::*;

use std::sync::{
	atomic::{AtomicBool, AtomicU64, Ordering},
	Arc,
};

use atomic_arena::Key;

use crate::{
	tween::{Tween, Tweener},
	ClockSpeed,
};

/// A unique identifier for a clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClockId(pub(crate) Key);

pub(crate) struct ClockShared {
	ticking: AtomicBool,
	ticks: AtomicU64,
	removed: AtomicBool,
}

impl ClockShared {
	pub fn new() -> Self {
		Self {
			ticking: AtomicBool::new(false),
			ticks: AtomicU64::new(0),
			removed: AtomicBool::new(false),
		}
	}

	pub fn ticking(&self) -> bool {
		self.ticking.load(Ordering::SeqCst)
	}

	pub fn ticks(&self) -> u64 {
		self.ticks.load(Ordering::SeqCst)
	}

	pub fn is_marked_for_removal(&self) -> bool {
		self.removed.load(Ordering::SeqCst)
	}

	pub fn mark_for_removal(&self) {
		self.removed.store(true, Ordering::SeqCst);
	}
}

enum State {
	NotStarted,
	Started { ticks: u64 },
}

pub(crate) struct Clock {
	shared: Arc<ClockShared>,
	ticking: bool,
	speed: Tweener<ClockSpeed>,
	state: State,
	tick_timer: f64,
}

impl Clock {
	pub(crate) fn new(speed: ClockSpeed) -> Self {
		Self {
			shared: Arc::new(ClockShared::new()),
			ticking: false,
			speed: Tweener::new(speed),
			state: State::NotStarted,
			tick_timer: 1.0,
		}
	}

	pub(crate) fn shared(&self) -> Arc<ClockShared> {
		self.shared.clone()
	}

	pub(crate) fn set_speed(&mut self, speed: ClockSpeed, tween: Tween) {
		self.speed.set(speed, tween);
	}

	pub(crate) fn start(&mut self) {
		self.ticking = true;
		self.shared.ticking.store(true, Ordering::SeqCst);
	}

	pub(crate) fn pause(&mut self) {
		self.ticking = false;
		self.shared.ticking.store(false, Ordering::SeqCst);
	}

	pub(crate) fn stop(&mut self) {
		self.pause();
		self.state = State::NotStarted;
		self.shared.ticks.store(0, Ordering::SeqCst);
	}

	pub(crate) fn update(&mut self, dt: f64) -> Option<u64> {
		self.speed.update(dt);
		if !self.ticking {
			return None;
		}
		let mut new_tick_count = None;
		self.tick_timer -= self.speed.value().as_ticks_per_second() * dt;
		while self.tick_timer <= 0.0 {
			self.tick_timer += 1.0;
			let tick_count = match &mut self.state {
				State::NotStarted => {
					self.state = State::Started { ticks: 0 };
					0
				}
				State::Started { ticks } => {
					*ticks += 1;
					*ticks
				}
			};
			self.shared.ticks.store(tick_count, Ordering::SeqCst);
			new_tick_count = Some(tick_count);
		}
		new_tick_count
	}

	pub(crate) fn on_clock_tick(&mut self, time: ClockTime) {
		self.speed.on_clock_tick(time);
	}
}
