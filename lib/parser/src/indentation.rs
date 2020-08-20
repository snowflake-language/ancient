//
// parser - snowflake's parser
//
// copyright (c) 2020 the snowflake authors <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//


#[derive(Debug, Clone, PartialEq)]
pub enum Indentation {
    Indent,
    Dedent,
    Ondent,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct IndentationLevel {
    stack: Vec<usize>,
}

impl IndentationLevel {
    pub fn new() -> IndentationLevel {
        IndentationLevel { stack: vec![] }
    }

    pub fn level(&self) -> usize {
        *self.stack.last().unwrap_or(&0)
    }

    pub fn update(&mut self, level: usize) -> Indentation {
        if level > self.level() {
            self.stack.push(level);
            Indentation::Indent
        } else if level < self.level() {
            while self.stack.pop().unwrap_or(0) <= level {};
            self.stack.push(level);
            Indentation::Dedent
        } else {
            Indentation::Ondent
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indentation() {
        let mut indentation = IndentationLevel::new();
        assert_eq!(indentation.update(1), Indentation::Indent);
        assert_eq!(indentation.level(), 1);
        assert_eq!(indentation.update(0), Indentation::Dedent);
        assert_eq!(indentation.level(), 0);
        assert_eq!(indentation.update(2), Indentation::Indent);
        assert_eq!(indentation.level(), 2);
        assert_eq!(indentation.update(1), Indentation::Dedent);
        assert_eq!(indentation.level(), 1);
        assert_eq!(indentation.update(3), Indentation::Indent);
        assert_eq!(indentation.level(), 3);
        assert_eq!(indentation.update(0), Indentation::Dedent);
        assert_eq!(indentation.level(), 0)
    }
}
