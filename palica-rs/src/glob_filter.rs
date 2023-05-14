/*
    palica media catalogue program
    Copyright (C) 2023 Yury Benesh

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
pub struct Pattern {
    compiled: pcre::Pcre,
}

impl Pattern {
    pub fn new(text: &str) -> Pattern {
        Pattern {
            compiled: pcre::Pcre::compile(text)
                .expect(&format!("Failed to compile regexp: {}", text)),
        }
    }

    pub fn accept(&mut self, text: &str) -> bool {
        self.compiled.exec(text).is_some()
    }
}

pub struct FilterItem {
    pub pattern_index: usize,
    pub include: bool,
}

impl FilterItem {
    pub fn include(&self, text: &str, patterns: &mut [Pattern]) -> Option<bool> {
        let accepted = patterns[self.pattern_index].accept(text);
        if accepted {
            Some(accepted == self.include)
        } else {
            None
        }
    }
}

pub struct Filter {
    pub patterns: Vec<Pattern>,
    pub items: Vec<FilterItem>,
}

impl Filter {
    /// true if the text must be included (allowed by the filter)
    pub fn include(&mut self, text: &str) -> bool {
        self.items.iter().fold(false, |prev, i| {
            i.include(text, &mut self.patterns).unwrap_or(prev)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filter1() {
        let mut f = Filter {
            patterns: vec![Pattern::new(r"^.+$"), Pattern::new(r"/\.thumbnails$")],
            items: vec![
                FilterItem {
                    pattern_index: 0,
                    include: true,
                },
                FilterItem {
                    pattern_index: 1,
                    include: false,
                },
            ],
        };

        assert_eq!(f.include("abc"), true);
        assert_eq!(f.include("/abc/def/.thumbnails/jkk"), true);
        assert_eq!(f.include("/abc/def/.thumbnails"), false);
    }

    #[test]
    fn filter2() {
        let mut f = Filter {
            patterns: vec![
                Pattern::new(r"^.+$"),
                Pattern::new(r"/\.thumbnails$"),
                Pattern::new(r"/\.png$"),
            ],
            items: vec![
                FilterItem {
                    pattern_index: 0,
                    include: false,
                },
                FilterItem {
                    pattern_index: 2,
                    include: true,
                },
                FilterItem {
                    pattern_index: 1,
                    include: true,
                },
            ],
        };

        assert_eq!(f.include("abc"), false);
        assert_eq!(f.include("/abc/def/.thumbnails/jkk"), false);
        assert_eq!(f.include("/abc/def/.thumbnails"), true);
        assert_eq!(f.include("/abc/def/.png"), true);
    }
}
