mod data;
mod handle;
mod settings;
mod sound;

pub use data::*;
pub use handle::*;
pub use settings::*;

use std::collections::VecDeque;

use kira::{dsp::Frame, parameter::Tween};

pub trait Decoder: Send + Sync {
	fn sample_rate(&mut self) -> u32;

	fn decode(&mut self) -> Option<VecDeque<Frame>>;

	fn reset(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
	Pause(Tween),
	Resume(Tween),
	Stop(Tween),
}
