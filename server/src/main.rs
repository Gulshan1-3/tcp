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
      }
      }
      Err(e) => {
        eprintln!("ignore these packets {:?}",e)
      }
      }
      match etherparse::TcpHeaderSlice::from_slice(&buf[i.slice().len()..nbytes]) {
        Ok(tcph) => {

        }
      }
    }
 
  Ok(())
}
