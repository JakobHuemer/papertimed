use std::collections::HashMap;

use minijinja::Environment;
use minijinja::Error;
use thiserror::Error;

use crate::{
    adapter::{AdapterError, WallpaperAdapter},
    daemon::WallpaperState,
};

#[derive(Debug, Error)]
pub enum CustomAdpaterError {
    #[error("command is not a valid template: {0}")]
    TemplateError(Error),

    #[error("command is empty")]
    CommandIsEmpty,
}

#[derive(Default, Debug, Clone)]
pub struct CustomAdapter {
    pub command: String,
}

impl WallpaperAdapter for CustomAdapter {
    type Input = WallpaperState;

    async fn update(&mut self, input: Self::Input) -> Result<(), super::AdapterError> {
        let env = Environment::new();

        for (monitor, wallpaper) in &input.wallpapers {
            let mut context: HashMap<&str, minijinja::Value> = HashMap::new();
            context.insert("monitor", minijinja::Value::from(monitor.clone()));
            context.insert("image", minijinja::Value::from(wallpaper.filename.clone()));

            let rendered = env
                .render_str(&self.command, &context)
                .map_err(|e| CustomAdpaterError::TemplateError(e))?;

            let parts: Vec<&str> = rendered.split_whitespace().collect();
            if parts.is_empty() {
                return Err(AdapterError::Custom(CustomAdpaterError::CommandIsEmpty));
            }

            let output = tokio::process::Command::new(parts[0])
                .args(&parts[1..])
                .output()
                .await
                .map_err(|_| AdapterError::UtilityNotInstalled(parts[0].to_string()))?;

            if !output.status.success() {
                let stdout_str = String::from_utf8_lossy(&output.stdout);
                let stderr_str = String::from_utf8_lossy(&output.stderr);
                let error_out = format!("\nstdout: {}\nstderr: {}", stdout_str, stderr_str);
                return Err(AdapterError::UtilityFailedWith {
                    status_code: output.status.code().unwrap_or_default(),
                    error_out,
                    utility: parts[0].to_string(),
                });
            }
        }

        Ok(())
    }
}
