pub mod errors;

// use idna::{
//     Errors,
//     uts46::{AsciiDenyList, Hyphens},
// };

// TODO: Add idna crate for proper Unicode normalization
pub fn normalize(name: &str) -> Result<String, errors::NormalizationError> {
    let trimmed_and_lowered = name.trim().to_lowercase();

    // let temp =
    //     idna::uts46::Uts46::to_user_interface(name.as_bytes(), AsciiDenyList::URL, Hyphens::Allow);

    let config = idna::domain_to_unicode(&trimmed_and_lowered);

    let (success_string, error_result) = config;

    if success_string.is_empty() {
        return Err(errors::NormalizationError::Error(
            "Empty string normalized".to_string(),
        ));
    }

    match error_result {
        Ok(_) => Ok(success_string.to_lowercase()),
        Err(error) => Err(errors::NormalizationError::Error(error.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let result = normalize("vitalik.eth");
        let final_result = result.expect("not valid");
        assert_eq!(final_result, "vitalik.eth");
    }

    #[test]
    fn test_valid_2() {
        let result = normalize("VITALIK.ETH");
        let final_result = result.expect("not valid");
        assert_eq!(final_result, "vitalik.eth");
    }

    #[test]
    fn test_valid_3() {
        let result = normalize("  vitalik.eth  ");
        let final_result = result.expect("not valid");
        assert_eq!(final_result, "vitalik.eth");
    }

    #[test]
    fn test_valid_4() {
        let result = normalize("");
        assert!(matches!(result, Err(errors::NormalizationError::Error(_))));
    }

    #[test]
    fn test_valid_5() {
        let result = normalize("münchen.eth");
        let final_result = result.expect("not valid");
        assert_eq!(final_result, "münchen.eth");
    }
}
