pub type BufferResult<T> = Result<T, Error>;

#[derive(Debug,PartialEq,Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T> {
    // size is always 1 larger than requested. This 1 element is used to distinguish between full
    // and empty buffers. If read and writeptr point to the same element, then the buffer is empty.
    // If the write pointer points to read-1, then the buffer is full.
    data: Vec<Option<T>>,
    size: usize,
    // points to the next place to read and return
    readptr: usize,
    // points to the next place to write data
    writeptr: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        let mut buf = Vec::with_capacity(size + 1);
        for _ in 0..size + 1 {
            buf.push(None)
        }
        CircularBuffer {
            data: buf,
            size: size + 1,
            readptr: 0,
            writeptr: 0,
        }
    }

    pub fn clear(&mut self) {
        self.readptr = 0;
        self.writeptr = 0;
    }

    pub fn read(&mut self) -> BufferResult<T> {
        if self.readptr == self.writeptr {
            return Err(Error::EmptyBuffer);
        }
        let tmp = self.data[self.readptr].take().unwrap();
        self.readptr = (self.readptr + 1) % self.size;
        return Ok(tmp);
    }

    pub fn write(&mut self, item: T) -> BufferResult<()> {
        if (self.writeptr + 1) % self.size == self.readptr {
            return Err(Error::FullBuffer);
        }
        self.data[self.writeptr] = Some(item);
        self.writeptr = (self.writeptr + 1) % self.size;
        Ok(())
    }

    pub fn overwrite(&mut self, item: T) {
        self.data[self.writeptr] = Some(item);
        self.writeptr = (self.writeptr + 1) % self.size;
        // increment readptr if necessar to keep the delimiter element
        if self.readptr == self.writeptr {
            self.readptr = (self.writeptr + 1) % self.size;
        }
    }
}
