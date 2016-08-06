use directory::{Directory, ReadOnlySource};
use std::io::{Cursor, Write, Seek, SeekFrom};
use std::io;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use directory::OpenError;
use directory::WritePtr;
use std::result;
use Result;

#[derive(Clone)]
pub struct SharedVec(Arc<RwLock<Cursor<Vec<u8>>>>);


pub struct RAMDirectory {
    fs: HashMap<PathBuf, SharedVec>,
}

impl SharedVec {
    pub fn new() -> SharedVec {
        SharedVec(Arc::new( RwLock::new(Cursor::new(Vec::new())) ))
    }
    
    pub fn copy_vec(&self,) -> Vec<u8> {
        self.0.read().unwrap().clone().into_inner()
    }
}

impl Write for SharedVec {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        try!(self.0.write().unwrap().write(buf));
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Seek for SharedVec {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.0.write().unwrap().seek(pos)
    }
}

impl fmt::Debug for RAMDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "RAMDirectory")
   }
}

impl RAMDirectory {
    pub fn create() -> RAMDirectory {
        RAMDirectory {
            fs: HashMap::new()
        }
    }
}

impl Directory for RAMDirectory {
    fn open_read(&self, path: &Path) -> result::Result<ReadOnlySource, OpenError> {
        match self.fs.get(path) {
            Some(ref data) => {
                let data_copy = data.copy_vec();
                Ok(ReadOnlySource::Anonymous(data_copy))
            },
            None => {
                Err(OpenError::FileDoesNotExist(PathBuf::from(path)))
            }
        }
    }
    fn open_write(&mut self, path: &Path) -> Result<WritePtr> {
        let full_path = PathBuf::from(&path);
        let data = SharedVec::new();
        self.fs.insert(full_path, data.clone());
        Ok(Box::new(data))
    }

    fn atomic_write(&mut self, path: &Path, data: &[u8]) -> Result<()> {
        let mut write = try!(self.open_write(path));
        try!(write.write_all(data));
        Ok(())
    }

    fn sync(&self, _: &Path) -> Result<()> {
        Ok(())
    }

    fn sync_directory(&self,) -> Result<()> {
        Ok(())
    }
}
