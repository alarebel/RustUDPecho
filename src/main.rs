extern crate mio;
extern crate bytes;
extern crate slab;
extern crate time;
use mio::*;
use mio::deprecated::{EventLoop, Handler};
use slab::*;
use mio::udp::*;
use std::net::{SocketAddr, SocketAddrV4};
use bytes::deprecated::{Buf, ByteBuf, MutByteBuf, SliceBuf};

const START_PORT:u16 = 1024;
const END_PORT:u16 = 65535;
const BUF_CAP:usize = 4096;
const PACKETS_INC:u64 = 100000;

const PRINT_PACKETS:bool = true;
const PRINT_DATA:bool = false;
const SEND_ECHO:bool = true;
const MEASURE_PPS:bool = false;

type UdpSlab = Slab<UdpSocket, Token>;
type EvLoop = EventLoop<UdpHandler>;

struct UdpHandler {
	sockets: UdpSlab,
	n: u64,
	t: time::Timespec,
	buf: MuteByteBuf
}

impl UdpHandler{
	fn new(sockets:Slab<UdpSocket, Token>)->UdpHandler{
		UdpHandler{
			sockets: sockets,
			n: 0,
			t: time::get_time(),
			buf: ByteBuf::mut_with_capacity(BUF_CAP)
		}
	}
}


fn main(){
	println!("done");
}
