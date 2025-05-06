use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use line_index::LineIndex;
use rowan::TextRange;

use crate::{Ignore, Rule};

pub(crate) struct IgnoreIndex {
    line_to_ignored_names: HashMap<u32, HashSet<Rule>>,
    line_index: LineIndex,
}

impl fmt::Debug for IgnoreIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IgnoreIndex:")?;
        let mut keys = self.line_to_ignored_names.keys().collect::<Vec<_>>();
        keys.sort();
        for line in keys {
            if let Some(set) = &self.line_to_ignored_names.get(line) {
                writeln!(f, "  {line}: {set:?}")?;
            }
        }
        Ok(())
    }
}

impl IgnoreIndex {
    pub(crate) fn new(text: &str, ignores: &[Ignore]) -> Self {
        let line_index = LineIndex::new(text);
        let mut line_to_ignored_names: HashMap<u32, HashSet<Rule>> = HashMap::new();
        for ignore in ignores {
            let line = line_index.line_col(ignore.range.start()).line;
            line_to_ignored_names.insert(line, ignore.violation_names.clone());
        }
        // TODO: we want to report unused ignores
        Self {
            line_to_ignored_names,
            line_index,
        }
    }

    pub(crate) fn contains(&self, range: TextRange, item: Rule) -> bool {
        // TODO: hmmm basically we want to ensure that either it's on the line before or it's inside the start of the node. we parse stuff so that the comment ends up inside the node :/
        let line = self.line_index.line_col(range.start()).line;
        for line in [line, if line == 0 { 0 } else { line - 1 }] {
            if let Some(set) = self.line_to_ignored_names.get(&line) {
                if set.contains(&item) {
                    return true;
                }
            }
        }
        false
    }
}
