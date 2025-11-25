


/// A general contract for resolving dependency tuples.
/// Implemented up to 8 levels manually for performance and control.
///
/// Recursive resolution will emit a compile-time error instead of runtime failure.
pub trait ResolveDepsFrom<C>: Sized {
    fn resolve_deps(container: &C) -> Self;
}


/// Base case: service has no dependencies.
impl ResolveDepsFrom<super::Container> for () {
    #[inline(always)]
    fn resolve_deps(_: &super::Container) -> Self {
        ()
    }
}

/// Automatically resolves a single dependency.
impl<A> ResolveDepsFrom<super::Container> for A
where
    A: super::Injectable,
    A::Deps: ResolveDepsFrom<super::Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &super::Container) -> Self {
        container.resolve::<A>()
    }
}




macro_rules! resolve_deps_from {
    (
      $( $T:ident),+
    ) => {
        impl<$($T),+> ResolveDepsFrom<super::Container> for ($($T),+)
            where
                $($T: super::Injectable),+,
                $($T::Deps:  ResolveDepsFrom<super::Container>),+
        {
            #[inline(always)]
            fn resolve_deps(container: &super::Container) -> Self {
                ($(container.resolve::<$T>()),+)
            }
        }
    };
}



// ResolveDepsFrom tuple arity up to 16
resolve_deps_from!(A, B);
resolve_deps_from!(A, B, C);
resolve_deps_from!(A, B, C, D);
resolve_deps_from!(A, B, C, D, E);
resolve_deps_from!(A, B, C, D, E, F);
resolve_deps_from!(A, B, C, D, E, F, G);
resolve_deps_from!(A, B, C, D, E, F, G, H);
resolve_deps_from!(A, B, C, D, E, F, G, H, I);
resolve_deps_from!(A, B, C, D, E, F, G, H, I, J);
resolve_deps_from!(A, B, C, D, E, F, G, H, I, J, K);
resolve_deps_from!(A, B, C, D, E, F, G, H, I, J, K, L);
resolve_deps_from!(A, B, C, D, E, F, G, H, I, J, K, L, M);
resolve_deps_from!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
resolve_deps_from!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
resolve_deps_from!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);