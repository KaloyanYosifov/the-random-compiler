use std::io::{BufRead, BufReader, Cursor, Read, Result as IOResult, Seek, SeekFrom};

pub trait SeekableBufRead: BufRead + Seek {}

impl<T: AsRef<[u8]>> SeekableBufRead for Cursor<T> {}
impl<T: Read + Seek> SeekableBufRead for BufReader<T> {}

pub struct LexerBufferReader {
    last_positions: Vec<u64>,
    peeked_char: Option<char>,
    buffer: Box<dyn SeekableBufRead>,
}

impl LexerBufferReader {
    pub fn new(buffer: Box<dyn SeekableBufRead>) -> Self {
        Self {
            buffer,
            peeked_char: None,
            last_positions: vec![],
        }
    }
}

impl LexerBufferReader {
    pub fn checkpoint(&mut self) -> Result<(), ()> {
        if let Ok(pos) = self.buffer.stream_position() {
            // if we have peeked already
            // set the actual pos to current - 1
            if self.peeked_char.is_some() {
                self.last_positions.push(pos - 1);
            } else {
                self.last_positions.push(pos);
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn read_line(&mut self, buf: &mut String) -> IOResult<usize> {
        let read_size = self.buffer.read_line(buf)?;
        *buf = buf.replace("\n", "");

        Ok(read_size)
    }

    pub fn read_char(&mut self) -> IOResult<char> {
        if let Some(char) = self.peeked_char.take() {
            return Ok(char);
        }

        let mut char_buf: [u8; 1] = [0; 1]; // we assume our source code is ASCII standard for now
        self.buffer.read_exact(&mut char_buf)?;

        Ok(char_buf[0] as char)
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        if !self.peeked_char.is_some() {
            match self.read_char() {
                Ok(char) => self.peeked_char = Some(char),
                _ => return None,
            }
        }

        self.peeked_char.as_ref()
    }

    pub fn back(&mut self) -> Result<u64, ()> {
        if let Some(pos) = self.last_positions.pop() {
            let seeked = self.buffer.seek(SeekFrom::Start(pos)).unwrap_or_default();

            self.peeked_char = None;

            Ok(seeked)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRING_FIXTURE: &str = "testing this\ninteresting thing";

    macro_rules! assert_next_line {
        ($reader:ident, $to_match_to:literal) => {{
            let mut line = String::from("");

            $reader.read_line(&mut line).unwrap();

            assert_eq!(line, $to_match_to);
        }};
    }

    macro_rules! assert_next_char {
        ($reader:ident, $to_match_to:literal) => {{
            let char = $reader.read_char().unwrap();

            assert_eq!(char, $to_match_to);
        }};
    }

    #[test]
    fn it_can_read_a_line_from_provided_buffer() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new("testing this")));

        assert_next_line!(reader, "testing this");
    }

    #[test]
    fn it_can_read_next_line_from_provided_buffer() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new(STRING_FIXTURE)));

        assert_next_line!(reader, "testing this");
        assert_next_line!(reader, "interesting thing");
    }

    #[test]
    fn it_can_store_a_checkpoint_and_go_back_to_it() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new(STRING_FIXTURE)));

        reader.checkpoint().unwrap();

        assert_next_line!(reader, "testing this");
        assert_next_line!(reader, "interesting thing");

        reader.back().unwrap();

        assert_next_line!(reader, "testing this");
    }

    #[test]
    fn it_can_store_a_checkpoint_and_go_back_to_it_for_single_characters_as_well() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new(STRING_FIXTURE)));

        reader.checkpoint().unwrap();

        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'e');
        assert_next_char!(reader, 's');

        reader.back().unwrap();

        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'e');
        assert_next_char!(reader, 's');

        reader.checkpoint().unwrap();

        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'i');
        assert_next_char!(reader, 'n');

        reader.back().unwrap();

        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'i');
        assert_next_char!(reader, 'n');
    }

    #[test]
    fn it_stores_checkpoint_correctly_even_if_we_have_peeked() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new(STRING_FIXTURE)));

        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'e');
        assert_eq!(reader.peek_char().unwrap(), &'s');

        reader.checkpoint().unwrap();

        assert_next_char!(reader, 's');

        reader.back().unwrap();

        assert_next_char!(reader, 's');
    }

    #[test]
    fn it_can_read_character_by_character() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new(STRING_FIXTURE)));

        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'e');
        assert_next_char!(reader, 's');
        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'i');
        assert_next_char!(reader, 'n');
        assert_next_char!(reader, 'g');

        assert_next_char!(reader, ' ');

        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'h');
        assert_next_char!(reader, 'i');
        assert_next_char!(reader, 's');

        assert_next_char!(reader, '\n');

        assert_next_char!(reader, 'i');
        assert_next_char!(reader, 'n');
        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'e');
        assert_next_char!(reader, 'r');
        assert_next_char!(reader, 'e');
        assert_next_char!(reader, 's');
        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'i');
        assert_next_char!(reader, 'n');
        assert_next_char!(reader, 'g');

        assert_next_char!(reader, ' ');

        assert_next_char!(reader, 't');
        assert_next_char!(reader, 'h');
        assert_next_char!(reader, 'i');
        assert_next_char!(reader, 'n');
        assert_next_char!(reader, 'g');
    }

    #[test]
    fn it_can_peek_next_char() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new(STRING_FIXTURE)));

        assert_eq!(reader.peek_char().unwrap(), &'t');
        assert_eq!(reader.peek_char().unwrap(), &'t');
        assert_eq!(reader.peek_char().unwrap(), &'t');
        assert_next_char!(reader, 't');
        assert_eq!(reader.peek_char().unwrap(), &'e');
    }

    #[test]
    fn it_returns_none_if_cannot_peek_next_char() {
        let mut reader = LexerBufferReader::new(Box::new(Cursor::new("")));

        assert!(reader.peek_char().is_none());
    }
}
