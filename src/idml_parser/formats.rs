use serde::de::{Deserialize, Deserializer};

/// Deserializer yielding a vec [N, N, N, N, ...] given a space seperated  string "N N N N ..."
pub fn deserialize_space_seperated_vec<'de, D, N>(deserializer: D) -> Result<Vec<N>, D::Error>
where
    D: Deserializer<'de>,
    N: std::str::FromStr + std::fmt::Debug,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    let s: std::borrow::Cow<str> = Deserialize::deserialize(deserializer)?;
    match s.trim() {
        "" => Ok(vec![]),
        s_trimmed => {
            let vec = s_trimmed
                .split(' ')
                .map(|e| {
                    e.to_string()
                        .parse::<N>()
                        .expect(format!("Failed to parse string '{}' into number", e).as_str())
                })
                .collect();

            Ok(vec)
        }
    }
}

/// Deserializer yielding a vec [N, N, N, N, ...] given a space seperated  string "N N N N ..."
pub fn deserialize_space_seperated_opt_vec<'de, D, N>(
    deserializer: D,
) -> Result<Option<Vec<N>>, D::Error>
where
    D: Deserializer<'de>,
    N: std::str::FromStr + std::fmt::Debug,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    match deserialize_space_seperated_vec(deserializer) {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(e),
    }
}

/// Deserializer that yielding valid IDML IDs. For example `"n"` will yield `None` and `"u123"` will yield `Some("u123")`
pub fn deserialize_id_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: Result<std::borrow::Cow<str>, D::Error> =
        serde::de::Deserialize::deserialize(deserializer);

    match result {
        Ok(s) => {
            let s_trimmed = s.trim();
            let id = match s_trimmed {
                "" => None,
                "n" => None,
                _ => Some(s_trimmed.to_owned()),
            };
            Ok(id)
        }
        Err(e) => Err(e),
    }
}
