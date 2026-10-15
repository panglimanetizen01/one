#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    Filesystem,
    Process,
    Network,
    NetworkConnect,
    NetworkListen,
    NetworkDiscover,
    NetworkAdmin,
    Native,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CapabilitySet {
    capabilities: Vec<Capability>,
}

impl CapabilitySet {
    pub fn none() -> Self { Self::default() }
    pub fn allow(mut self, capability: Capability) -> Self { if !self.capabilities.contains(&capability) { self.capabilities.push(capability); } self }
    pub fn permits(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
            || (capability == Capability::NetworkConnect && self.capabilities.contains(&Capability::Network))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_connect_requires_network_connect() {
        assert!(CapabilitySet::none().allow(Capability::NetworkConnect).permits(Capability::NetworkConnect));
    }

    #[test]
    fn network_discovery_requires_network_discover() {
        let capabilities = CapabilitySet::none().allow(Capability::NetworkConnect);
        assert!(!capabilities.permits(Capability::NetworkDiscover));
    }

    #[test]
    fn network_admin_is_not_implied_by_network_connect() {
        let capabilities = CapabilitySet::none().allow(Capability::NetworkConnect);
        assert!(!capabilities.permits(Capability::NetworkAdmin));
    }

    #[test]
    fn coarse_network_capability_preserves_existing_connect_compatibility() {
        let capabilities = CapabilitySet::none().allow(Capability::Network);
        assert!(capabilities.permits(Capability::NetworkConnect));
        assert!(!capabilities.permits(Capability::NetworkAdmin));
    }
}
