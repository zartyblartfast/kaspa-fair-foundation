//! Safe-by-default capability flags for the foundation core.

/// Describes which high-risk capabilities are exposed by a build/profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FoundationCapabilities {
    pub signing_api_exposed: bool,
    pub submit_api_exposed: bool,
    pub mainnet_enabled: bool,
    pub roulette_enabled: bool,
}

/// Default ENV-068 capabilities: offline metadata and fixture verification only.
pub const fn default_capabilities() -> FoundationCapabilities {
    FoundationCapabilities {
        signing_api_exposed: false,
        submit_api_exposed: false,
        mainnet_enabled: false,
        roulette_enabled: false,
    }
}

/// Human-readable safety boundary for ENV-068.
pub const OFFLINE_ONLY_BOUNDARY: &str =
    "offline-only: no live RPC, no signing, no submit/broadcast, no wallet/key access, no mainnet, no roulette/web app";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_safe() {
        assert_eq!(
            default_capabilities(),
            FoundationCapabilities {
                signing_api_exposed: false,
                submit_api_exposed: false,
                mainnet_enabled: false,
                roulette_enabled: false,
            }
        );
    }
}
