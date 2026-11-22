// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/event.rs
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! This module provides a way to construct a `File`.
//! It is intended to be completely decoupled from the
//! parser, so as to allow to evolve the tree representation
//! and the parser algorithm independently.

use std::mem;
use std::num::NonZeroU32;

use crate::{output::Output, syntax_kind::SyntaxKind};

/// `Parser` produces a flat list of `Event`s.
/// They are converted to a tree-structure in
/// a separate pass, via `SyntaxTreeBuilder`.
///
/// A start event signifies the start of a node.
/// It should be either abandoned (in which case the
/// `kind` is `TOMBSTONE`, and the event is ignored),
/// or completed via a `Finish` event.
///
/// All tokens between a `Start` and a `Finish` would
/// become the children of the respective node.
///
/// For left-recursive syntactic constructs, the parser produces
/// a child node before it sees a parent. `forward_parent`
/// saves the position of current event's parent.
///
/// Consider this path
///
/// `foo::bar`
///
/// The events for it would look like this:
///
/// ```text
/// START(PATH) IDENT('foo') FINISH START(PATH) T![::] IDENT('bar') FINISH
///       |                          /\
///       |                          |
///       +------forward-parent------+
/// ```
///
/// And the tree would look like this
///
/// ```text
///    +--PATH---------+
///    |   |           |
///    |   |           |
///    |  '::'       'bar'
///    |
///   PATH
///    |
///   'foo'
/// ```
///
/// See also `CompletedMarker::precede`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Event(u64);

impl Event {
    pub(crate) const TOMBSTONE: Event = Event::new(Output::enter_event(SyntaxKind::TOMBSTONE));

    const fn new(output: u32) -> Self {
        Event(output as u64)
    }

    pub(crate) const fn finish() -> Self {
        Self::new(Output::EXIT)
    }

    pub(crate) const fn token(kind: SyntaxKind, n_raw_tokens: u8) -> Self {
        Self::new(Output::token_event(kind, n_raw_tokens))
    }

    pub(crate) fn error(index: u32) -> Self {
        Self::new(Output::error_event(index))
    }

    #[allow(clippy::cast_possible_truncation)]
    fn output(self) -> u32 {
        self.0 as u32
    }

    fn forward_parent(self) -> Option<NonZeroU32> {
        NonZeroU32::new((self.0 >> 32) as u32)
    }

    fn is_start(self) -> bool {
        Output::is_enter_event(self.output())
    }

    fn start_kind(self) -> SyntaxKind {
        debug_assert!(self.is_start());
        Output::event_kind(self.output()).into()
    }

    pub(crate) fn complete_start(&mut self, kind: SyntaxKind) {
        debug_assert!(self.is_start());
        *self = Event(u64::from(Output::enter_event(kind)) | self.0 & !u64::from(u32::MAX));
    }

    pub(crate) fn set_forward_parent(&mut self, forward_parent: NonZeroU32) {
        debug_assert!(self.is_start());
        *self = Event(u64::from(self.output()) | u64::from(forward_parent.get()) << 32);
    }
}

/// Generate the syntax tree with the control of events.
pub(super) fn process(mut events: Vec<Event>, errors: Vec<String>, node_count: usize) -> Output {
    let mut res = vec![0; events.len()];
    let mut len = 0;
    let mut forward_parents = vec![];

    for i in 0..events.len() {
        let event = events[i];
        let Some(forward_parent) = event.forward_parent() else {
            res[len] = event.output();
            len += usize::from(event != Event::TOMBSTONE);
            continue;
        };
        // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
        // in the normal control flow, the parent-child relation: `A -> B -> C`,
        // while with the magic forward_parent, it writes: `C <- B <- A`.

        // append `A` into parents.
        forward_parents.push(event.start_kind());
        let mut idx = i;
        let mut fp = Some(forward_parent);
        while let Some(fwd) = fp {
            idx += fwd.get() as usize;
            // append `A`'s forward_parent `B`
            let parent = mem::replace(&mut events[idx], Event::TOMBSTONE);
            forward_parents.push(parent.start_kind());
            fp = parent.forward_parent();
            // append `B`'s forward_parent `C` in the next stage.
        }

        for kind in forward_parents.drain(..).rev() {
            if kind != SyntaxKind::TOMBSTONE {
                res[len] = Output::enter_event(kind);
                len += 1;
            }
        }
    }

    res.truncate(len);
    debug_assert_eq!(
        res.iter()
            .filter(|&&event| Output::is_enter_event(event))
            .count(),
        node_count
    );
    Output::new(res, errors, node_count)
}
