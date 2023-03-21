use bevy::prelude::Resource;

use super::config::INITIAL_DROP_FRAMES;

#[derive(Resource)]
pub struct FrameCounter {
    pub elapsed: i64,
    pub target: i64,
}

impl FrameCounter {
    pub fn default() -> Self {
        return FrameCounter {
            elapsed: 0,
            target: INITIAL_DROP_FRAMES,
        };
    }

    pub fn tick(&mut self) {
        self.elapsed += 1;
    }

    pub fn set_target(&mut self, tar: i64) {
        self.target = tar;
    }

    pub fn reset(&mut self) {
        self.elapsed = 0;
    }

    pub fn is_finished(&mut self) -> bool {
        if self.target < 0 {
            return false;
        }

        return self.target <= self.elapsed;
    }

    pub fn force_finish(&mut self) {
        self.elapsed = self.target;
    }
}
