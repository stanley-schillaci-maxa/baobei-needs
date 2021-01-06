//! Manage cooldown timers.

use bevy::prelude::*;

/// TODO: Use Bevy cooldown => <https://github.com/bevyengine/bevy/issues/1127>
#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Cooldown {
    /// Remaining time until available.
    remaining: f32,
    /// The duration of the cooldown.
    duration: f32,
    /// Whether or not the cooldown is available, ie. not in progress.
    available: bool,
}

impl Cooldown {
    /// Creates a cooldown that is available at start.
    pub const fn from_seconds(seconds: f32) -> Self {
        Self {
            duration: seconds,
            available: true,
            remaining: 0.0,
        }
    }

    /// Starts the cooldown, making it unavailable for the given duration.
    #[inline]
    pub fn start(&mut self) {
        self.available = false;
        self.remaining = self.duration;
    }

    /// Returns true if the cooldown is available.
    #[inline]
    pub const fn available(&self) -> bool {
        self.available
    }

    /// Advances the cooldown by `delta` seconds.
    pub fn tick(&mut self, delta: f32) -> &Self {
        if self.available {
            return self;
        }

        self.remaining -= delta;

        if self.remaining <= 0.0 {
            self.remaining = 0.0;
            self.available = true;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Cooldown;

    #[test]
    fn test_cooldown() {
        let mut cd = Cooldown::from_seconds(2.0);
        assert_eq!(cd.available(), true);

        // Start the cooldown
        cd.start();
        assert_eq!(cd.available(), false);
        assert_eq!(cd.tick(0.75).available(), false);
        assert_eq!(cd.tick(1.5).available(), true);
        assert_eq!(cd.tick(10.0).available(), true);

        // Re-start the cooldown
        cd.start();
        assert_eq!(cd.available(), false);
        assert_eq!(cd.tick(0.75).available(), false);
        assert_eq!(cd.tick(0.75).available(), false);
        assert_eq!(cd.tick(0.75).available(), true);
    }
}
