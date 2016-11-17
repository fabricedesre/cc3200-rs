// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use core::convert::TryInto;
use smallhttp::traits::{Channel, ChannelError};

use cc3200_sys::socket::{Family, Protocol, SocketType, RawSocket, sl_Socket, sl_Close, sl_Connect, sl_Send, sl_Recv};

pub struct SocketChannel {
    inner: RawSocket,
}

impl SocketChannel {
    pub fn new() -> Self {
        let socket = unsafe { sl_Socket(Family::AF_INET, SocketType::SOCK_STREAM, Protocol::IPPROTO_TCP) };
        // TODO: manage errors creating the socket...
        SocketChannel {
            inner: socket,
        }
    }
}

impl Drop for SocketChannel {
    fn drop(&mut self) {
        unsafe { sl_Close(self.inner); }
    }
}

impl Channel for SocketChannel {
    // Opens a channel to the given host:port destination, with TLS support is needed.
    fn open(&mut self, host: &str, port: i16, tls: bool) -> Result<(), ChannelError> {

    }

    // Tries to send `len` bytes. Returns the number of bytes successfully sent,
    // or an error.
    fn send(&self, data: &[u8], len: usize) -> Result<usize, ChannelError> {
        assert!(len < i16::max_value() as usize);
        let ret = unsafe { sl_Send(self.inner, data.as_ptr(), len as i16, 0 /* flags */) };
        // Rustc doesn't seem to return a SizeOrError but just a i16...
        if ret >= 0 {
            Ok(ret as usize)
        } else {
            Err(ChannelError::SomethingWentWrong)
        }
    }

    // Tries to receive at most `max_len` bytes. Returns the number of bytes successfully received,
    // or an error.
    fn recv(&self, data: &mut [u8], max_len: usize) -> Result<usize, ChannelError> {
        assert!(max_len < i16::max_value() as usize);
        let ret = unsafe { sl_Recv(self.inner, data.as_ptr() as *mut u8, max_len as i16, 0 /* flags */) };
        // Rustc doesn't seem to return a SizeOrError but just a i16...
        if ret >= 0 {
            Ok(ret as usize)
        } else {
            Err(ChannelError::SomethingWentWrong)
        }
    }
}
