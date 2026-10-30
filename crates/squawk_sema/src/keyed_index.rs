use std::{fmt::Debug, hash::Hash, ops::Index};

use la_arena::Idx;
use smallvec::SmallVec;

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::interner::Name;
use crate::ir::{
    AccessMethod, Cast, CastTypes, Database, EventTrigger, ForeignDataWrapper, GraphElement,
    GraphElementId, GraphLabel, GraphLabelId, GraphProperty, Language, MemberOwner,
    OperatorFamilyMember, OperatorFamilyMemberId, Policy, Publication, Role, Rule, Schema, Server,
    Subscription, TableId, Tablespace, Trigger,
};

pub trait Keyed {
    type Key: Copy + Eq + Hash + Debug + 'static;

    fn key(&self) -> Self::Key;
}

#[derive(Debug)]
pub struct KeyedIndex<T: Keyed + 'static> {
    arena: LayeredArena<T>,
    by_key: LayeredMultimap<T::Key, Idx<T>>,
}

impl<T: Keyed> Default for KeyedIndex<T> {
    fn default() -> Self {
        KeyedIndex {
            arena: LayeredArena::default(),
            by_key: LayeredMultimap::default(),
        }
    }
}

impl<T: Keyed> KeyedIndex<T> {
    pub(crate) fn on_top_of(base: &'static KeyedIndex<T>) -> Self {
        KeyedIndex {
            arena: LayeredArena::on_top_of(&base.arena),
            by_key: LayeredMultimap::on_top_of(&base.by_key),
        }
    }

    pub(crate) fn alloc(&mut self, value: T) -> Idx<T> {
        let key = value.key();
        let id = self.arena.alloc(value);
        self.by_key.insert(key, id);
        id
    }

    pub fn get(&self, key: T::Key) -> impl DoubleEndedIterator<Item = Idx<T>> + '_ {
        self.by_key.get(key)
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (Idx<T>, &T)> {
        self.arena.iter()
    }

    pub(crate) fn next_idx(&self) -> Idx<T> {
        self.arena.next_idx()
    }

    pub(crate) fn range_from(&self, start: Idx<T>) -> IdxRange<T> {
        self.arena.range_from(start)
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.arena.shrink_to_fit();
        self.by_key.shrink_to_fit();
    }
}

impl<T: Keyed> Index<Idx<T>> for KeyedIndex<T> {
    type Output = T;

    fn index(&self, id: Idx<T>) -> &T {
        &self.arena[id]
    }
}

impl KeyedIndex<GraphElement> {
    pub(crate) fn set_endpoints(
        &mut self,
        id: GraphElementId,
        source: Option<GraphElementId>,
        destination: Option<GraphElementId>,
    ) {
        let element = &mut self.arena[id];
        element.source = source;
        element.destination = destination;
    }
}

impl KeyedIndex<OperatorFamilyMember> {
    pub(crate) fn move_owned(
        &mut self,
        from: MemberOwner,
        to: MemberOwner,
    ) -> SmallVec<[OperatorFamilyMemberId; 1]> {
        let ids = self.by_key.take(from);
        for &id in &ids {
            self.arena[id].owner = to;
            self.by_key.insert(to, id);
        }
        ids
    }

    pub(crate) fn set_owner(&mut self, id: OperatorFamilyMemberId, owner: MemberOwner) -> bool {
        let Some(member) = self.arena.get_local_mut(id) else {
            return false;
        };
        let previous = std::mem::replace(&mut member.owner, owner);
        if previous != owner {
            self.by_key.remove(previous, id);
            self.by_key.insert(owner, id);
        }
        true
    }
}

pub(crate) type AccessMethodIndex = KeyedIndex<AccessMethod>;
pub(crate) type CastIndex = KeyedIndex<Cast>;
pub(crate) type DatabaseIndex = KeyedIndex<Database>;
pub(crate) type EventTriggerIndex = KeyedIndex<EventTrigger>;
pub(crate) type ForeignDataWrapperIndex = KeyedIndex<ForeignDataWrapper>;
pub(crate) type GraphElementIndex = KeyedIndex<GraphElement>;
pub(crate) type GraphLabelIndex = KeyedIndex<GraphLabel>;
pub(crate) type GraphPropertyIndex = KeyedIndex<GraphProperty>;
pub(crate) type LanguageIndex = KeyedIndex<Language>;
pub(crate) type OperatorFamilyMemberIndex = KeyedIndex<OperatorFamilyMember>;
pub(crate) type PolicyIndex = KeyedIndex<Policy>;
pub(crate) type PublicationIndex = KeyedIndex<Publication>;
pub(crate) type RoleIndex = KeyedIndex<Role>;
pub(crate) type RuleIndex = KeyedIndex<Rule>;
pub(crate) type SchemaIndex = KeyedIndex<Schema>;
pub(crate) type ServerIndex = KeyedIndex<Server>;
pub(crate) type SubscriptionIndex = KeyedIndex<Subscription>;
pub(crate) type TablespaceIndex = KeyedIndex<Tablespace>;
pub(crate) type TriggerIndex = KeyedIndex<Trigger>;

macro_rules! keyed_by_name {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Keyed for $ty {
                type Key = Name;

                fn key(&self) -> Name {
                    self.name
                }
            }
        )*
    };
}

keyed_by_name!(
    AccessMethod,
    Database,
    EventTrigger,
    ForeignDataWrapper,
    Language,
    Publication,
    Role,
    Schema,
    Server,
    Subscription,
    Tablespace,
);

macro_rules! keyed_by_table {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Keyed for $ty {
                type Key = TableId;

                fn key(&self) -> TableId {
                    self.table
                }
            }
        )*
    };
}

keyed_by_table!(Policy, Rule, Trigger);

impl Keyed for Cast {
    type Key = CastTypes;

    fn key(&self) -> CastTypes {
        CastTypes {
            source: self.source,
            target: self.target,
        }
    }
}

impl Keyed for GraphElement {
    type Key = TableId;

    fn key(&self) -> TableId {
        self.graph
    }
}

impl Keyed for GraphLabel {
    type Key = GraphElementId;

    fn key(&self) -> GraphElementId {
        self.element
    }
}

impl Keyed for GraphProperty {
    type Key = GraphLabelId;

    fn key(&self) -> GraphLabelId {
        self.label
    }
}

impl Keyed for OperatorFamilyMember {
    type Key = MemberOwner;

    fn key(&self) -> MemberOwner {
        self.owner
    }
}
