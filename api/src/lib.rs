pub use v1::{Field, FieldContent, GameServer, GameServerList};

pub mod v1 {
    use country_parser::Country;
    use serde::{
        de::{Deserializer, Error, Unexpected},
        ser::Serializer,
        Deserialize, Serialize,
    };
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct GameServerList {
        /// List of all servers registered to this serverbrowser
        pub servers: Vec<GameServer>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct GameServer {
        /// The name of the server.
        pub name: String,
        /// The address through which the server might be accessed on the open
        /// internet. This field may be an IPv4 address, IPv6 address,
        /// URL, must not contain a port
        pub address: String,
        /// Port of the gameserver address (usually `14004`)
        pub port: u16,
        /// The server description.
        pub description: String,
        /// The ISO 3166‑1 Alpha-2 code that the server is physically based in
        /// (note: this field is intended as an indication of factors
        /// like ping, not the language of the server). (e.g. "US")
        #[serde(deserialize_with = "deserialize_country")]
        #[serde(serialize_with = "serialize_country")]
        #[serde(default)]
        pub location: Option<Country>,
        /// The auth server that must be used to connect to this server.
        /// If you want to use the official auth server use `Some("https://auth.veloren.net")`
        pub auth_server: String,
        /// The version channel used by the server. `None` means not running a
        /// channel distributed by Airshipper. If in doubt, `"weekly"`
        /// is probably correct.
        pub channel: Option<String>,
        /// Whether the server is officially affiliated with the Veloren
        /// project.
        pub official: bool,
        /// Any extra attributes provided by the server.
        ///
        /// The key is a machine-readable ID. Frontends may choose to display
        /// these fields in a different way (for example, adding an
        /// icon) based on this machine-readable ID, if they recognise it. There
        /// is no specific list of valid IDs and recognition is based on
        /// convention. Some examples of IDs include:
        ///
        /// - `website`
        /// - `email`
        /// - `discord`
        /// - `mastodon`
        /// - `reddit`
        /// - `youtube`
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        pub extra: HashMap<String, Field>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct Field {
        /// A human-readable suggested name. Frontends are permitted to override
        /// this name for purposes such as localisation or aesthetic
        /// purposes.
        pub name: String,
        /// The content of the field.
        pub content: FieldContent,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    #[non_exhaustive]
    #[serde(rename_all = "lowercase")]
    pub enum FieldContent {
        /// This field's content should be interpreted as human-readable
        /// plaintext.
        Text(String),
        /// This field's content should be interpreted as a URL.
        Url(String),
        /// This field's content was of an unknown format. This cannot be
        /// serialized but only exists to guarantee forward compatibility
        #[serde(other)]
        #[serde(skip_serializing)]
        Unknown,
    }

    fn deserialize_country<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Country>, D::Error> {
        country_parser::parse(String::deserialize(de)?)
            .map(Some)
            .ok_or_else(|| {
                D::Error::invalid_value(
                    Unexpected::Other("invalid country"),
                    &"valid ISO-3166 country",
                )
            })
    }

    fn serialize_country<S: Serializer>(
        country: &Option<Country>,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        match country {
            Some(country) => ser.serialize_str(&country.iso2),
            None => ser.serialize_none(),
        }
    }

    impl GameServer {
        pub fn new(
            name: &str,
            address: &str,
            port: u16,
            desc: &str,
            location: Option<Country>,
            auth: &str,
            channel: Option<&str>,
            official: bool,
            extra: HashMap<String, Field>,
        ) -> Self {
            Self {
                name: name.to_string(),
                address: address.to_string(),
                port,
                description: desc.to_string(),
                location,
                auth_server: auth.to_string(),
                channel: channel.map(|c| c.to_string()),
                official,
                extra,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_server_list_ron_deserialize() {
        ron::de::from_reader::<_, GameServerList>(
            &include_bytes!("../examples/v1/example_server_list.ron")[..],
        )
        .unwrap();
    }

    #[test]
    fn check_server_list_json_deserialize() {
        serde_json::de::from_reader::<_, GameServerList>(
            &include_bytes!("../examples/v1/example_server_list.json")[..],
        )
        .unwrap();
    }

    #[test]
    fn check_server_list_json_roundtrip() {
        let data = serde_json::de::from_reader::<_, GameServerList>(
            &include_bytes!("../examples/v1/example_server_list.json")[..],
        )
        .unwrap();
        serde_json::to_string_pretty(&data).unwrap();
    }

    #[test]
    fn serialize_unknown_is_not_possible() {
        let field = Field {
            name: "never_serialze".to_string(),
            content: FieldContent::Unknown,
        };
        let result = serde_json::to_string(&field);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_data());
    }

    #[test]
    fn check_json_schema() {
        use jsonschema::JSONSchema;
        use serde_json::Value;
        let schema = serde_json::de::from_reader::<_, Value>(
            &include_bytes!("../examples/v1/schema.json")[..],
        )
        .unwrap();
        JSONSchema::compile(&schema).expect("A valid schema");
    }

    #[test]
    fn validate_json_schema() {
        use jsonschema::JSONSchema;
        use serde_json::Value;
        let schema = serde_json::de::from_reader::<_, Value>(
            &include_bytes!("../examples/v1/schema.json")[..],
        )
        .unwrap();
        let json = serde_json::de::from_reader::<_, Value>(
            &include_bytes!("../examples/v1/example_server_list.json")[..],
        )
        .unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&json);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
            panic!("json schema isn't valid");
        }
    }
}
