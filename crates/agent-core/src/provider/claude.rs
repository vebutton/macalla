use crate::{
    HealthCheckOptions, ProviderAuthStatus, ProviderHealth, ProviderHealthStatus, ProviderKind,
};

pub fn health(options: &HealthCheckOptions) -> ProviderHealth {
    let health = anlg_claude::health_check_with_options(&anlg_claude::ClaudeOptions {
        claude_path_override: options.claude_path_override.clone(),
        ..Default::default()
    });

    ProviderHealth {
        provider: ProviderKind::Claude,
        binary_path: health.binary_path,
        installed: health.installed,
        version: health.version,
        status: health.status.into(),
        auth_status: health.auth_status.into(),
        message: health.message,
    }
}

impl From<anlg_claude::HealthStatus> for ProviderHealthStatus {
    fn from(value: anlg_claude::HealthStatus) -> Self {
        match value {
            anlg_claude::HealthStatus::Ready => Self::Ready,
            anlg_claude::HealthStatus::Warning => Self::Warning,
            anlg_claude::HealthStatus::Error => Self::Error,
        }
    }
}

impl From<anlg_claude::HealthAuthStatus> for ProviderAuthStatus {
    fn from(value: anlg_claude::HealthAuthStatus) -> Self {
        match value {
            anlg_claude::HealthAuthStatus::Authenticated => Self::Authenticated,
            anlg_claude::HealthAuthStatus::Unauthenticated => Self::Unauthenticated,
            anlg_claude::HealthAuthStatus::Unknown => Self::Unknown,
        }
    }
}
