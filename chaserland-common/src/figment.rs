use figment::{
    Metadata, Profile, Provider,
    providers::{Format, Json, Toml, Yaml},
    value::{Dict, Map},
};
use serde::de::Error;
use std::path::Path;

pub struct FileProvider(Box<dyn Provider>);

impl FileProvider {
    pub fn from<P>(provider: P) -> Self
    where
        P: Provider + 'static,
    {
        FileProvider(Box::new(provider))
    }

    pub fn try_from<P>(path: P) -> Result<Self, figment::Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let provider: Box<dyn Provider> = match path.extension() {
            Some(ext) if ext == "yaml" || ext == "yml" => Box::new(Yaml::file(path)),
            Some(ext) if ext == "json" => Box::new(Json::file(path)),
            Some(ext) if ext == "toml" => Box::new(Toml::file(path)),
            _ => return Err(figment::Error::custom("Unsupported file format")),
        };

        Ok(FileProvider(provider))
    }
}

impl Provider for FileProvider {
    fn metadata(&self) -> Metadata {
        self.0.metadata()
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        self.0.data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use figment::{
        Figment, Jail,
        providers::{Format, Json, Toml, Yaml},
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TestConfig {
        #[serde(rename = "string")]
        pub string_value: String,
        #[serde(rename = "int")]
        pub int_value: i32,
        #[serde(rename = "sub")]
        pub sub_config: SubConfig,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SubConfig {
        #[serde(rename = "value")]
        pub sub_value: String,
    }

    #[test]
    fn test_json() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "config.json",
                r#"
            {
                "string": "test",
                "int": 42,
                "sub": {
                    "value": "sub_test"
                }
            }
            "#,
            )?;

            let file_provider = FileProvider::from(Json::file("config.json"));
            let config: TestConfig = Figment::new().merge(file_provider).extract()?;

            let target = TestConfig {
                string_value: "test".into(),
                int_value: 42,
                sub_config: SubConfig {
                    sub_value: "sub_test".into(),
                },
            };

            assert_eq!(config, target);
            Ok(())
        });
    }

    #[test]
    fn test_yaml() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "config.yaml",
                r#"
            string: test
            int: 42
            sub:
                value: sub_test
            "#,
            )?;

            let file_provider = FileProvider::from(Yaml::file("config.yaml"));
            let config: TestConfig = Figment::new().merge(file_provider).extract()?;

            let target = TestConfig {
                string_value: "test".into(),
                int_value: 42,
                sub_config: SubConfig {
                    sub_value: "sub_test".into(),
                },
            };

            assert_eq!(config, target);
            Ok(())
        });
    }

    #[test]
    fn test_toml() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "config.toml",
                r#"
                string = "test"
                int = 42
                [sub]
                value = "sub_test"
                "#,
            )?;

            let file_provider = FileProvider::from(Toml::file("config.toml"));
            let config: TestConfig = Figment::new().merge(file_provider).extract()?;

            let target = TestConfig {
                string_value: "test".into(),
                int_value: 42,
                sub_config: SubConfig {
                    sub_value: "sub_test".into(),
                },
            };

            assert_eq!(config, target);
            Ok(())
        });
    }
}
