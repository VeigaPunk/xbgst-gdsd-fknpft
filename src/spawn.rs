use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpawnError {
    #[error("denied: {0}")]
    Denied(String),
    #[error("empty command")]
    Empty,
}

const DENY: &[&str] = ["claude", "anthropic", "xask", "sonnet", "opus"];

#[derive(Debug, Clone)]
pub struct SpawnSpec {
    pub role: String,
    pub argv: Vec<String>,
    pub node: Option<String>,
}

/// fnm multishells — only isolation that keeps model granularity clean.
pub fn plan(spec: &SpawnSpec) -> Result<String, SpawnError> {
    if spec.argv.is_empty() {
        return Err(SpawnError::Empty);
    }
    let joined = spec.argv.join(" ").to_ascii_lowercase();
    for d in DENY {
        if joined.contains(d) {
            return Err(SpawnError::Denied(d.to_string()));
        }
    }
    let node = spec.node.as_deref().unwrap_or("default");
    let cmd = spec.argv.join(" ");
    let fnm = format!(
        "fnm exec --using {node} -- bash -c 'export AGENT_ID=gx-{}-$$; export TMPDIR=/tmp/xbgst-gx-{}-$$; mkdir -p \"$TMPDIR\"; {cmd}'",
        spec.role, spec.role
    );
    Ok(format!("spawn_method: fnm-multishell\nrole: {}\nfnm: {fnm}\n", spec.role))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnm_plan_wraps_any_cli() {
        let spec = SpawnSpec {
            role: "labrat".into(),
            argv: vec!["sekhmet".into(), "run".into(), "--task".into(), "ping".into()],
            node: Some("default".into()),
        };
        let p = plan(&spec).unwrap();
        assert!(p.contains("fnm exec --using default -- bash -c"));
        assert!(p.contains("sekhmet"));
        assert!(!p.contains("xask"));
    }

    #[test]
    fn denies_claude() {
        let spec = SpawnSpec {
            role: "scout".into(),
            argv: vec!["claude".into(), "-p".into(), "hi".into()],
            node: None,
        };
        assert!(plan(&spec).is_err());
    }
}
