## Status

The project is currently in the development phase and has not yet reached a point where a fully
functional concept can be presented. The minimal viable product is currently being implemented, and
it is important to note that the project is still evolving and subject to change as many aspects are
still in the process of being defined and discovered.

## Styling

> TODO: Move styling to a distinct file as the README expands.

#### Linting rules are defined in the `main.rs`.

#### Max line width

is restricted to 100 characters to ensure optimal readability across diverse machines.

#### This project employs a two-space indentation for several reasons:

- Two spaces remain legible.
- They require less space than four spaces, thus reducing the overall file sizes.
- They do not display differently depending on the environment, in contrast to tabs.

#### Imports (keyword `use`):

- **Imports are placed subsequently to the submodules** (keyword `mod`), and not prior to.
  - It is of the utmost importance to distinguish between imports and the module system. Therefore,
    it is essential to place them in the appropriate locations to avoid any mixing.
- It is important to note that when importing, the `core` and `std` **namespaces** must not be
  separated by any spacing. Furthermore, any **modules** that have not been qualified by a parent
  **namespace** (such as `core`, `std`, `crate`, `super`, or `self`) must be imported in the same
  manner. This also applies to the `crate`, `super`, and `self` namespaces.
- **The ordering of the imports** is straightforward. The standard library is the first item to be
  imported, followed by any modules (external or internal), and then project namespaces (`crate`,
  `super`, `self`).
- **Namespace ordering** is also a factor in determining the precedence. In the standard library,
  the namespace `core` is placed before `std`, given that `std` is dependent on `core.` In the
  project itself, the namespace `crate` represents the highest level of organization and thus
  precedes all others. Following `crate`, the `super` namespace, which represents a superior level
  of organization, and then the `self` namespace.
- The uppermost namespace **should never duplicate**.

The illustration of the rules above:

```rs
mod submodule;
mod other_submodule;
// -- SPACING --
use core::{core_module, other_core_module};
use std::{std_module::std_func, other_std_module};
use std::std_module; // Not allowed.
// -- SPACING --
use external_dependency::{external_module};
use submodule::{some_fn};
// Or
use submodule::{some_fn};
use external_dependency::{external_module};
// -- SPACING --
use crate::{uppermost_level_import};
use super::{super_import};
use self::{submodule}; // Not required, as `submodule` is allowed to be imported directly.
```

The result of removing the documentary comments is as follows:

```rs
mod submodule;
mod other_submodule;

use core::{core_module, other_core_module};
use std::{std_module, other_std_module};

use external_dependency::{external_module};
use submodule::{some_fn};

use crate::{uppermost_level_import};
use super::{super_import};
use self::{submodule};
```

#### Exports

- A consensus has yet to be reached on the precise definition.
