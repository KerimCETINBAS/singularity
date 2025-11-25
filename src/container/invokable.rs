


/// A stateless execution contract.
///
/// - `Deps` is auto-resolved by the container.
/// - `Output` is optional; use `invoke()` for fire-and-forget.
/// - `invoke_with()` enables value extraction without persistence.
///
/// Always prefer using `invoke()` unless you need the callback.
pub trait Invokable: super::Injectable {
    /// Type describing resolved dependencies.
    type Deps;
    /// Value returned by execution.
    type Output;

    const SCOPE: super::scope::Scope = super::scope::Scope::Scoped;
    fn inject(_: <Self as Invokable>::Deps)  -> Self {
        panic!("invokable inject not implemented");
    }

    /// Executes and returns `Output` via a callback.
    fn invoke_with<F>(deps: <Self as Invokable>::Deps, callback: F)
    where
        F: FnOnce(Self::Output);

    /// Fire-and-forget version of `invoke_with()`.
    /// Callback is suppressed using `no-op` closure.
    #[inline(always)]
    fn invoke(deps: <Self as Invokable>::Deps) {
        Self::invoke_with(deps, |_| {});
    }
}

impl<T> super::Injectable for T where T: Invokable
{
    type Deps = ();
    fn inject(_: <T as super::Injectable>::Deps) -> Self { panic!("invokable inject not implemented") }
}