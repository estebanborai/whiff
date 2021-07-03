pub mod ip_header;
pub mod raw;
pub mod tcp;

#[derive(Debug)]
pub enum Packet {
    Tcp(tcp::Tcp),
    Unknown(raw::Raw),
}