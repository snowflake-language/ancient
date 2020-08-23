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
    Dedent(usize),
    Ondent,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct IndentationLevel {
    pub stack: Vec<usize>,
}

impl IndentationLevel {
    pub fn new() -> IndentationLevel {
        IndentationLevel { stack: vec![] }
    }

    pub fn level(&self) -> usize {
        *self.stack.last().unwrap_or(&0)
    }

    pub fn update(&mut self, level: usize) -> Result<Indentation, &'static str> {
        if level > self.level() {
            self.stack.push(level);
            Ok(Indentation::Indent)
        } else if level < self.level() {
            if level == 0 || self.stack.iter().find(|&&x| x == level).is_some() {
                let stack_level = self.stack.len();
                while self.level() > level {
                    self.stack.pop();
                }
                Ok(Indentation::Dedent(stack_level - self.stack.len()))
            } else {
                Err("indentation level while dedenting does not match any previously indented level.")
            }
        } else {
            Ok(Indentation::Ondent)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indentation() {
        let mut indentation = IndentationLevel::new();
        assert_eq!(indentation.update(1), Ok(Indentation::Indent));
        assert_eq!(indentation.level(), 1);
        assert_eq!(indentation.update(0), Ok(Indentation::Dedent(1)));
        assert_eq!(indentation.level(), 0);
        assert_eq!(indentation.update(1), Ok(Indentation::Indent));
        assert_eq!(indentation.level(), 1);
        assert_eq!(indentation.update(3), Ok(Indentation::Indent));
        assert_eq!(indentation.level(), 3);
        assert_eq!(
            indentation.update(2),
            Err("indentation level while dedenting does not match any previously indented level.")
        );
        assert_eq!(indentation.level(), 3);
        assert_eq!(indentation.update(0), Ok(Indentation::Dedent(2)));
        assert_eq!(indentation.level(), 0);
    }

    #[test]
    fn test_indentation_stack_size() {
        let mut indentation = IndentationLevel::new();
        assert_eq!(indentation.update(1), Ok(Indentation::Indent));
        assert_eq!(indentation.level(), 1);
        assert_eq!(indentation.update(2), Ok(Indentation::Indent));
        assert_eq!(indentation.level(), 2);
        assert_eq!(indentation.update(3), Ok(Indentation::Indent));
        assert_eq!(indentation.level(), 3);
        assert_eq!(indentation.stack.len(), 3);
        assert_eq!(indentation.update(2), Ok(Indentation::Dedent(1)));
        assert_eq!(indentation.stack.len(), 2)
    }
}
