#[cfg(test)]
pub use net::{TcpListener, TcpStream};
#[cfg(not(test))]
pub use std::net::{TcpListener, TcpStream};

#[cfg(test)]
pub mod net {
    use crate::network::packet;
    use crate::network::test::AutoGenFuzz;
    use std::iter::Iterator;
    use std::vec;
    use std::{io, io::Error, result};

    #[derive(Clone)]
    pub struct TcpStream {
        inputs: vec::Vec<packet::Packet>,
        outputs: vec::Vec<packet::Packet>,
    }

    impl TcpStream {
        pub fn new(inputs: vec::Vec<packet::Packet>, outputs: vec::Vec<packet::Packet>) -> Self {
            Self { inputs, outputs }
        }

        pub fn peer_addr(&self) -> Result<String, Error> {
            Ok("".to_string())
        }

        pub fn read_exact(&mut self, buff: &mut [u8]) -> Result<(), Error> {
            assert_eq!(buff.len(), packet::BUFFER_SIZE);
            match self.inputs.pop() {
                Some(data) => {
                    buff.copy_from_slice(&data.pub_pack());
                    Ok(())
                }
                None => Err(Error::new(
                    io::ErrorKind::NotConnected,
                    "No more data delivered for this test",
                )),
            }
        }

        pub fn connect(buff: &str) -> Result<Self, Error> {
            todo!()
        }

        pub fn set_nonblocking(&self, b: bool) -> Result<(), Error> {
            Ok(())
        }

        pub fn write_all(&mut self, buff: &[u8]) -> Result<(), Error> {
            let mut internal_buffer = [0_u8; packet::BUFFER_SIZE];
            internal_buffer.copy_from_slice(buff);
            match self.outputs.pop() {
                Some(data) => {
                    assert_eq!(internal_buffer, data.pub_pack());
                    Ok(())
                }
                None => {
                    panic!("No more data available to send, please rewrite you're tests")
                }
            }
        }
    }

    pub struct TcpListener;
    pub struct Incomming;

    impl Iterator for Incomming {
        type Item = io::Result<TcpStream>;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    impl TcpListener {
        pub fn bind(addr: &str) -> io::Result<Self> {
            Ok(TcpListener)
        }

        pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
            Ok(())
        }

        pub fn incoming(&self) -> Incomming {
            todo!()
        }
    }
}
