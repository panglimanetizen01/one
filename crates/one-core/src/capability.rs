#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    Filesystem,
    Process,
    Network,
    Native,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CapabilitySet {
    capabilities: Vec<Capability>,
}

impl CapabilitySet {
    pub fn none() -> Self { Self::default() }
    pub fn allow(mut self, capability: Capability) -> Self { if !self.capabilities.contains(&capability) { self.capabilities.push(capability); } self }
    pub fn permits(&self, capability: Capability) -> bool { self.capabilities.contains(&capability) }
}
