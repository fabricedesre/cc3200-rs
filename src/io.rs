extern crate cc3200_sys;

use collections::String;
use collections::Vec;

use core::convert::TryFrom;
use core::ptr;

use self::cc3200_sys::simplelink::*;

macro_rules! try_fs {
    ($e:expr) => ({
        let rc: i32 = unsafe { $e };
        if rc < 0 {
            return Err(SimpleLinkError::FileSystem(try!(FileSystemError::try_from(rc))));
        }
        rc
    })
}

pub trait Read {
    fn read(&mut self, buf: &mut[u8]) -> Result<usize, SimpleLinkError>;
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, SimpleLinkError>;
    fn read_to_string(&mut self, str: &mut String) -> Result<usize, SimpleLinkError>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, SimpleLinkError>;
}

pub trait Seek {
    fn seek(&mut self, pos: u64) -> Result<u64, SimpleLinkError>;
}

pub struct File {
    offset: usize,
    file_handle: i32,
}

impl File {
    /// Returns a file's info structure from the file system
    pub fn get_info(file_name: &str) -> Result<SlFsFileInfo, SimpleLinkError> {
        let mut file_info = SlFsFileInfo {
            flags : 0,
            file_length : 0,
            allocated_length : 0,
            token : [0; 4]
        };
        try_fs!(sl_FsGetInfo(file_name.as_ptr(), 0, &mut file_info) as i32);
        Ok(file_info)
    }

    /// Removes a file fro the file system
    pub fn remove(file_name: &str) -> Result<(), SimpleLinkError> {
        try_fs!(sl_FsDel(file_name.as_ptr(), 0) as i32);
        Ok(())
    }

    /// Opens a file for reading
    pub fn open(file_name: &str) -> Result<File, SimpleLinkError> {
        File::open_with_mode(file_name, File::mode(false, false, false, 0))
    }

    /// Opens a file for writing; possibly creating it in the process
    pub fn create(file_name: &str, max_len: usize, failsafe: bool) -> Result<File, SimpleLinkError> {
        File::open_with_mode(file_name, File::mode(true, true, failsafe, max_len as u32))
    }

    // Returns the file-open mode
    fn mode(write: bool, create: bool, failsafe: bool, max_size: u32) -> u32 {
        unsafe { sl_FsMode(write, create, failsafe, max_size) }
    }

    // Open file with the specified mode
    fn open_with_mode(file_name: &str, mode: u32) -> Result<File, SimpleLinkError> {
        let mut file_handle = -1 as i32;
        try_fs!(sl_FsOpen(file_name.as_ptr(), mode, ptr::null(), &mut file_handle));
        Ok(File { offset: 0, file_handle: file_handle })
    }

    // Read at specific offset
    fn read_at(&self, buf: &mut[u8], offset: usize) -> Result<usize, SimpleLinkError> {
        Ok(try_fs!(sl_FsRead(self.file_handle,
                             offset as u32,
                             buf.as_mut_ptr(),
                             buf.len() as u32)) as usize)
    }

    // Write at specific offset
    fn write_at(&self, buf: &[u8], offset: usize) -> Result<usize, SimpleLinkError> {
        Ok(try_fs!(sl_FsWrite(self.file_handle,
                              offset as u32,
                              buf.as_ptr(),
                              buf.len() as u32)) as usize)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            sl_FsClose(self.file_handle, ptr::null(), ptr::null(), 0);
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut[u8]) -> Result<usize, SimpleLinkError> {
        let len = try!(self.read_at(buf, self.offset));
        self.offset += len;
        Ok(len)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, SimpleLinkError> {

        let incr_buffer_size = 64;

        let mut offset = self.offset;
        let mut len = 0; // number of valid bytes in buf

        loop {
            if len == buf.len() {
                let resize = buf.len() + incr_buffer_size;
                buf.resize(resize, 0);
            }

            let readlen = try!(self.read_at(&mut buf[len..], offset));

            len += readlen;
            offset += readlen;

            if len < buf.len() {
                break; // EOF
            }
        }

        buf.truncate(len);
        self.offset = offset;

        Ok(len)
    }

    fn read_to_string(&mut self, str: &mut String) -> Result<usize, SimpleLinkError> {
        let mut buf: Vec<u8> = Vec::new();
        let len = try!(self.read_to_end(&mut buf));

        match String::from_utf8(buf) {
            Ok(res) => {
                str.clear();
                str.push_str(&res);
            },
            Err(_) => {
                return Result::Err(SimpleLinkError::FileSystem(FileSystemError::NOT_SUPPORTED))
            },
        }

        Ok(len)
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize, SimpleLinkError> {
        let len = try!(self.write_at(buf, self.offset));
        self.offset += len;
        Ok(len)
    }
}

impl Seek for File {
    fn seek(&mut self, pos: u64) -> Result<u64, SimpleLinkError> {
        self.offset = pos as usize;
        Ok(self.offset as u64)
    }
}
