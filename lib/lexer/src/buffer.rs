use std::io::{BufRead, BufReader, Cursor, Read, Result as IOResult, Seek, SeekFrom};

pub trait SeekableBufRead: BufRead + Seek {}

impl<T: AsRef<[u8]>> SeekableBufRead for Cursor<T> {}
impl<T: Read + Seek> SeekableBufRead for BufReader<T> {}

pub struct LexerBufferReader {
    last_position: Option<u64>,
    last_positions: Vec<u64>,
    buffer: Box<dyn SeekableBufRead>,
}

impl LexerBufferReader {
    pub fn new(buffer: Box<dyn SeekableBufRead>) -> Self {
        Self {
            buffer,
            last_position: None,
            last_positions: vec![],
        }
    }
}

impl LexerBufferReader {
    pub fn read_line(&mut self, buf: &mut String) -> IOResult<usize> {
        if let Some(last_position) = self.last_position {
            self.last_positions.push(last_position);
        }

        // we want to store the last know position from the previous read line
        // so that we can return to the correct spot it started reading that line
        if let Ok(pos) = self.buffer.stream_position() {
            self.last_position = Some(pos);
        }

        let read_size = self.buffer.read_line(buf)?;
        *buf = buf.replace("\n", "");

        Ok(read_size)
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_next_line {
        ($reader:ident, $to_match_to:literal) => {{
            let mut line = String::from("");

            $reader.read_line(&mut line).unwrap();

            assert_eq!(line, $to_match_to);
        }};
    }

    #[test]
    fn it_can_read_a_line_from_provided_buffer() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new("testing this")));

        assert_next_line!(reader, "testing this");
    }

    #[test]
    fn it_can_read_next_line_from_provided_buffer() {
        let mut reader =
            LexerBufferReader::new(Box::new(Cursor::new("testing this\ninteresting thing")));

        assert_next_line!(reader, "testing this");
        assert_next_line!(reader, "interesting thing");
    }

    #[test]
    fn it_can_go_back_to_read_previous_line() {
        let mut reader =
            LexerBufferReader::new(Box::new(Cursor::new("testing this\ninteresting thing")));

        assert_next_line!(reader, "testing this");
        assert_next_line!(reader, "interesting thing");

        reader.back().unwrap();

        assert_next_line!(reader, "testing this");
    }
}
