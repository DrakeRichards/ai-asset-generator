#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "A character in a role-playing game."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://github.com/DrakeRichards/rpg-generation-assets/characters/character.schema.json\","]
#[doc = "  \"title\": \"Character\","]
#[doc = "  \"description\": \"A character in a role-playing game.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"behaviorQuirk\": {"]
#[doc = "      \"description\": \"A quirk or behavior that the character exhibits. Useful to make NPCs stand out from each other.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"dislikes\": {"]
#[doc = "      \"description\": \"Things that the character dislikes.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"gender\": {"]
#[doc = "      \"description\": \"The character's gender.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hook\": {"]
#[doc = "      \"description\": \"A hook to help game masters introduce the character into the story, such as a small problem the character is currently facing.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"likes\": {"]
#[doc = "      \"description\": \"Things that the character likes.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the character.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"occupation\": {"]
#[doc = "      \"description\": \"The character's occupation.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"overview\": {"]
#[doc = "      \"description\": \"An overview of the character's personality and background.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"personalGoals\": {"]
#[doc = "      \"description\": \"What does this character want to achieve?\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"physicalDescription\": {"]
#[doc = "      \"description\": \"A description of the character's physical appearance.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"race\": {"]
#[doc = "      \"description\": \"The character's race.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"voiceDescription\": {"]
#[doc = "      \"description\": \"A brief description of the character's voice, to help game masters role-play the character.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Character {
    #[doc = "A quirk or behavior that the character exhibits. Useful to make NPCs stand out from each other."]
    #[serde(rename = "behaviorQuirk", default)]
    pub behavior_quirk: String,
    #[doc = "Things that the character dislikes."]
    #[serde(default)]
    pub dislikes: String,
    #[doc = "The character's gender."]
    #[serde(default)]
    pub gender: String,
    #[doc = "A hook to help game masters introduce the character into the story, such as a small problem the character is currently facing."]
    #[serde(default)]
    pub hook: String,
    #[doc = "Things that the character likes."]
    #[serde(default)]
    pub likes: String,
    #[doc = "The name of the character."]
    #[serde(default)]
    pub name: String,
    #[doc = "The character's occupation."]
    #[serde(default)]
    pub occupation: String,
    #[doc = "An overview of the character's personality and background."]
    #[serde(default)]
    pub overview: String,
    #[doc = "What does this character want to achieve?"]
    #[serde(rename = "personalGoals", default)]
    pub personal_goals: String,
    #[doc = "A description of the character's physical appearance."]
    #[serde(rename = "physicalDescription", default)]
    pub physical_description: String,
    #[doc = "The character's race."]
    #[serde(default)]
    pub race: String,
    #[doc = "A brief description of the character's voice, to help game masters role-play the character."]
    #[serde(rename = "voiceDescription", default)]
    pub voice_description: String,
}
impl From<&Character> for Character {
    fn from(value: &Character) -> Self {
        value.clone()
    }
}
impl Character {
    pub fn builder() -> builder::Character {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Character {
        behavior_quirk: Result<String, String>,
        dislikes: Result<String, String>,
        gender: Result<String, String>,
        hook: Result<String, String>,
        likes: Result<String, String>,
        name: Result<String, String>,
        occupation: Result<String, String>,
        overview: Result<String, String>,
        personal_goals: Result<String, String>,
        physical_description: Result<String, String>,
        race: Result<String, String>,
        voice_description: Result<String, String>,
    }
    impl Default for Character {
        fn default() -> Self {
            Self {
                behavior_quirk: Ok(Default::default()),
                dislikes: Ok(Default::default()),
                gender: Ok(Default::default()),
                hook: Ok(Default::default()),
                likes: Ok(Default::default()),
                name: Ok(Default::default()),
                occupation: Ok(Default::default()),
                overview: Ok(Default::default()),
                personal_goals: Ok(Default::default()),
                physical_description: Ok(Default::default()),
                race: Ok(Default::default()),
                voice_description: Ok(Default::default()),
            }
        }
    }
    impl Character {
        pub fn behavior_quirk<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.behavior_quirk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for behavior_quirk: {}", e));
            self
        }
        pub fn dislikes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.dislikes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dislikes: {}", e));
            self
        }
        pub fn gender<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.gender = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gender: {}", e));
            self
        }
        pub fn hook<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.hook = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hook: {}", e));
            self
        }
        pub fn likes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.likes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for likes: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn occupation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.occupation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for occupation: {}", e));
            self
        }
        pub fn overview<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.overview = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for overview: {}", e));
            self
        }
        pub fn personal_goals<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.personal_goals = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for personal_goals: {}", e));
            self
        }
        pub fn physical_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.physical_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for physical_description: {}",
                    e
                )
            });
            self
        }
        pub fn race<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.race = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for race: {}", e));
            self
        }
        pub fn voice_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.voice_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for voice_description: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<Character> for super::Character {
        type Error = super::error::ConversionError;
        fn try_from(value: Character) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                behavior_quirk: value.behavior_quirk?,
                dislikes: value.dislikes?,
                gender: value.gender?,
                hook: value.hook?,
                likes: value.likes?,
                name: value.name?,
                occupation: value.occupation?,
                overview: value.overview?,
                personal_goals: value.personal_goals?,
                physical_description: value.physical_description?,
                race: value.race?,
                voice_description: value.voice_description?,
            })
        }
    }
    impl From<super::Character> for Character {
        fn from(value: super::Character) -> Self {
            Self {
                behavior_quirk: Ok(value.behavior_quirk),
                dislikes: Ok(value.dislikes),
                gender: Ok(value.gender),
                hook: Ok(value.hook),
                likes: Ok(value.likes),
                name: Ok(value.name),
                occupation: Ok(value.occupation),
                overview: Ok(value.overview),
                personal_goals: Ok(value.personal_goals),
                physical_description: Ok(value.physical_description),
                race: Ok(value.race),
                voice_description: Ok(value.voice_description),
            }
        }
    }
}
