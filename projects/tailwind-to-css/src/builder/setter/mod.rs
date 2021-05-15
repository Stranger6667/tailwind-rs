use super::*;

impl TailwindBuilder {
    /// Config for preflight progress
    pub fn preflight(&mut self) -> &mut PreflightSystem {
        &mut self.preflight
    }
    /// Add custom preflight information
    pub fn preflight_addition(&mut self, custom: impl Into<String>) {
        self.preflight.custom = custom.into()
    }
}

impl Default for TailwindBuilder {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            screens: BreakPointSystem::builtin(),
            colors: PaletteSystem::builtin(),
            fonts: FontSystem::builtin(),
            preflight: PreflightSystem::default(),
        }
    }
}