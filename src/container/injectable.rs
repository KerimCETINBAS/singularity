
/// Marks a type as constructible via DI.
/// Must be implemented manually per service.
///
/// Safety: Any recursive dependency will result in **compile-time failure**.
pub trait Injectable: Sized {
    type Deps;
    const SCOPE: super::scope::Scope = super::scope::Scope::Scoped;
    fn inject(deps: Self::Deps) -> Self;
}


/// Macro for defining DI-ready structs with auto-generated `Injectable` implementations.
/// (full docs below)
#[macro_export]
macro_rules! injectable {
    // Unit struct — `injectable!(() => <vis>? <Name>)`
    (() => $vis:vis $name:ident) => {
        #[derive(Copy, Clone)]
        $vis struct $name;

        impl Injectable for $name {
            type Deps = ();
            #[inline(always)]
            fn inject(_: Self::Deps) -> Self {
                Self
            }
        }
    };
    // endregion



    // Named struct, no dependencies —
    // `injectable!(() => <vis>? <Name> { <field>: <Type> = <expr>, ... })`
    (() => $vis:vis $name:ident  {
        $( $field:ident: $field_type:ty = $field_expr:expr ),* $(,)?
    }) => {
        $vis struct $name {
            $($field: $field_type),*
        }

        impl Injectable for $name {
            type Deps = ();
            #[inline(always)]
            fn inject(_: Self::Deps) -> Self {
                Self {
                    $($field: $field_expr,)*
                }
            }
        }
    };


    // Tuple struct, no dependencies —
    // `injectable!(() => <vis>? <Name>(<Type> = <expr>, ...))`
    (
        () => $vis:vis $name:ident  (
            $( $field_type:ty = $field_expr:expr ),*  $(,)?
        )
    ) => {
        $vis struct $name ($($field_type),*);

        impl Injectable for $name {
            type Deps = ();
            #[inline(always)]
            fn inject(_: Self::Deps) -> Self {
                Self ($($field_expr),*)
            }
        }
    };

    // Named struct, one dependency —
    // `injectable!((dep: Type) => <vis>? <Name> { <field>: <Type> = <expr>, ... })`
    (
        ($param_name:ident : $param_type:ty) => $vis:vis $name:ident {
            $( $field_name:ident: $field_type:ty = $field_expr:expr),*  $(,)?
        }
    ) => {
        $vis struct $name {
            $param_name : $param_type,
            $( $field_name : $field_type ),*
        }

        impl Injectable for $name {
            type Deps = $param_type;
            #[inline(always)]
            fn inject($param_name: Self::Deps) -> Self {
                Self {
                    $param_name,
                    $( $field_name: $field_expr ),*
                }
            }
        }
    };

    // Tuple struct, one dependency —
    // `injectable!((dep: Type) => <vis>? <Name>(<Type> = <expr>, ...))`
    (
        ($param_name:ident : $param_type:ty ) => $vis:vis $name:ident ($( $field_type:ty = $field_expr:expr ),* $(,)?)
    ) => {
        $vis struct $name ($param_type, $($field_type),*);

        impl Injectable for $name {
            type Deps = $param_type;
            #[inline(always)]
            fn inject(deps: Self::Deps) -> Self {
                Self (deps, $($field_expr),*)
            }
        }
    };

    // Named struct, multiple dependencies —
    // `injectable!((a:A, b:B, ...) => <vis>? <Name> { <field>: <Type> = <expr>, ... })`
    (
       ( $f_param:ident : $f_type:ty , $( $r_param:ident : $r_type:ty),+ $(,)? ) => $vis:vis $name:ident {
           $( $field_name:ident: $field_type:ty = $field_expr:expr),* $(,)?
       }
    ) => {
        $vis struct $name {
            $f_param: $f_type,
            $($r_param: $r_type, ),+
            $($field_name: $field_type,)*
        }

        impl Injectable for $name {
            type Deps =  ($f_type, $($r_type),+);
            #[inline(always)]
            fn inject(($f_param, $($r_param),+): Self::Deps) -> Self {
                Self { $f_param, $($r_param),+ , $($field_name: $field_expr),* }
            }
        }
    };


    // Tuple struct, multiple dependencies —
    // `injectable!((a:A, b:B, ...) => <vis>? <Name>(<Type> = <expr>, ...))`
    (
        ( $f_param:ident : $f_param_type:ty, $( $r_param:ident : $r_param_type:ty ),+ ) =>
            $vis:vis $name:ident (
                $( $field_type:ty = $field_expr:expr ),* $(,)?
            )
    ) => {
        $vis struct $name (
            $f_param_type,
            $( $r_param_type ),+,
            $( $field_type ),*
        );

        impl Injectable for $name {
            type Deps = ($f_param_type, $( $r_param_type ),+);

            #[inline(always)]
            fn inject(($f_param, $($r_param),+): Self::Deps) -> Self {

                Self(
                    $f_param,
                    $($r_param),+,
                    $( $field_expr ),*)
            }
        }
    };

}


pub use injectable;
#[cfg(test)]
mod injectable_trait_test;


