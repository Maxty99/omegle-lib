use strum::Display;

/// Enum for working with omegle language codes in Rust
#[derive(Display, Debug, Clone, Copy)]
pub enum LangCode {
    #[strum(serialize = "en")]
    English,
    #[strum(serialize = "fr")]
    French,
    #[strum(serialize = "es")]
    Spanish,
}

#[cfg(test)]
mod tests {
    use crate::types::lang::LangCode;

    #[test]

    fn serializes_properly() {
        assert_eq!(LangCode::English.to_string(), "en");
        assert_eq!(LangCode::French.to_string(), "fr");
        assert_eq!(LangCode::Spanish.to_string(), "es");
    }
}
