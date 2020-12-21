#[derive(Debug, Clone)]
pub struct Border {
    pub id: u32,
    base: String
}

impl Border {
    pub fn build(text: String) -> Border {
        Border {
            id: Border::parse_border(&text),
            base: text
        }
    }

    pub fn reverse(&self) -> Border {
        let text = &self.base.chars().rev().collect();
        Border {
            id: Border::parse_border(text),
            base: String::from(text)
        }
    }

    fn parse_border(border: &String) -> u32 {
        let mut value1 = 0;
        for (i, c) in border.chars().enumerate() {
            if c == '#' {
                value1 += 2u32.pow(i as u32);
            }
        }
        value1
    }
}