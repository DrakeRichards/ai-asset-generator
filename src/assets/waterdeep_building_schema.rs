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
#[doc = "A building in the city of Waterdeep."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://github.com/DrakeRichards/rpg-generation-assets/buildings/waterdeep-building.schema.json\","]
#[doc = "  \"title\": \"Waterdeep Building\","]
#[doc = "  \"description\": \"A building in the city of Waterdeep.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cityWard\": {"]
#[doc = "      \"description\": \"The city ward where the building is located.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A description of the building.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hook\": {"]
#[doc = "      \"description\": \"A hook to help game masters introduce the building into the story, such as a rumor about the building or a problem that the occupants are facing.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the building.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"occupants\": {"]
#[doc = "      \"description\": \"Who lives or works in the building? Who can an adventuring party expect to find inside?\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of building, such as a tavern, shop, or residence.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WaterdeepBuilding {
    #[doc = "The city ward where the building is located."]
    #[serde(rename = "cityWard", default, skip_serializing_if = "Option::is_none")]
    pub city_ward: Option<String>,
    #[doc = "A description of the building."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A hook to help game masters introduce the building into the story, such as a rumor about the building or a problem that the occupants are facing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    #[doc = "The name of the building."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Who lives or works in the building? Who can an adventuring party expect to find inside?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occupants: Option<String>,
    #[doc = "The type of building, such as a tavern, shop, or residence."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&WaterdeepBuilding> for WaterdeepBuilding {
    fn from(value: &WaterdeepBuilding) -> Self {
        value.clone()
    }
}
impl WaterdeepBuilding {
    pub fn builder() -> builder::WaterdeepBuilding {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct WaterdeepBuilding {
        city_ward: Result<Option<String>, String>,
        description: Result<Option<String>, String>,
        hook: Result<Option<String>, String>,
        name: Result<Option<String>, String>,
        occupants: Result<Option<String>, String>,
        type_: Result<Option<String>, String>,
    }
    impl Default for WaterdeepBuilding {
        fn default() -> Self {
            Self {
                city_ward: Ok(Default::default()),
                description: Ok(Default::default()),
                hook: Ok(Default::default()),
                name: Ok(Default::default()),
                occupants: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl WaterdeepBuilding {
        pub fn city_ward<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.city_ward = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for city_ward: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn hook<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.hook = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hook: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn occupants<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.occupants = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for occupants: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<WaterdeepBuilding> for super::WaterdeepBuilding {
        type Error = super::error::ConversionError;
        fn try_from(value: WaterdeepBuilding) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                city_ward: value.city_ward?,
                description: value.description?,
                hook: value.hook?,
                name: value.name?,
                occupants: value.occupants?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::WaterdeepBuilding> for WaterdeepBuilding {
        fn from(value: super::WaterdeepBuilding) -> Self {
            Self {
                city_ward: Ok(value.city_ward),
                description: Ok(value.description),
                hook: Ok(value.hook),
                name: Ok(value.name),
                occupants: Ok(value.occupants),
                type_: Ok(value.type_),
            }
        }
    }
}
