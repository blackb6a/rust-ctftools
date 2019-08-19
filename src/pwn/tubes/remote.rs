use std::net::{ToSocketAddrs, SocketAddr};

use super::sock::Sock;

pub struct Remote {
    sock_object: Sock,
    rhost: String,
    rport: u16,
    family,
    type,
    proto,
    sock
}

impl Remote {
    #[macro_use]
    macro_rules! remote {
        () => {
            
        };
    }
    fn __init__(host: String, port: u16, fam, typ, ssl, sock: bool, *args, **kwargs){
        let sock_object = Sock::sock(args, kwargs);
        if sock {

        } 
        else {

        }
    }
}