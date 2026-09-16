use rowan::{TextRange, TextSize};

#[derive(Debug)]
pub struct DecodedText {
    text: String,
    marks: Vec<Mark>,
}

#[derive(Debug)]
struct Mark {
    decoded: u32,
    pos: u32,
}

impl DecodedText {
    pub fn new(pos: TextSize) -> Self {
        Self {
            text: String::new(),
            marks: vec![Mark {
                decoded: 0,
                pos: pos.into(),
            }],
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn into_text(self) -> String {
        self.text
    }

    pub fn push_str(&mut self, text: &str, pos: TextSize) {
        self.sync(pos);
        self.text.push_str(text);
    }

    pub fn push_char(&mut self, c: char, pos: TextSize) {
        self.sync(pos);
        self.text.push(c);
    }

    pub fn mark_end(&mut self, pos: TextSize) {
        self.sync(pos);
    }

    fn sync(&mut self, pos: TextSize) {
        let decoded = self.text.len() as u32;
        let pos = pos.into();
        match self.marks.last_mut() {
            Some(mark) if mark.decoded == decoded => mark.pos = pos,
            Some(mark) if mark.pos + (decoded - mark.decoded) == pos => (),
            _ => self.marks.push(Mark { decoded, pos }),
        }
    }

    pub fn source_pos(&self, offset: TextSize) -> TextSize {
        let offset = u32::from(offset);
        let idx = self.marks.partition_point(|mark| mark.decoded <= offset);
        let mark = &self.marks[idx.saturating_sub(1)];
        TextSize::new(mark.pos + offset.saturating_sub(mark.decoded))
    }

    pub fn source_range(&self, range: TextRange) -> TextRange {
        TextRange::new(self.source_pos(range.start()), self.source_pos(range.end()))
    }
}
