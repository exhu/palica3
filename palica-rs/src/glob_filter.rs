pub struct Pattern {}

impl Pattern {
    pub fn new(text: &str) -> Pattern {
        // TODO
        Pattern {}
    }

    pub fn accept(&self, text: &str) -> bool {
        // TODO
        false
    }
}

pub struct FilterItem {
    pub pattern_index: usize,
    pub include: bool,
}

impl FilterItem {
    pub fn include(&self, text: &str, patterns: &[Pattern]) -> bool {
        let accepted = patterns[self.pattern_index].accept(text);
        accepted == self.include
    }
}

pub struct Filter {
    pub patterns: Vec<Pattern>,
    pub items: Vec<FilterItem>,
}

impl Filter {
    pub fn include(&self, text: &str) -> bool {
        self.items
            .iter()
            .fold(false, |_, i| i.include(text, &self.patterns))
    }
}
