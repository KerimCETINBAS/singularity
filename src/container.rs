
mod injectable;

mod invokable;
mod resolve_deps_from;
mod resolver;
mod scope;

pub use injectable::Injectable;

// pub use invokable::Invokable;

use resolve_deps_from::ResolveDepsFrom;
pub mod macros {
    pub use super::injectable::injectable as injectable;
}


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


pub struct Container {
    /// Prevents direct struct initialization via `Container {}` or `Container;`
    /// Enforces usage via `Container::new()`
    _private: (),
}

impl Container {

    pub fn new() -> Self {
        Container { _private: () }
    }

    #[inline(always)]
    pub fn resolve<T>(&self) -> T
    where
        T: Injectable,
        T::Deps: ResolveDepsFrom<Self>,
    {
        T::inject(T::Deps::resolve_deps(self))
    }

    // pub fn invoke<T>(&self)
    // where
    //     T: Invokable,
    //     <T as Invokable>::Deps: ResolveDepsFrom<Self>,
    // {
    //     let deps = <T as Invokable>::Deps::resolve_deps(self);
    //     T::invoke(deps);
    // }
    // 
    // 
    // pub fn invoke_with<T>(&self, callback: impl FnOnce(T::Output))
    // where
    //     T: Invokable,
    //     <T as Invokable>::Deps: ResolveDepsFrom<Self>,
    // {
    //     let deps = <T as Invokable>::Deps::resolve_deps(self);
    //     T::invoke_with(deps, callback);
    // }
}


