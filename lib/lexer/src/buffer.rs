use std::io::{BufRead, BufReader, Cursor, Read, Result as IOResult, Seek, SeekFrom};

pub trait SeekableBufRead: BufRead + Seek {}

impl<T: AsRef<[u8]>> SeekableBufRead for Cursor<T> {}
impl<T: Read + Seek> SeekableBufRead for BufReader<T> {}

pub struct LexerBufferReader {
    last_positions: Vec<u64>,
    buffer: Box<dyn SeekableBufRead>,
}

impl LexerBufferReader {
    pub fn new(buffer: Box<dyn SeekableBufRead>) -> Self {
        Self {
            buffer,
            last_positions: vec![],
        }
    }
}

impl LexerBufferReader {
    pub fn read_line(&mut self, buf: &mut String) -> IOResult<usize> {
        if let Ok(pos) = self.buffer.stream_position() {
            self.last_positions.push(pos);
        }

        self.buffer.read_line(buf)
    }

    pub fn back(&mut self) -> Result<u64, ()> {
        if let Some(pos) = self.last_positions.pop() {
            let seeked = self.buffer.seek(SeekFrom::Start(pos)).unwrap_or_default();

            Ok(seeked)
        } else {
            Err(())
        }
    }
}
