//! # math_trait
//!
//! Ein Trait-System für algebraische (und beliebige andere) Strukturen,
//! **unabhängig von konkreten Basisdatentypen**: dieses Crate implementiert
//! nichts für `i32`, `f64` & Co. – das kann ein Aufsatz-Crate tun.
//!
//! ## Aufbau
//!
//! ```text
//! Operation          Marker-Typ, der eine Verknüpfung benennt (Additive, Multiplicative, ...)
//!    │
//! BinaryOp<Op, Rhs>  allgemeinste Verknüpfung:  Self × Rhs → Output   (nicht abgeschlossen)
//!    │
//! Magma<Op>          abgeschlossene Verknüpfung: Self × Self → Self
//!    │
//!    └── (später) Semigroup, Monoid, Group, ... bauen auf Magma auf
//! ```
//!
//! Die Operation ist ein **Typparameter** des Traits und nicht Teil des
//! Typs selbst. Dadurch kann ein Typ dieselbe Struktur bezüglich mehrerer
//! Operationen haben (z. B. `Magma<Additive>` *und* `Magma<Multiplicative>`),
//! was für Ringe und Körper nötig ist.

#![no_std]
#![forbid(unsafe_code)]

mod binary_op;
mod magma;
mod operation;

pub use binary_op::BinaryOp;
pub use magma::Magma;
pub use operation::{Additive, Multiplicative, Operation};
