#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorProfile {
    pub theme: String,
    pub enable_lsp: bool,
    pub enable_treesitter: bool,
}

impl EditorProfile {
    pub fn sovereign_default() -> Self {
        Self {
            theme: "sigmaos-monokai".into(),
            enable_lsp: true,
            enable_treesitter: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_editor_profile_aaa() {
        // Arrange & Act
        let profile = EditorProfile::sovereign_default();
        // Assert
        assert!(profile.enable_lsp);
        assert!(profile.enable_treesitter);
        assert_eq!(profile.theme, "sigmaos-monokai");
    }
}
