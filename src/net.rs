use crate::{Error, Result};
use core::str::FromStr;
use no_std_net as upstream;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Ipv4Addr(pub(crate) upstream::Ipv4Addr);

impl Ipv4Addr {
    pub const fn octets(&self) -> [u8; 4] {
        self.0.octets()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Ipv6Addr(pub(crate) upstream::Ipv6Addr);

impl Ipv6Addr {
    pub const fn octets(&self) -> [u8; 16] {
        self.0.octets()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl FromStr for IpAddr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.parse::<upstream::IpAddr>() {
            Ok(ip_addr) => match ip_addr {
                upstream::IpAddr::V4(ip_addr) => Ok(IpAddr::V4(Ipv4Addr(ip_addr))),
                upstream::IpAddr::V6(ip_addr) => Ok(IpAddr::V6(Ipv6Addr(ip_addr))),
            },
            Err(_) => Err(Error::InvalidIpAddr),
        }
    }
}