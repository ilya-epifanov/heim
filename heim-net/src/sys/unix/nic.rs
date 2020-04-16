use macaddr::MacAddr;
use nix::ifaddrs;
use nix::net::if_::InterfaceFlags;
use nix::sys::socket;

use heim_common::prelude::*;

use crate::Address;

#[derive(Debug)]
pub struct Nic(ifaddrs::InterfaceAddress);

impl Nic {
    pub fn name(&self) -> &str {
        self.0.interface_name.as_str()
    }

    pub fn address(&self) -> Address {
        self.0
            .address
            .as_ref()
            .expect("NIC stream should exclude entries without address")
            .into()
    }

    pub fn netmask(&self) -> Option<Address> {
        self.0.netmask.as_ref().map(Into::into)
    }

    pub fn broadcast(&self) -> Option<Address> {
        self.0.broadcast.as_ref().map(Into::into)
    }

    pub fn destination(&self) -> Option<Address> {
        self.0.destination.as_ref().map(Into::into)
    }

    pub fn is_up(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_UP)
    }

    pub fn is_broadcast(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_BROADCAST)
    }

    pub fn is_loopback(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_LOOPBACK)
    }

    pub fn is_point_to_point(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_POINTOPOINT)
    }

    pub fn is_multicast(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_MULTICAST)
    }
}

pub fn nic() -> Result<impl Iterator<Item = Result<Nic>>> {
    // `nix::ifaddrs` structs are not safe to send between threads,
    // so collecting them in a once
    let interfaces = ifaddrs::getifaddrs()?
        .filter_map(|addr| {
            // Skipping unsupported address families
            if addr.address.is_some() {
                Some(Nic(addr))
            } else {
                None
            }
        })
        .map(Ok);

    Ok(interfaces)
}

impl From<&socket::SockAddr> for Address {
    fn from(s: &socket::SockAddr) -> Self {
        use nix::sys::socket::SockAddr::*;

        match *s {
            Inet(addr) => Address::Inet(addr.to_std()),
            Link(addr) => Address::Link(MacAddr::from(addr.addr())),
            other => unimplemented!("Unknown sockaddr: {:?}", other),
        }
    }
}
