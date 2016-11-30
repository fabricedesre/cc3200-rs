// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use core::mem;
use cc3200_sys::socket::{Family, Protocol, SocketError, SocketType, RawSocket, sl_Socket,
                         sl_Close, sl_Connect, sl_Send, sl_Recv, SlSockAddrIn_t, SlSockAddr_t,
                         sl_Htonl, sl_Htons};
use simplelink::SimpleLink;
use smallhttp::traits::{Channel, ChannelError};

#[derive(Clone)]
pub struct SocketChannel {
    inner: RawSocket,
}

impl SocketChannel {
    pub fn new() -> Option<Self> {
        let socket = unsafe {
            sl_Socket(Family::AF_INET,
                      SocketType::SOCK_STREAM,
                      Protocol::IPPROTO_TCP)
        };
        if socket >= 0 {
            Some(SocketChannel { inner: socket })
        } else {
            None
        }

    }
}

impl Drop for SocketChannel {
    fn drop(&mut self) {
        unsafe {
            sl_Close(self.inner);
        }
    }
}

impl Channel for SocketChannel {
    // Opens a channel to the given host:port destination, with TLS support is needed.
    fn open(&mut self, host: &str, port: u16, tls: bool) -> Result<(), ChannelError> {
        // Convert the host name into a socket address.
        let addr = match SimpleLink::netapp_get_host_by_name(host) {
            Ok(addr) => addr,
            Err(err) => {
                error!("Unable to resolve {} : {}", host, err);
                return Err(ChannelError::InvalidHostName);
            }
        };

        // TODO: use getsockopt to set the TLS options if tls is true.
        if tls {
            return Err(ChannelError::TlsUnsupported);
        }

        let inaddr = SlSockAddrIn_t {
            sin_family: Family::AF_INET,
            sin_port: unsafe { sl_Htons(port) },
            sin_addr: unsafe { sl_Htonl(addr) },
            sin_zero: [0; 8],
        };
        let sockaddr =
            unsafe { ::core::intrinsics::transmute::<SlSockAddrIn_t, SlSockAddr_t>(inaddr) };
        let ret = unsafe {
            sl_Connect(self.inner,
                       &sockaddr as *const SlSockAddr_t,
                       mem::size_of::<SlSockAddrIn_t>() as i16)
        };

        if ret != SocketError::SOC_OK {
            debug!("Unable to connect to {:?} : {:?}", sockaddr, ret);
            Err(ChannelError::UnableToConnect)
        } else {
            Ok(())
        }
    }

    // Tries to send `len` bytes. Returns the number of bytes successfully sent,
    // or an error.
    fn send(&mut self, data: &[u8], len: usize) -> Result<usize, ChannelError> {
        assert!(len < i16::max_value() as usize);
        let ret = unsafe {
            sl_Send(self.inner, data.as_ptr(), len as i16, 0 /* flags */)
        };
        // Rustc doesn't seem to return a SizeOrError but just a i16...
        if ret >= 0 {
            Ok(ret as usize)
        } else {
            debug!("Unable to send: {:?}", ret);
            Err(ChannelError::SomethingWentWrong)
        }
    }

    // Tries to receive at most `max_len` bytes. Returns the number of bytes successfully received,
    // or an error.
    fn recv(&mut self, data: &mut [u8], max_len: usize) -> Result<usize, ChannelError> {
        assert!(max_len < i16::max_value() as usize);
        let ret = unsafe {
            sl_Recv(self.inner,
                    data.as_ptr() as *mut u8,
                    max_len as i16,
                    0 /* flags */)
        };
        // Rustc doesn't seem to return a SizeOrError but just a i16...
        if ret > 0 {
            Ok(ret as usize)
        } else if ret == 0 {
            Err(ChannelError::EndOfStream)
        } else {
            Err(ChannelError::SomethingWentWrong)
        }
    }
}
