//! An interface for controlling sounds.

use crate::{
	command::{
		producer::{CommandProducer, CommandProducerError},
		InstanceCommand,
	},
	instance::{
		handle::InstanceHandle, Instance, InstanceSettings, PauseInstanceSettings,
		ResumeInstanceSettings, StopInstanceSettings,
	},
	mixer::TrackIndex,
};

use super::{Sound, SoundId};

/// Allows you to control a sound.
#[derive(Debug, Clone)]
pub struct SoundHandle {
	id: SoundId,
	duration: f64,
	default_track: TrackIndex,
	semantic_duration: Option<f64>,
	default_loop_start: Option<f64>,
	command_sender: CommandProducer,
}

impl SoundHandle {
	pub(crate) fn new(sound: &Sound, command_sender: CommandProducer) -> Self {
		Self {
			id: sound.id(),
			duration: sound.duration(),
			default_track: sound.default_track(),
			semantic_duration: sound.semantic_duration(),
			default_loop_start: sound.default_loop_start(),
			command_sender,
		}
	}

	/// Returns the ID of the sound.
	pub fn id(&self) -> SoundId {
		self.id
	}

	/// Returns the duration of the sound (in seconds).
	pub fn duration(&self) -> f64 {
		self.duration
	}

	/// Returns the default track instances of this
	/// sound will play on.
	pub fn default_track(&self) -> TrackIndex {
		self.default_track
	}

	/// Returns the "musical length" of the sound (if there
	/// is one).
	pub fn semantic_duration(&self) -> Option<f64> {
		self.semantic_duration
	}

	/// Returns the default time (in seconds) instances
	/// of this sound will loop back to when they reach
	/// the end.
	pub fn default_loop_start(&self) -> Option<f64> {
		self.default_loop_start
	}

	/// Plays the sound.
	pub fn play(
		&mut self,
		settings: InstanceSettings,
	) -> Result<InstanceHandle, CommandProducerError> {
		let id = settings.id;
		let instance = Instance::new(
			self.id.into(),
			self.duration,
			None,
			settings.into_internal(self.duration, self.default_loop_start, self.default_track),
		);
		let handle = InstanceHandle::new(id, instance.public_state(), self.command_sender.clone());
		self.command_sender
			.push(InstanceCommand::Play(id, instance).into())?;
		Ok(handle)
	}

	/// Pauses all instances of this sound.
	pub fn pause(&mut self, settings: PauseInstanceSettings) -> Result<(), CommandProducerError> {
		self.command_sender
			.push(InstanceCommand::PauseInstancesOf(self.id.into(), settings).into())
	}

	/// Resumes all instances of this sound.
	pub fn resume(&mut self, settings: ResumeInstanceSettings) -> Result<(), CommandProducerError> {
		self.command_sender
			.push(InstanceCommand::ResumeInstancesOf(self.id.into(), settings).into())
	}

	/// Stops all instances of this sound.
	pub fn stop(&mut self, settings: StopInstanceSettings) -> Result<(), CommandProducerError> {
		self.command_sender
			.push(InstanceCommand::StopInstancesOf(self.id.into(), settings).into())
	}
}
