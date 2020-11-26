pub fn deserialize_space_seperated_vec<'de, D, N>(deserializer: D) -> Result<Vec<N>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    N: std::str::FromStr + std::fmt::Debug,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    let s: std::borrow::Cow<str> = serde::de::Deserialize::deserialize(deserializer)?;
    let s_trimmed = s.trim();
    
    if s_trimmed.is_empty() {
        return Ok(vec![]);
    }
    
    let vec = s_trimmed.split(' ').map(|e| 
        e.to_string().parse::<N>().expect(format!("Failed to parse string '{}' into number", e).as_str())
    ).collect();

    Ok(vec)
}

pub fn deserialize_space_seperated_opt_vec<'de, D, N>(deserializer: D) -> Result<Option<Vec<N>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    N: std::str::FromStr + std::fmt::Debug,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    // FIXME: Cannot handle cases where field does not exist
    match deserialize_space_seperated_vec(deserializer) {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(e)
    }
}