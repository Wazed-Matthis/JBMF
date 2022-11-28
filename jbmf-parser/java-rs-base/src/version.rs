#[derive(Debug, Clone, PartialOrd, PartialEq, Hash, Ord, Eq)]
pub struct JavaVersion {
    pub major: u16,
    pub minor: u16,
}

impl JavaVersion {
    pub fn supports(&self, major: u16, minor: u16) -> bool {
        self.major > major || (self.major == major && self.minor >= minor)
    }
    pub fn supports_version(&self, version: &JavaVersion) -> bool {
        self.supports(version.major, version.minor)
    }
}
