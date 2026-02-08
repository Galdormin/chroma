use bevy::{audio::Volume, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AudioSettings>();
    app.add_systems(
        Update,
        apply_audio_settings.run_if(resource_changed::<AudioSettings>),
    );
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum VolumeType {
    Master,
    Music,
    Sfx,
}

impl ToString for VolumeType {
    fn to_string(&self) -> String {
        match self {
            VolumeType::Master => "Master",
            VolumeType::Music => "Music",
            VolumeType::Sfx => "SFX",
        }
        .into()
    }
}

/// Taken and adapted from https://github.com/benfrankel/pyri_new_jam/blob/main/src/core/audio.rs
#[derive(Resource, Reflect, Clone, Debug)]
#[reflect(Resource)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.5,
            music_volume: 0.5,
            sfx_volume: 0.5,
        }
    }
}

impl AudioSettings {
    pub fn set_volume(&mut self, volume_type: VolumeType, value: f32) {
        match volume_type {
            VolumeType::Master => self.master_volume = value,
            VolumeType::Music => self.music_volume = value,
            VolumeType::Sfx => self.sfx_volume = value,
        }
    }

    pub fn get_volume(&self, volume_type: VolumeType) -> f32 {
        match volume_type {
            VolumeType::Master => self.master_volume,
            VolumeType::Music => self.music_volume,
            VolumeType::Sfx => self.sfx_volume,
        }
    }

    pub fn music_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.music_volume)
    }

    pub fn sfx_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.sfx_volume)
    }
}

fn apply_audio_settings(
    audio_settings: Res<AudioSettings>,
    music_audio_query: Query<Entity, With<Music>>,
    sfx_audio_query: Query<Entity, With<SoundEffect>>,
    mut volume_query: Query<(Option<&mut PlaybackSettings>, Option<&mut AudioSink>)>,
) {
    // Apply music volume.
    let volume = audio_settings.music_volume();
    for entity in &music_audio_query {
        let Ok((playback, sink)) = volume_query.get_mut(entity) else {
            continue;
        };

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }

    // Apply SFX volume.
    let volume = audio_settings.sfx_volume();
    for entity in &sfx_audio_query {
        let Ok((playback, sink)) = volume_query.get_mut(entity) else {
            continue;
        };

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>, audio_settings: &AudioSettings) -> impl Bundle {
    (
        AudioPlayer(handle),
        PlaybackSettings::LOOP.with_volume(audio_settings.music_volume()),
        Music,
    )
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>, audio_settings: &AudioSettings) -> impl Bundle {
    (
        AudioPlayer(handle),
        PlaybackSettings::DESPAWN.with_volume(audio_settings.sfx_volume()),
        SoundEffect,
    )
}
