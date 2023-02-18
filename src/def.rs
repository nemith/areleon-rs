use std::collections::BTreeMap;

use format_serde_error::SerdeError;
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Definition {
    pub networks: BTreeMap<String, NetworkDefinition>,
    pub services: BTreeMap<String, Vec<ServiceDefinition>>,
}

impl Definition {
    pub fn from_yaml_str(input: &str) -> Result<Self> {
        let def = serde_yaml::from_str::<Definition>(input)
            .map_err(|err| SerdeError::new(input.to_string(), err))?;
        Ok(def)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NetworkDefinition {
    pub comment: Option<String>,
    pub values: Vec<Network>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Network {
    NamedRef(String),
    Network {
        comment: Option<String>,
        #[serde(alias = "address")]
        prefix: String,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum ServiceDefinition {
    NamedRef(String),
    Protocol(Protocol),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case", tag = "protocol")]
pub enum Protocol {
    Tcp { port: Port },
    Udp { port: Port },
    Icmp { r#type: u8, code: u8 },
    Numbered(u8),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Port {
    Number(u16),
    Range(PortRange),
}

#[derive(PartialEq, Debug)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

impl<'de> Deserialize<'de> for PortRange {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de::Error;
        let range = String::deserialize(deserializer)?;
        let (start, end) = range
            .split_once('-')
            .ok_or(Error::custom("invalid port range"))?;
        Ok(PortRange {
            start: start.trim().parse().map_err(Error::custom)?,
            end: end.trim().parse().map_err(Error::custom)?,
        })
    }
}

impl Serialize for PortRange {
    fn serialize<S>(self: &PortRange, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("{}-{}", self.start, self.end))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_yaml_parse_def() {
        let input = "
networks:
    RFC1918:
        values:
        - address: 10.0.0.0/8
        - address: 172.16.0.0/12
        - address: 192.168.0.0/16
    WEB_SERVERS:
        values:
        - address: 10.0.0.1/32
          comment: Web Server 1
        - address: 10.0.0.2/32
          comment: Web Server 2
    MAIL_SERVERS:
        values:
        - address: 10.0.0.3/32
          comment: Mail Server 1
        - address: 10.0.0.4/32
          comment: Mail Server 2
    ALL_SERVERS:
        values:
        - WEB_SERVERS
        - MAIL_SERVERS
services:
    HTTP:
        - protocol: tcp
          port: 80
    HTTPS:
        - protocol: tcp
          port: 443
    WEB:
        - HTTP
        - HTTPS
    HIGH_PORTS:
        - port: 1024-65535
          protocol: tcp
        - port: 1024-65535
          protocol: udp
        ";

        let def: Definition = serde_yaml::from_str(&input).unwrap();
        dbg!(&def);
        assert_eq!(def.networks.len(), 4);
    }
}
