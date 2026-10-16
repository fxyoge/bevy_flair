use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

/// Externally samples all CSS keyframe animations on this entity.
///
/// Transitions still use their normal clock. Keyframes use normalized effect
/// progress, independent of their duration, delay, iterations, direction, fill
/// mode and play state; easing remains part of the resolved keyframe curve.
/// External sampling does not emit clock-driven start/finish events. Removing
/// this component resumes the animation clock from where it was suspended.
///
/// Update before [`crate::StyleSystems::SetAnimationValues`] (and before
/// [`crate::StyleSystems::TickAnimations`] when changing clock ownership).
#[derive(Component, Reflect, Clone, Copy, Debug, PartialEq)]
#[reflect(Component, Clone, Debug, PartialEq)]
pub struct AnimationProgress(f32);

impl AnimationProgress {
    /// Returns `None` for non-finite progress. Finite values clamp to 0–1.
    pub fn new(progress: f32) -> Option<Self> {
        progress.is_finite().then(|| Self(progress.clamp(0.0, 1.0)))
    }

    /// Normalized effect progress.
    pub fn get(self) -> f32 {
        self.0
    }
}
