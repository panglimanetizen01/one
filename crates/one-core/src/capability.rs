#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    Filesystem,
    FilesystemRead,
    FilesystemWrite,
    FilesystemDelete,
    Process,
    DatabaseQuery,
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
            || (matches!(capability, Capability::FilesystemRead | Capability::FilesystemWrite) && self.capabilities.contains(&Capability::Filesystem))
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

    #[test]
    fn filesystem_read_and_write_capabilities_are_isolated() {
        let read = CapabilitySet::none().allow(Capability::FilesystemRead);
        let write = CapabilitySet::none().allow(Capability::FilesystemWrite);
        assert!(read.permits(Capability::FilesystemRead));
        assert!(!read.permits(Capability::FilesystemWrite));
        assert!(write.permits(Capability::FilesystemWrite));
        assert!(!write.permits(Capability::FilesystemRead));
    }

    #[test]
    fn coarse_filesystem_capability_preserves_read_write_compatibility() {
        let capabilities = CapabilitySet::none().allow(Capability::Filesystem);
        assert!(capabilities.permits(Capability::FilesystemRead));
        assert!(capabilities.permits(Capability::FilesystemWrite));
    }


    #[test]
    fn filesystem_delete_requires_an_explicit_grant() {
        let coarse = CapabilitySet::none().allow(Capability::Filesystem);
        let write = CapabilitySet::none().allow(Capability::FilesystemWrite);
        let delete = CapabilitySet::none().allow(Capability::FilesystemDelete);
        assert!(!coarse.permits(Capability::FilesystemDelete));
        assert!(!write.permits(Capability::FilesystemDelete));
        assert!(delete.permits(Capability::FilesystemDelete));
    }

    #[test]
    fn database_query_requires_explicit_capability() {
        assert!(!CapabilitySet::none().permits(Capability::DatabaseQuery));
        assert!(CapabilitySet::none().allow(Capability::DatabaseQuery).permits(Capability::DatabaseQuery));
    }
}
