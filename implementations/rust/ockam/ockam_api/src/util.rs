use crate::config::lookup::{ConfigLookup, InternetAddress};
use core::str::FromStr;
use ockam::{Address, TCP};
use ockam_core::{Route, LOCAL};
use ockam_multiaddr::proto::{DnsAddr, Ip4, Ip6, Node, Service, Tcp};
use ockam_multiaddr::{MultiAddr, Protocol};
use std::net::{SocketAddrV4, SocketAddrV6};

/// Go through a multiaddr and remove all instances of
/// `/node/<whatever>` out of it and replaces it with a fully
/// qualified address to the target
pub fn clean_multiaddr(input: &MultiAddr, lookup: &ConfigLookup) -> Option<MultiAddr> {
    let mut new_ma = MultiAddr::default();
    let it = input.iter().peekable();
    for p in it {
        match p.code() {
            Node::CODE => {
                let alias = p.cast::<Node>()?;
                let addr = lookup
                    .get_node(&*alias)
                    .expect("provided invalid substitution route");

                match addr {
                    InternetAddress::Dns(dns, _) => new_ma.push_back(DnsAddr::new(dns)).ok()?,
                    InternetAddress::V4(v4) => new_ma.push_back(Ip4(*v4.ip())).ok()?,
                    InternetAddress::V6(v6) => new_ma.push_back(Ip6(*v6.ip())).ok()?,
                }

                new_ma.push_back(Tcp(addr.port())).ok()?;
            }
            _ => new_ma.push_back_value(&p).ok()?,
        }
    }

    Some(new_ma)
}

/// Try to convert a multi-address to an Ockam route.
pub fn multiaddr_to_route(ma: &MultiAddr) -> Option<Route> {
    let mut rb = Route::new();
    let mut it = ma.iter().peekable();
    while let Some(p) = it.next() {
        match p.code() {
            Ip4::CODE => {
                let ip4 = p.cast::<Ip4>()?;
                let tcp = it.next()?.cast::<Tcp>()?;
                let add = Address::new(TCP, SocketAddrV4::new(*ip4, *tcp).to_string());
                rb = rb.append(add)
            }
            Ip6::CODE => {
                let ip6 = p.cast::<Ip6>()?;
                let tcp = it.next()?.cast::<Tcp>()?;
                let add = Address::new(TCP, SocketAddrV6::new(*ip6, *tcp, 0, 0).to_string());
                rb = rb.append(add)
            }
            DnsAddr::CODE => {
                let host = p.cast::<DnsAddr>()?;
                if let Some(p) = it.peek() {
                    if p.code() == Tcp::CODE {
                        let tcp = p.cast::<Tcp>()?;
                        rb = rb.append(Address::new(TCP, format!("{}:{}", &*host, *tcp)));
                        let _ = it.next();
                        continue;
                    }
                }
                rb = rb.append(Address::new(TCP, &*host))
            }
            Service::CODE => {
                let local = p.cast::<Service>()?;
                rb = rb.append(Address::new(LOCAL, &*local))
            }

            // If your code crashes here then the front-end CLI isn't
            // properly calling `clean_multiaddr` before passing it to
            // the backend
            Node::CODE => unreachable!(),

            other => {
                error!(target: "ockam_api", code = %other, "unsupported protocol");
                return None;
            }
        }
    }
    Some(rb.into())
}

/// Try to convert a multiaddr to an Ockam Address
pub fn multiaddr_to_addr(ma: &MultiAddr) -> Option<Address> {
    let mut it = ma.iter().peekable();

    let proto = it.next()?;
    match proto.code() {
        DnsAddr::CODE => {
            let host = proto.cast::<DnsAddr>()?;
            if let Some(p) = it.peek() {
                if p.code() == Tcp::CODE {
                    let tcp = proto.cast::<Tcp>()?;
                    return Some(Address::new(TCP, format!("{}:{}", &*host, *tcp)));
                }
            }
        }
        Service::CODE => {
            let local = proto.cast::<Service>()?;
            return Some(Address::new(LOCAL, &*local));
        }
        _ => {}
    };

    None
}

/// Try to convert an Ockam Route into a MultiAddr.
pub fn route_to_multiaddr(r: &Route) -> Option<MultiAddr> {
    let mut ma = MultiAddr::default();
    for a in r.iter() {
        match a.transport_type() {
            TCP => {
                if let Ok(sa) = SocketAddrV4::from_str(a.address()) {
                    ma.push_back(Ip4::new(*sa.ip())).ok()?;
                    ma.push_back(Tcp::new(sa.port())).ok()?
                } else if let Ok(sa) = SocketAddrV6::from_str(a.address()) {
                    ma.push_back(Ip6::new(*sa.ip())).ok()?;
                    ma.push_back(Tcp::new(sa.port())).ok()?
                } else if let Some((host, port)) = a.address().split_once(':') {
                    ma.push_back(DnsAddr::new(host)).ok()?;
                    ma.push_back(Tcp::new(u16::from_str(port).ok()?)).ok()?
                } else {
                    ma.push_back(DnsAddr::new(a.address())).ok()?
                }
            }
            LOCAL => ma.push_back(Service::new(a.address())).ok()?,
            other => {
                error!(target: "ockam_api", transport = %other, "unsupported transport type");
                return None;
            }
        }
    }
    Some(ma)
}

#[test]
fn clean_multiaddr_simple() {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    let addr: MultiAddr = "/node/hub/service/echoer".parse().unwrap();

    let lookup = {
        let mut map = ConfigLookup::new();
        map.set_node(
            "hub",
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 666)).into(),
        );
        map
    };

    let new_addr = clean_multiaddr(&addr, &lookup).unwrap();
    assert_ne!(addr, new_addr); // Make sure the address changed

    let new_route = multiaddr_to_route(&new_addr).unwrap();
    println!("{:#?}", new_route);
}
