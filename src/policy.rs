use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Policy {
    pub filters: Vec<Filter>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Filter {
    pub header: Header,
    pub terms: Vec<Term>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Header {
    pub comment: Option<String>,
    pub apply_groups: Option<Vec<String>>,
    pub apply_groups_except: Option<Vec<String>>,
    // pub targets: Vec<TargetOptions>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum TargetOptions {
    Aruba(),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Term {
    pub name: String,
    pub action: Action,
    pub option: Option<ConnOption>,

    pub protocol: Protocol,
    pub protocol_except: Vec<Protocol>,

    pub source_addrss: String, // todo make this an address rtype
    pub source_exclude: String,
    pub destination_address: String, // todo make these more dynami
    pub desintation_exclude: String,

    pub source_port: u16,
    pub destination_port: u16,
    pub icmp_type: IcmpType,

    pub verbatim: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Action {
    Accept,
    Deny,
    Reject,
    Next,
    RejectWithTcpRst,
}

#[derive(Serialize, Deserialize. PartialEq, Debug)]
#[repr(u8)]
pub enum Protocol {
    Tcp = 6,
    Udp = 17,
    Icmp = 1,
    Numbered(u8),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum IcmpType {
    Icmp4(Icmp4Types),
    Icmp6(Icmp6Types),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[repr(u8)]
pub enum Icmp4Types {
    EchoReply = 1,
    Unreachable = 3,
    SouceQuench = 4,
    Redirect = 5,
    AlternatAddress = 6,
    EchoRequest = 7,
    RouterAdvertisment = 9,
    RouterSolicitation = 10,
    TimeExceeded = 11,
    ParameterProblem = 12,
    TimestampRequest = 13,
    TimestampReply = 14,
    InformationRequest = 15,
    InformationReply = 16,
    MaskRequest = 17,
    MaskReply = 18,
    CoversionError = 31,
    MobileRedirect = 32,
    Unnamed(u8),
}

impl From<u8> for Icmp4Types {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::EchoReply,
            3 => Self::Unreachable,
            4 => Self::SouceQuench,
            5 => Self::Redirect,
            6 => Self::AlternatAddress,
            7 => Self::EchoRequest,
            9 => Self::RouterAdvertisment,
            10 => Self::RouterSolicitation,
            11 => Self::TimeExceeded,
            12 => Self::ParameterProblem,
            13 => Self::TimestampRequest,
            14 => Self::TimestampReply,
            15 => Self::InformationRequest,
            16 => Self::InformationReply,
            17 => Self::MaskRequest,
            18 => Self::MaskReply,
            31 => Self::CoversionError,
            32 => Self::MobileRedirect,
            _ => Self::Unnamed(value),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Icmp6Types {
    DestinationUnreachable,
    PacketTooBig,
    TimeExceeded,
    ParameterProblem,
    EchoRequest,
    EchoReply,
    MulticastListenerQuery,
    MulticastListenerReport,
    MulticastListenerDone,
    RouterSolicit,
    RouterAdvertisement,
    NeighborSolicit,
    NeighborAdvertisement,
    RedirectMessage,
    RouterRenumbering,
    IcmpNodeInformationQuery,
    IcmpNodeInformationResponse,
    InverseNeighborDiscoverySolicitation,
    InverseNeighborDiscoveryAdvertisement,
    Version2MulticastListenerReport,
    HomeAgentAddressDiscoveryRequest,
    HomeAgentAddressDiscoveryReply,
    MobilePrefixSolicitation,
    MobilePrefixAdvertisement,
    CertificationPathSolicitation,
    CertificationPathAdvertisement,
    MulticastRouterAdvertisement,
    MulticastRouterSolicitation,
    MulticastRouterTermination,
    Unnamed(u8),
}

impl From<u8> for Icmp6Types {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::DestinationUnreachable,
            2 => Self::PacketTooBig,
            3 => Self::TimeExceeded,
            4 => Self::ParameterProblem,
            128 => Self::EchoRequest,
            129 => Self::EchoReply,
            130 => Self::MulticastListenerQuery,
            131 => Self::MulticastListenerReport,
            132 => Self::MulticastListenerDone,
            133 => Self::RouterSolicit,
            134 => Self::RouterAdvertisement,
            135 => Self::NeighborSolicit,
            136 => Self::NeighborAdvertisement,
            137 => Self::RedirectMessage,
            138 => Self::RouterRenumbering,
            139 => Self::IcmpNodeInformationQuery,
            140 => Self::IcmpNodeInformationResponse,
            141 => Self::InverseNeighborDiscoverySolicitation,
            142 => Self::InverseNeighborDiscoveryAdvertisement,
            143 => Self::Version2MulticastListenerReport,
            144 => Self::HomeAgentAddressDiscoveryRequest,
            145 => Self::HomeAgentAddressDiscoveryReply,
            146 => Self::MobilePrefixSolicitation,
            147 => Self::MobilePrefixAdvertisement,
            148 => Self::CertificationPathSolicitation,
            149 => Self::CertificationPathAdvertisement,
            151 => Self::MulticastRouterAdvertisement,
            152 => Self::MulticastRouterSolicitation,
            153 => Self::MulticastRouterTermination,
            _ => Self::Unnamed(value),
        }
    }
}

enum ConnOption {
    Estabilished,
    TcpEstablished,
    Sample,
    Initial,
    Rst,
    FirstFragement,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_yaml_parse_def() {
        let input = "
            unnamed
        ";

        let typ: Icmp4Types = serde_yaml::from_str(&input).unwrap();
        dbg!(&typ);
    }
}
