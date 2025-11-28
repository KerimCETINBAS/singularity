use syn::{FieldsNamed, FieldsUnnamed};

pub (crate) enum StructKind<'a> {
    Named(&'a FieldsNamed),
    Unnamed(&'a FieldsUnnamed),
    Unit
}