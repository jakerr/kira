use std::{error::Error, fmt::Display, sync::Arc};

use kira::{parameter::Tween, value::Value};
use ringbuf::Producer;

use crate::{sound::Shared, Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandQueueFull;

impl Display for CommandQueueFull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Cannot send a command to the sound because the command queue is full")
	}
}

impl Error for CommandQueueFull {}

pub struct StreamingSoundHandle {
	pub(crate) shared: Arc<Shared>,
	pub(crate) command_producer: Producer<Command>,
}

impl StreamingSoundHandle {
	pub fn position(&self) -> f64 {
		self.shared.position()
	}

	pub fn set_volume(&mut self, volume: impl Into<Value>) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::SetVolume(volume.into()))
			.map_err(|_| CommandQueueFull)
	}

	pub fn set_playback_rate(
		&mut self,
		playback_rate: impl Into<Value>,
	) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::SetPlaybackRate(playback_rate.into()))
			.map_err(|_| CommandQueueFull)
	}

	pub fn set_panning(&mut self, panning: impl Into<Value>) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::SetPanning(panning.into()))
			.map_err(|_| CommandQueueFull)
	}

	pub fn pause(&mut self, tween: Tween) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::Pause(tween))
			.map_err(|_| CommandQueueFull)
	}

	pub fn resume(&mut self, tween: Tween) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::Resume(tween))
			.map_err(|_| CommandQueueFull)
	}

	pub fn stop(&mut self, tween: Tween) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::Stop(tween))
			.map_err(|_| CommandQueueFull)
	}

	pub fn seek_to(&mut self, position: f64) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::SeekTo(position))
			.map_err(|_| CommandQueueFull)
	}

	pub fn seek_by(&mut self, amount: f64) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::SeekBy(amount))
			.map_err(|_| CommandQueueFull)
	}
}