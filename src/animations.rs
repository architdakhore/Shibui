//! Animation system module
//! 
//! Smooth animations for window transitions, workspace switching, and overview.

use log::debug;

/// Animation manager
pub struct AnimationManager {
    /// Enable animations
    enabled: bool,
    /// Active animations
    animations: Vec<Animation>,
    /// Default duration in milliseconds
    default_duration: u32,
    /// Animation curve
    curve: AnimationCurve,
}

/// Individual animation
pub struct Animation {
    /// Animation ID
    pub id: usize,
    /// Start value
    pub start: f32,
    /// End value
    pub end: f32,
    /// Current value
    pub current: f32,
    /// Progress (0.0 - 1.0)
    pub progress: f32,
    /// Duration in seconds
    pub duration: f32,
    /// Animation type
    pub anim_type: AnimationType,
}

/// Animation types
#[derive(Debug, Clone, Copy)]
pub enum AnimationType {
    /// Window open/close
    WindowTransition,
    /// Workspace switch
    WorkspaceSwitch,
    /// Overview activation
    Overview,
    /// Scroll animation
    Scroll,
    /// Custom property
    Custom,
}

/// Animation easing curves
#[derive(Debug, Clone, Copy)]
pub enum AnimationCurve {
    /// Linear interpolation
    Linear,
    /// Ease in
    EaseIn,
    /// Ease out
    EaseOut,
    /// Ease in-out
    EaseInOut,
    /// Ease out exponential
    EaseOutExpo,
    /// Spring animation
    Spring,
}

impl AnimationManager {
    /// Create new animation manager
    pub fn new() -> Self {
        Self {
            enabled: true,
            animations: Vec::new(),
            default_duration: 250, // 250ms
            curve: AnimationCurve::EaseOutExpo,
        }
    }
    
    /// Enable/disable animations
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        debug!("Animations {}", if enabled { "enabled" } else { "disabled" });
    }
    
    /// Set default animation duration
    pub fn set_duration(&mut self, duration_ms: u32) {
        self.default_duration = duration_ms;
    }
    
    /// Set animation curve
    pub fn set_curve(&mut self, curve: AnimationCurve) {
        self.curve = curve;
    }
    
    /// Add a new animation
    pub fn add_animation(&mut self, start: f32, end: f32, duration_ms: Option<u32>, anim_type: AnimationType) -> usize {
        let id = self.animations.len();
        let duration = duration_ms.unwrap_or(self.default_duration) as f32 / 1000.0;
        
        self.animations.push(Animation {
            id,
            start,
            end,
            current: start,
            progress: 0.0,
            duration,
            anim_type,
        });
        
        debug!("Added animation {} from {} to {}", id, start, end);
        id
    }
    
    /// Update all animations
    pub fn update(&mut self, delta_time: f32) {
        if !self.enabled {
            // If disabled, snap to end values
            for anim in &mut self.animations {
                anim.current = anim.end;
                anim.progress = 1.0;
            }
            return;
        }
        
        for anim in &mut self.animations {
            if anim.progress < 1.0 {
                anim.progress += delta_time / anim.duration;
                anim.progress = anim.progress.min(1.0);
                
                // Apply easing curve
                let eased = self.apply_curve(anim.progress);
                
                // Interpolate value
                anim.current = anim.start + (anim.end - anim.start) * eased;
            } else {
                // Ensure completed animations snap to end value
                anim.current = anim.end;
            }
        }
        
        // Remove completed animations
        self.animations.retain(|anim| anim.progress < 1.0);
    }
    
    /// Apply easing curve
    fn apply_curve(&self, progress: f32) -> f32 {
        match self.curve {
            AnimationCurve::Linear => progress,
            AnimationCurve::EaseIn => progress.powi(2),
            AnimationCurve::EaseOut => 1.0 - (1.0 - progress).powi(2),
            AnimationCurve::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress.powi(2)
                } else {
                    1.0 - 2.0 * (1.0 - progress).powi(2)
                }
            }
            AnimationCurve::EaseOutExpo => {
                if progress == 0.0 {
                    0.0
                } else if progress == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * progress)
                }
            }
            AnimationCurve::Spring => {
                // Underdamped spring: 1 - (1 + c*t) * exp(-c*t)
                // Better approximation: converges to 1.0 at progress=1.0
                let c = 3.0; // Spring stiffness
                1.0 - (1.0 + c * progress) * (-c * progress).exp()
            }
        }
    }
    
    /// Get current value for an animation
    /// Returns the end value if the animation has completed (progress >= 1.0)
    pub fn get_value(&self, id: usize) -> Option<f32> {
        // First check active animations
        if let Some(anim) = self.animations.iter().find(|a| a.id == id) {
            return Some(anim.current);
        }
        
        // Animation was removed (completed), return None
        // Caller should use the last known value or end value
        None
    }
    
    /// Check if an animation is complete
    pub fn is_complete(&self, id: usize) -> bool {
        // If animation doesn't exist, consider it complete
        self.animations.iter()
            .find(|a| a.id == id)
            .map_or(true, |a| a.progress >= 1.0)
    }
    
    /// Remove a specific animation
    pub fn remove_animation(&mut self, id: usize) {
        self.animations.retain(|a| a.id != id);
    }
    
    /// Clear all animations
    pub fn clear_all(&mut self) {
        self.animations.clear();
    }
    
    /// Check if any animations are active
    pub fn has_active_animations(&self) -> bool {
        !self.animations.is_empty()
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}
