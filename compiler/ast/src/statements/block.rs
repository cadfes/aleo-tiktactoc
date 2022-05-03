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

use crate::{Node, Statement, Type};
use leo_errors::Result;
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A block `{ [stmt]* }` consisting of a list of statements to execute in order.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Block {
    /// The list of statements to execute.
    pub statements: Vec<Statement>,
    /// The span from `{` to `}`.
    pub span: Span,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        if self.statements.is_empty() {
            writeln!(f, "\t")?;
        } else {
            self.statements
                .iter()
                .try_for_each(|statement| writeln!(f, "\t{}", statement))?;
        }
        write!(f, "}}")
    }
}

impl Node for Block {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    fn get_type(&self) -> Result<Option<Type>> {
        Ok(None)
    }
}
