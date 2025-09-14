use crate::types::{Level, Profile};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ProfileError {
    InvalidProfileLevelCombination(Profile, Level),
    UnsupportedProfile(Profile),
    UnsupportedLevel(Level),
    InvalidConfiguration(String),
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProfileError::InvalidProfileLevelCombination(profile, level) => {
                write!(
                    f,
                    "Invalid profile-level combination: {:?} with {:?}",
                    profile, level
                )
            }
            ProfileError::UnsupportedProfile(profile) => {
                write!(f, "Unsupported profile: {:?}", profile)
            }
            ProfileError::UnsupportedLevel(level) => {
                write!(f, "Unsupported level: {:?}", level)
            }
            ProfileError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
        }
    }
}

impl Error for ProfileError {}

pub fn validate_profile_level_combination(
    profile: Profile,
    level: Level,
) -> Result<(), ProfileError> {
    match (profile, level) {
        // Light Profile constraints (ISO/IEC 21122-1:2024)
        (Profile::Light, Level::Level1) => Ok(()),
        (Profile::Light, Level::Level2) => Ok(()),
        (Profile::Light, level) => {
            Err(ProfileError::InvalidProfileLevelCombination(profile, level))
        }

        // Main Profile constraints
        (Profile::Main, Level::Level1) => Ok(()),
        (Profile::Main, Level::Level2) => Ok(()),
        (Profile::Main, Level::Level3) => Ok(()),
        (Profile::Main, Level::Level4) => Ok(()),
        (Profile::Main, level) => Err(ProfileError::InvalidProfileLevelCombination(profile, level)),

        // High Profile constraints
        (Profile::High, Level::Level1) => Ok(()),
        (Profile::High, Level::Level2) => Ok(()),
        (Profile::High, Level::Level3) => Ok(()),
        (Profile::High, Level::Level4) => Ok(()),
        (Profile::High, Level::Level5) => Ok(()),
    }
}

pub fn get_max_bitrate_mbps(profile: Profile, level: Level) -> Result<u32, ProfileError> {
    validate_profile_level_combination(profile, level)?;

    match (profile, level) {
        // Light Profile bitrates
        (Profile::Light, Level::Level1) => Ok(100),
        (Profile::Light, Level::Level2) => Ok(400),

        // Main Profile bitrates
        (Profile::Main, Level::Level1) => Ok(200),
        (Profile::Main, Level::Level2) => Ok(800),
        (Profile::Main, Level::Level3) => Ok(1600),
        (Profile::Main, Level::Level4) => Ok(3200),

        // High Profile bitrates
        (Profile::High, Level::Level1) => Ok(400),
        (Profile::High, Level::Level2) => Ok(1600),
        (Profile::High, Level::Level3) => Ok(3200),
        (Profile::High, Level::Level4) => Ok(6400),
        (Profile::High, Level::Level5) => Ok(12800),

        // Invalid combinations handled by validate_profile_level_combination
        _ => unreachable!(),
    }
}

pub fn get_max_resolution(profile: Profile, level: Level) -> Result<(u32, u32), ProfileError> {
    validate_profile_level_combination(profile, level)?;

    match (profile, level) {
        // Light Profile resolutions
        (Profile::Light, Level::Level1) => Ok((1920, 1080)),
        (Profile::Light, Level::Level2) => Ok((3840, 2160)),

        // Main Profile resolutions
        (Profile::Main, Level::Level1) => Ok((1920, 1080)),
        (Profile::Main, Level::Level2) => Ok((3840, 2160)),
        (Profile::Main, Level::Level3) => Ok((7680, 4320)),
        (Profile::Main, Level::Level4) => Ok((15360, 8640)),

        // High Profile resolutions
        (Profile::High, Level::Level1) => Ok((1920, 1080)),
        (Profile::High, Level::Level2) => Ok((3840, 2160)),
        (Profile::High, Level::Level3) => Ok((7680, 4320)),
        (Profile::High, Level::Level4) => Ok((15360, 8640)),
        (Profile::High, Level::Level5) => Ok((30720, 17280)),

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_profile_level_combinations() {
        assert!(validate_profile_level_combination(Profile::Light, Level::Level1).is_ok());
        assert!(validate_profile_level_combination(Profile::Light, Level::Level2).is_ok());
        assert!(validate_profile_level_combination(Profile::Main, Level::Level1).is_ok());
        assert!(validate_profile_level_combination(Profile::Main, Level::Level4).is_ok());
        assert!(validate_profile_level_combination(Profile::High, Level::Level5).is_ok());
    }

    #[test]
    fn test_invalid_profile_level_combinations() {
        assert!(validate_profile_level_combination(Profile::Light, Level::Level3).is_err());
        assert!(validate_profile_level_combination(Profile::Light, Level::Level4).is_err());
        assert!(validate_profile_level_combination(Profile::Light, Level::Level5).is_err());
        assert!(validate_profile_level_combination(Profile::Main, Level::Level5).is_err());
    }

    #[test]
    fn test_bitrate_limits() {
        assert_eq!(
            get_max_bitrate_mbps(Profile::Light, Level::Level1).unwrap(),
            100
        );
        assert_eq!(
            get_max_bitrate_mbps(Profile::Main, Level::Level4).unwrap(),
            3200
        );
        assert_eq!(
            get_max_bitrate_mbps(Profile::High, Level::Level5).unwrap(),
            12800
        );
    }

    #[test]
    fn test_resolution_limits() {
        assert_eq!(
            get_max_resolution(Profile::Light, Level::Level1).unwrap(),
            (1920, 1080)
        );
        assert_eq!(
            get_max_resolution(Profile::Main, Level::Level3).unwrap(),
            (7680, 4320)
        );
        assert_eq!(
            get_max_resolution(Profile::High, Level::Level5).unwrap(),
            (30720, 17280)
        );
    }
}
