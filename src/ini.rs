use crate::error::Result;
use crate::utils::ini_to_string;
use ini::Ini;

#[derive(Debug, Clone)]
pub struct IniSerializerConfig {
    pub pretty: bool,
    pub assignment: String,
    pub line_break: String,
    pub comment_char: String,
    pub deduplicate: bool,
}

impl Default for IniSerializerConfig {
    fn default() -> Self {
        Self {
            pretty: false,
            assignment: "=".to_string(),
            line_break: "\n".to_string(),
            comment_char: "#".to_string(),
            deduplicate: false,
        }
    }
}

pub struct IniSerializer {
    config: IniSerializerConfig,
}

impl IniSerializer {
    pub fn new(config: Option<IniSerializerConfig>) -> Self {
        Self {
            config: config.unwrap_or_default(),
        }
    }

    pub fn serialize_section(&self, section: &str, properties: &[(String, String)]) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(section);

        for (key, value) in properties {
            ini.with_section(section).set(key, value);
        }

        ini_to_string(&ini)
    }

    pub fn add_comment(&self, text: &str) -> String {
        format!("{} {}", self.config.comment_char, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_section() {
        let serializer = IniSerializer::new(None);
        let properties = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];
        
        let result = serializer.serialize_section("test", &properties).unwrap();
        assert!(result.contains("[test]"));
        assert!(result.contains("key1=value1"));
        assert!(result.contains("key2=value2"));
    }

    #[test]
    fn test_add_comment() {
        let serializer = IniSerializer::new(None);
        let comment = serializer.add_comment("Test comment");
        assert_eq!(comment, "# Test comment");
    }
}
