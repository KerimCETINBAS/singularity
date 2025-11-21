/// # Singularity Dependency Resolver 🪓
///
/// A zero-cost compile-time dependency resolver that avoids:
/// - Service Locators ❌
/// - Runtime Reflection ❌
/// - Container lifecycle complexity ❌
///
/// Instead, it resolves dependencies **statically**, via type constraints.
///
/// ## Key Principles
/// - **Bushcraft philosophy** – use only what's already available
/// - **No runtime resolution logic**
/// - **Constructor-based dependency flow**
/// - **Circular dependencies caught at compile time**
/// - Supports up to **8 dependency parameters**
///
/// ## Example
/// ```rust
/// struct A;
/// impl Injectable for A {
///     type Deps = ();
///     fn inject(_: ()) -> Self { A }
/// }
///
/// struct B(A);
/// impl Injectable for B {
///     type Deps = A;
///     fn inject(a: A) -> Self { B(a) }
/// }
///
/// struct Test;
/// impl Invokable for Test {
///     type Deps = (A, B);
///     type Output = ();
///
///     fn invoke_with((a, b): Self::Deps, _: impl FnOnce(Self::Output)) {
///         println!("Resolved: {:?} {:?}", core::any::type_name::<A>(), core::any::type_name::<B>());
///     }
/// }
///
/// let c = Container::new();
/// c.invoke::<Test>();
/// ```
pub struct Container {
    /// Prevents direct struct initialization via `Container {}` or `Container;`
    /// Enforces usage via `Container::new()`
    _private: (),
}

impl Container {
    /// Creates a new DI container instance.
    /// Container is intentionally ephemeral – lifecycle management must be external.
    pub fn new() -> Self {
        Container { _private: () }
    }

    /// Resolves a service of type `T` using its associated `Injectable::Deps`.
    /// Resolution should occur through `Mediator`, `invoke()`, or higher-level abstractions.
    #[inline(always)]
    pub fn resolve<T>(&self) -> T
    where
        T: Injectable,
        T::Deps: ResolveDepsFrom<Self>,
    {
        T::inject(T::Deps::resolve_deps(self))
    }

    /// Invokes an `Invokable` type without a callback.
    /// Useful for commands that do not return a value.
    pub fn invoke<T>(&self)
    where
        T: Invokable,
        T::Deps: ResolveDepsFrom<Self>,
    {
        let deps = T::Deps::resolve_deps(self);
        T::invoke(deps);
    }

    /// Invokes an `Invokable` type and optionally captures its `Output`.
    /// This is ideal for commands that return scoped data or must be pipelined.
    ///
    /// ```
    /// container.invoke_with::<ComputeSum>(|result| println!("Result = {}", result));
    /// ```
    pub fn invoke_with<T>(&self, callback: impl FnOnce(T::Output))
    where
        T: Invokable,
        T::Deps: ResolveDepsFrom<Self>,
    {
        let deps = T::Deps::resolve_deps(self);
        T::invoke_with(deps, callback);
    }
}

/// A stateless execution contract.
///
/// - `Deps` is auto-resolved by the container.
/// - `Output` is optional; use `invoke()` for fire-and-forget.
/// - `invoke_with()` enables value extraction without persistence.
///
/// Always prefer using `invoke()` unless you need the callback.
pub trait Invokable {
    /// Type describing resolved dependencies.
    type Deps;
    /// Value returned by execution.
    type Output;

    /// Executes and returns `Output` via a callback.
    fn invoke_with<F>(deps: Self::Deps, callback: F)
    where
        F: FnOnce(Self::Output);

    /// Fire-and-forget version of `invoke_with()`.
    /// Callback is suppressed using `no-op` closure.
    #[inline(always)]
    fn invoke(deps: Self::Deps) {
        Self::invoke_with(deps, |_| {});
    }
}

/// Marks a type as constructible via DI.
/// Must be implemented manually per service.
///
/// Safety: Any recursive dependency will result in **compile-time failure**.
pub trait Injectable: Sized {
    type Deps;
    fn inject(deps: Self::Deps) -> Self;
}

/// A general contract for resolving dependency tuples.
/// Implemented up to 8 levels manually for performance and control.
///
/// Recursive resolution will emit a compile-time error instead of runtime failure.
pub trait ResolveDepsFrom<C>: Sized {
    fn resolve_deps(container: &C) -> Self;
}

//
// ---------------------
// Dependency Arity Core
// ---------------------
//

/// Base case: service has no dependencies.
impl ResolveDepsFrom<Container> for () {
    #[inline(always)]
    fn resolve_deps(_: &Container) -> Self {
        ()
    }
}

/// Automatically resolves a single dependency.
impl<A> ResolveDepsFrom<Container> for A
where
    A: Injectable,
    A::Deps: ResolveDepsFrom<Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &Container) -> Self {
        container.resolve::<A>()
    }
}

// ⬇ Arity 2–8 tuple resolution.
// Each depends on DI capability of contained types.

impl<A, B> ResolveDepsFrom<Container> for (A, B)
where
    A: Injectable,
    B: Injectable,
    A::Deps: ResolveDepsFrom<Container>,
    B::Deps: ResolveDepsFrom<Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &Container) -> Self {
        (container.resolve::<A>(), container.resolve::<B>())
    }
}

impl<A, B, C> ResolveDepsFrom<Container> for (A, B, C)
where
    A: Injectable,
    B: Injectable,
    C: Injectable,
    A::Deps: ResolveDepsFrom<Container>,
    B::Deps: ResolveDepsFrom<Container>,
    C::Deps: ResolveDepsFrom<Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &Container) -> Self {
        (container.resolve::<A>(), container.resolve::<B>(), container.resolve::<C>())
    }
}

impl<A, B, C, D> ResolveDepsFrom<Container> for (A, B, C, D)
where
    A: Injectable,
    B: Injectable,
    C: Injectable,
    D: Injectable,
    A::Deps: ResolveDepsFrom<Container>,
    B::Deps: ResolveDepsFrom<Container>,
    C::Deps: ResolveDepsFrom<Container>,
    D::Deps: ResolveDepsFrom<Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &Container) -> Self {
        (
            container.resolve::<A>(),
            container.resolve::<B>(),
            container.resolve::<C>(),
            container.resolve::<D>(),
        )
    }
}

impl<A, B, C, D, E> ResolveDepsFrom<Container> for (A, B, C, D, E)
where
    A: Injectable,
    B: Injectable,
    C: Injectable,
    D: Injectable,
    E: Injectable,
    A::Deps: ResolveDepsFrom<Container>,
    B::Deps: ResolveDepsFrom<Container>,
    C::Deps: ResolveDepsFrom<Container>,
    D::Deps: ResolveDepsFrom<Container>,
    E::Deps: ResolveDepsFrom<Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &Container) -> Self {
        (
            container.resolve::<A>(),
            container.resolve::<B>(),
            container.resolve::<C>(),
            container.resolve::<D>(),
            container.resolve::<E>(),
        )
    }
}

impl<A, B, C, D, E, F> ResolveDepsFrom<Container> for (A, B, C, D, E, F)
where
    A: Injectable,
    B: Injectable,
    C: Injectable,
    D: Injectable,
    E: Injectable,
    F: Injectable,
    A::Deps: ResolveDepsFrom<Container>,
    B::Deps: ResolveDepsFrom<Container>,
    C::Deps: ResolveDepsFrom<Container>,
    D::Deps: ResolveDepsFrom<Container>,
    E::Deps: ResolveDepsFrom<Container>,
    F::Deps: ResolveDepsFrom<Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &Container) -> Self {
        (
            container.resolve::<A>(),
            container.resolve::<B>(),
            container.resolve::<C>(),
            container.resolve::<D>(),
            container.resolve::<E>(),
            container.resolve::<F>(),
        )
    }
}

impl<A, B, C, D, E, F, G> ResolveDepsFrom<Container> for (A, B, C, D, E, F, G)
where
    A: Injectable,
    B: Injectable,
    C: Injectable,
    D: Injectable,
    E: Injectable,
    F: Injectable,
    G: Injectable,
    A::Deps: ResolveDepsFrom<Container>,
    B::Deps: ResolveDepsFrom<Container>,
    C::Deps: ResolveDepsFrom<Container>,
    D::Deps: ResolveDepsFrom<Container>,
    E::Deps: ResolveDepsFrom<Container>,
    F::Deps: ResolveDepsFrom<Container>,
    G::Deps: ResolveDepsFrom<Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &Container) -> Self {
        (
            container.resolve::<A>(),
            container.resolve::<B>(),
            container.resolve::<C>(),
            container.resolve::<D>(),
            container.resolve::<E>(),
            container.resolve::<F>(),
            container.resolve::<G>(),
        )
    }
}

impl<A, B, C, D, E, F, G, H> ResolveDepsFrom<Container> for (A, B, C, D, E, F, G, H)
where
    A: Injectable,
    B: Injectable,
    C: Injectable,
    D: Injectable,
    E: Injectable,
    F: Injectable,
    G: Injectable,
    H: Injectable,
    A::Deps: ResolveDepsFrom<Container>,
    B::Deps: ResolveDepsFrom<Container>,
    C::Deps: ResolveDepsFrom<Container>,
    D::Deps: ResolveDepsFrom<Container>,
    E::Deps: ResolveDepsFrom<Container>,
    F::Deps: ResolveDepsFrom<Container>,
    G::Deps: ResolveDepsFrom<Container>,
    H::Deps: ResolveDepsFrom<Container>,
{
    #[inline(always)]
    fn resolve_deps(container: &Container) -> Self {
        (
            container.resolve::<A>(),
            container.resolve::<B>(),
            container.resolve::<C>(),
            container.resolve::<D>(),
            container.resolve::<E>(),
            container.resolve::<F>(),
            container.resolve::<G>(),
            container.resolve::<H>(),
        )
    }
}
