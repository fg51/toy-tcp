use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex, RwLock, RwLockWriteGuard};
use std::time::{Duration, SystemTime};
use std::{cmp, ops::Range, str, thread};

use anyhow::{Context, Result};

use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
//use pnet::transport::{self, TransportChannelType};
use rand::{rngs::ThreadRng, Rng};

use crate::packet::TCPPacket;
use crate::socket::{SockID, Socket, TcpStatus};
use crate::tcpflags;

const UNDETERMINED_IP_ADDR: std::net::Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const UNDETERMINED_PORT: u16 = 0;
const MAX_TRANSMITTION: u8 = 5;
const RETRANMITTION_TIMEOUT: u64 = 3;
const MSS: usize = 1460;
const PORT_RANGE: Range<u16> = 40000..60000;

pub struct TCP {
    sockets: HashMap<SockID, Socket>,
}

impl Default for TCP {
    fn default() -> Self {
        let sockets = HashMap::new();
        let tcp = Self { sockets };
        tcp
    }
}

impl TCP {
    pub fn new() -> Self {
        Self::default()
    }

    fn select_unused_port(&self, _rng: &mut ThreadRng) -> Result<u16> {
        Ok(33445)
    }

    pub fn connect(&self, addr: Ipv4Addr, port: u16) -> Result<SockID> {
        let mut rng = rand::thread_rng();
        let mut socket = Socket::new(
            get_source_addr_to(addr)?,
            addr,
            self.select_unused_port(&mut rng)?,
            port,
        )?;
        socket.send_tcp_packet(tcpflags::SYN, &[])?;
        let sock_id = socket.get_sock_id();
        Ok(sock_id)
    }
}

fn get_source_addr_to(_addr: Ipv4Addr) -> Result<Ipv4Addr> {
    Ok("10.0.0.1".parse().unwrap())
}
