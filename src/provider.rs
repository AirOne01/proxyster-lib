use serde_derive::Deserialize;

use crate::provider_source::ProviderSource;

/**
A proxy provider

# Examples

(in `providers.toml`)

```toml
[[proxy]]
name = "proxy1"
sources = ["https://raw.githubusercontent.com/..."]
```
*/
#[derive(Deserialize)]
pub struct Provider {
    pub name: String,
    pub sources: Vec<ProviderSource>, // array of InitialProviderSource
}

// implement From for Providerpo
impl From<toml::Value> for Provider {
    fn from(value: toml::Value) -> Self {
        let sources: Vec<ProviderSource> = value
            .get("sources")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|v| ProviderSource::from(v.clone()))
            .collect();

        Provider {
            name: value.get("name").unwrap().as_str().unwrap().to_owned(),
            sources,
        }
    }
}

