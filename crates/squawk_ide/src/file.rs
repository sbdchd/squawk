// via https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/hir-expand/src/files.rs#L21C21-L21C21
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

pub use crate::db::FileId;
use crate::ir::BodyId;
use rowan::{TextRange, TextSize};
use squawk_syntax::{AstPtr, SyntaxNodePtr, ast::AstNode};

/// `InFile<T>` stores a value of `T` inside a particular file/syntax tree.
///
/// Typical usages are:
///
/// * `InFile<SyntaxNode>` -- syntax node in a file
/// * `InFile<ast::FnDef>` -- ast node in a file
/// * `InFile<TextSize>` -- offset in a file
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InFileWrapper<FileKind, T> {
    pub file_id: FileKind,
    pub value: T,
}

impl<FileKind, T> InFileWrapper<FileKind, T> {
    pub fn new(file_id: impl Into<FileKind>, value: T) -> Self {
        Self {
            file_id: file_id.into(),
            value,
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> InFileWrapper<FileKind, U> {
        InFileWrapper {
            file_id: self.file_id,
            value: f(self.value),
        }
    }
}

impl<FileKind: Copy, T> InFileWrapper<FileKind, T> {
    pub fn with_value<U>(&self, value: U) -> InFileWrapper<FileKind, U> {
        InFileWrapper {
            file_id: self.file_id,
            value,
        }
    }
}

pub type InFile<T> = InFileWrapper<FileId, T>;
pub type FileRange = InFile<TextRange>;
pub type FilePosition = InFile<TextSize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum IrFileId {
    File(FileId),
    Body(BodyId),
}

impl From<FileId> for IrFileId {
    fn from(file_id: FileId) -> Self {
        IrFileId::File(file_id)
    }
}

pub(crate) type InIrFile<T> = InFileWrapper<IrFileId, T>;
pub(crate) type IrFileRange = InIrFile<TextRange>;

impl<FileKind: Copy> InFileWrapper<FileKind, SyntaxNodePtr> {
    pub fn file_range(&self) -> InFileWrapper<FileKind, TextRange> {
        self.with_value(self.value.text_range())
    }
}

impl<FileKind: Copy, N: AstNode> InFileWrapper<FileKind, AstPtr<N>> {
    pub fn syntax_node_ptr(&self) -> InFileWrapper<FileKind, SyntaxNodePtr> {
        self.map(|ptr| ptr.syntax_node_ptr())
    }

    pub fn file_range(&self) -> InFileWrapper<FileKind, TextRange> {
        self.with_value(self.value.text_range())
    }
}
