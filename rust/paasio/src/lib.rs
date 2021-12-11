use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    read_val: R,
    bytes: usize,
    count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            read_val: wrapped,
            bytes: 0,
            count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.read_val
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_val.read(buf).map(|read_bytes| {
            self.count += 1;
            self.bytes += read_bytes;
            read_bytes
        })
    }
}

pub struct WriteStats<W> {
    write_val: W,
    bytes: usize,
    count: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            write_val: wrapped,
            bytes: 0,
            count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.write_val
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_val.write(buf).map(|written_bytes| {
            self.count += 1;
            self.bytes += written_bytes;
            written_bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.write_val.flush()
    }
}
