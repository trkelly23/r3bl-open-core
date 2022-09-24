/*
 *   Copyright (c) 2022 R3BL LLC
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use std::fmt::{Debug, Display};

use crate::*;

/// Global scope struct.
pub struct GlobalScopeArgs<'a, S, A>
where
  S: Default + Display + Clone + PartialEq + Debug + Sync + Send,
  A: Default + Display + Clone + Sync + Send,
{
  pub shared_tw_data: &'a SharedTWData,
  pub shared_store: &'a SharedStore<S, A>,
  pub state: &'a S,
}

/// Component scope struct.
pub struct ComponentScopeArgs<'a, S, A>
where
  S: Default + Display + Clone + PartialEq + Debug + Sync + Send,
  A: Default + Display + Clone + Sync + Send,
{
  pub shared_tw_data: &'a SharedTWData,
  pub shared_store: &'a SharedStore<S, A>,
  pub state: &'a S,
  pub component_registry: &'a mut ComponentRegistry<S, A>,
}

/// [EditorEngine] common struct.
pub struct EditorEngineArgs<'a, S, A>
where
  S: Default + Display + Clone + PartialEq + Debug + Sync + Send,
  A: Default + Display + Clone + Sync + Send,
{
  pub shared_tw_data: &'a SharedTWData,
  pub shared_store: &'a SharedStore<S, A>,
  pub state: &'a S,
  pub component_registry: &'a mut ComponentRegistry<S, A>,
  pub self_id: &'a str,
  pub editor_buffer: &'a EditorBuffer,
}

// ╭┄┄┄┄┄┄┄┄┄╮
// │ Aliases │
// ╯         ╰┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄
pub type ScrollOffset = Position;
pub type Nope = Option<()>;
