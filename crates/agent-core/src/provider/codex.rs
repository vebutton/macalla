use crate::{
    HealthCheckOptions, ProviderAuthStatus, ProviderHealth, ProviderHealthStatus, ProviderKind,
};

pub fn health(options: &HealthCheckOptions) -> ProviderHealth {
    let health = anlg_codex::health_check_with_options(&anlg_codex::CodexOptions {
        codex_path_override: options.codex_path_override.clone(),
        ..Default::default()
    });

    ProviderHealth {
        provider: ProviderKind::Codex,
        binary_path: health.binary_path,
        installed: health.installed,
        version: health.version,
        status: health.status.into(),
        auth_status: health.auth_status.into(),
        message: health.message,
    }
}

impl From<anlg_codex::HealthStatus> for ProviderHealthStatus {
    fn from(value: anlg_codex::HealthStatus) -> Self {
        match value {
            anlg_codex::HealthStatus::Ready => Self::Ready,
            anlg_codex::HealthStatus::Warning => Self::Warning,
            anlg_codex::HealthStatus::Error => Self::Error,
        }
    }
}

impl From<anlg_codex::HealthAuthStatus> for ProviderAuthStatus {
    fn from(value: anlg_codex::HealthAuthStatus) -> Self {
        match value {
            anlg_codex::HealthAuthStatus::Authenticated => Self::Authenticated,
            anlg_codex::HealthAuthStatus::Unauthenticated => Self::Unauthenticated,
            anlg_codex::HealthAuthStatus::Unknown => Self::Unknown,
        }
    }
}
