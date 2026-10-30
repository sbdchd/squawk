use std::ops::Index;

use crate::arena::{IdxRange, LayeredMultimap};
use crate::interner::Name;
use crate::ir::{Statistics, StatisticsId, TableId};
use crate::qualified_index::QualifiedIndex;

#[derive(Debug, Default)]
pub struct StatisticsIndex {
    statistics: QualifiedIndex<Statistics>,
    by_table: LayeredMultimap<TableId, StatisticsId>,
}

impl StatisticsIndex {
    pub(crate) fn on_top_of(base: &'static StatisticsIndex) -> Self {
        StatisticsIndex {
            statistics: QualifiedIndex::on_top_of(&base.statistics),
            by_table: LayeredMultimap::on_top_of(&base.by_table),
        }
    }

    pub(crate) fn alloc(&mut self, statistics: Statistics) -> StatisticsId {
        let table = statistics.table;
        let id = self.statistics.alloc(statistics);
        if let Some(table) = table {
            self.by_table.insert(table, id);
        }
        id
    }

    pub(crate) fn named(
        &self,
        schema: Name,
        name: Name,
    ) -> impl Iterator<Item = StatisticsId> + '_ {
        self.statistics.named(schema, name)
    }

    pub(crate) fn in_schema(
        &self,
        schema: Name,
    ) -> impl DoubleEndedIterator<Item = StatisticsId> + '_ {
        self.statistics.in_schema(schema)
    }

    pub(crate) fn on_table(
        &self,
        table: TableId,
    ) -> impl DoubleEndedIterator<Item = StatisticsId> + '_ {
        self.by_table.get(table)
    }

    pub(crate) fn next_idx(&self) -> StatisticsId {
        self.statistics.next_idx()
    }

    pub(crate) fn range_from(&self, start: StatisticsId) -> IdxRange<Statistics> {
        self.statistics.range_from(start)
    }

    pub(crate) fn set_schema(&mut self, id: StatisticsId, schema: Name) -> bool {
        self.statistics.set_schema(id, schema)
    }

    pub(crate) fn rename_schema(&mut self, from: Name, to: Name) {
        self.statistics.rename_schema(from, to);
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.statistics.shrink_to_fit();
        self.by_table.shrink_to_fit();
    }
}

impl Index<StatisticsId> for StatisticsIndex {
    type Output = Statistics;

    fn index(&self, id: StatisticsId) -> &Statistics {
        &self.statistics[id]
    }
}
