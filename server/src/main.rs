use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;
struct TcpState {}
struct Quad{
  src:(Ipv4Addr,u16),
  dst:(Ipv4Addr,u16),
}
fn main() -> io::Result<()>{
  let mut connections:HashMap<TcpState,Quad> = Default::default();
  let nic =  tun_tap::Iface::new("mytun",tun_tap::Mode::Tun)?;
  let mut buf = [0u8;1504];
  loop {
    let nbytes =  nic.recv(&mut buf[..])?;
    let flags = u16::from_be_bytes([buf[0],buf[1]]);
    let proto = u16::from_be_bytes([buf[2],buf[3]]);
    if proto != 0x0800 {
      continue;
    }
    match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
      Ok(i) =>{
        let src = i.source_addr();
        let dst = i.destination_addr();
        let proto = i.protocol();
        let proto = i.protocol(); 
        if i.protocol() != 0x06 {
          eprintln!("BAD PROTOCOL");
          // not tcp
          continue;
      }
      }
      Err(e) => {
        eprintln!("ignore these packets {:?}",e)
      }
      }
      match etherparse::TcpHeaderSlice::from_slice(&buf[i.slice().len()..nbytes]) {
        Ok(TCP) {

        }
      }
    }

  Ok(())
}

