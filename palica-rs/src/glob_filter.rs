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
    pub fn include(&self, text: &str, patterns: &mut [Pattern]) -> bool {
        let accepted = patterns[self.pattern_index].accept(text);
        accepted == self.include
    }
}

pub struct Filter {
    pub patterns: Vec<Pattern>,
    pub items: Vec<FilterItem>,
}

impl Filter {
    /// true if the text must be included (allowed by the filter)
    pub fn include(&mut self, text: &str) -> bool {
        self.items
            .iter()
            .fold(false, |_, i| i.include(text, &mut self.patterns))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filter() {
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
}
