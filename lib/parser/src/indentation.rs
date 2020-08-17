//
// parser - snowflake's parser
//
// copyright (c) 2020 the snowflake authors <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

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

    pub fn update(&mut self, level: usize) {
        if level > self.level() {
            self.stack.push(level);
        } else if level < self.level() {
            self.stack.pop();
        }
    }
}
