// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Identifier, Node, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An access expression to an circuit constant., e.g. `u8::MAX`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssociatedConstant {
    /// The inner circuit type.
    pub ty: Type,
    /// The circuit constant that is being accessed.
    pub name: Identifier,
    /// The span for the entire expression `Foo::bar()`.
    pub span: Span,
}

impl fmt::Display for AssociatedConstant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.ty, self.name)
    }
}

crate::simple_node_impl!(AssociatedConstant);
