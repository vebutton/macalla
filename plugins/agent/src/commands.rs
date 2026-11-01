use crate::AgentPluginExt;

#[tauri::command]
#[specta::specta]
pub fn health_check<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<anlg_agent_core::HealthCheckResponse, String> {
    Ok(app.agent().health_check())
}
