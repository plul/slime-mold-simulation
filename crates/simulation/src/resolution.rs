/// Represents a screen resolution with width and height.
///
/// # Examples
///
/// ```
/// let resolution = "1920x1080";
/// assert!(Resolution.parse(resolution).is_ok());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}
impl std::str::FromStr for Resolution {
    type Err = ResolutionError;

    /// Parses a resolution string in the format "WIDTHxHEIGHT".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const DELIMITER: char = 'x';

        let mut parts = s.split(DELIMITER);

        match (parts.next(), parts.next(), parts.next()) {
            (Some(w), Some(h), None) => {
                let width = w.parse::<u32>().map_err(|_| ResolutionError::invalid_format(s))?;
                let height = h.parse::<u32>().map_err(|_| ResolutionError::invalid_format(s))?;
                Ok(Resolution { width, height })
            }
            _ => Err(ResolutionError::invalid_format(s)),
        }
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ResolutionError {
    #[error("Failed to parse resolution: {}. Invalid format.", _0)]
    InvalidFormat(String),
}
impl ResolutionError {
    fn invalid_format(s: &str) -> Self {
        Self::InvalidFormat(s.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_resolution() {
        let resolution = "1920x1080";
        let result = resolution.parse::<Resolution>();
        assert!(matches!(result, Ok(Resolution { width: 1920, height: 1080 })));
    }

    #[test]
    fn test_invalid_resolution() {
        let resolution = "1920xabc";
        let result = resolution.parse::<Resolution>();
        assert!(matches!(result, Err(ResolutionError::InvalidFormat(_))));
    }
}
