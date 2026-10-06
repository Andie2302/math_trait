//! Operations-Tags.

/// Marker-Trait für Typen, die eine Verknüpfung *benennen*.
///
/// Ein Tag trägt keine Daten; er unterscheidet nur, *welche* Verknüpfung eines
/// Typs gemeint ist. Eigene Tags sind beliebig möglich:
///
/// ```
/// use math_trait::Operation;
///
/// struct Concat;
/// impl Operation for Concat {}
/// ```
pub trait Operation {}

/// Tag für die „additive“ Verknüpfung (Konvention, keine Eigenschaft).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Additive;

/// Tag für die „multiplikative“ Verknüpfung (Konvention, keine Eigenschaft).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Multiplicative;

impl Operation for Additive {}
impl Operation for Multiplicative {}
