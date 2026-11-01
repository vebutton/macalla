use crate::{
    HealthCheckOptions, ProviderAuthStatus, ProviderHealth, ProviderHealthStatus, ProviderKind,
};

pub fn health(options: &HealthCheckOptions) -> ProviderHealth {
    let health = anlg_opencode::health_check_with_options(&anlg_opencode::OpencodeOptions {
        opencode_path_override: options.opencode_path_override.clone(),
        ..Default::default()
    });

    ProviderHealth {
        provider: ProviderKind::Opencode,
        binary_path: health.binary_path,
        installed: health.installed,
        version: health.version,
        status: health.status.into(),
        auth_status: health.auth_status.into(),
        message: health.message,
    }
}

impl From<anlg_opencode::HealthStatus> for ProviderHealthStatus {
    fn from(value: anlg_opencode::HealthStatus) -> Self {
        match value {
            anlg_opencode::HealthStatus::Ready => Self::Ready,
            anlg_opencode::HealthStatus::Warning => Self::Warning,
            anlg_opencode::HealthStatus::Error => Self::Error,
        }
    }
}

impl From<anlg_opencode::HealthAuthStatus> for ProviderAuthStatus {
    fn from(value: anlg_opencode::HealthAuthStatus) -> Self {
        match value {
            anlg_opencode::HealthAuthStatus::Authenticated => Self::Authenticated,
            anlg_opencode::HealthAuthStatus::Unauthenticated => Self::Unauthenticated,
            anlg_opencode::HealthAuthStatus::Unknown => Self::Unknown,
        }
    }
}
