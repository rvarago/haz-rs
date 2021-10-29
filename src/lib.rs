//! A thin abstraction over polymorphic environments.
//!
//! We might want to a polymorphic when:
//!
//! - we want access to a set of types smaller than what a concrete environment may expose, or
//! - we do not care about any concrete environment as long as it exposes all required types
//!
//! # Example
//!
//! A procedure responsible for spawning a TCP server from a config might only require *some* type,
//! from which it can retrieve host and port without looking at anything else.
//!
//! ```
//! use haz::{Has, access_from};
//!
//! struct Config {
//!   host: Host,
//!   port: Port,
//!   abort_on_error: bool,
//!   debug: bool,
//! }
//!
//! #[derive(Debug)]
//! struct Host(String);
//!
//! #[derive(Debug)]
//! struct Port(u16);
//!
//! impl Has<Host> for Config {
//!   fn access(&self) -> &Host {
//!     &self.host
//!   }
//! }
//!
//! impl Has<Port> for Config {
//!   fn access(&self) -> &Port {
//!     &self.port
//!   }
//! }
//!
//!
//! fn run_with<C>(cfg: &C)
//! where C: Has<Host> + Has<Port> {
//!   let host: &Host = cfg.access();
//!   let port = access_from::<Port, _>(cfg);
//!
//!   println!("host: {:?}, port: {:?}", host, port);
//! }
//! ```

#![deny(missing_docs)]

/// A representation of a type which can give access to some `Component`.
pub trait Has<Component> {
    /// Borrows read-only access to a component of the parent container.
    fn access(&self) -> &Component;
}

/// Accesses a component from its container via a turbofish-friendly syntax.
pub fn access_from<Component, Container>(container: &Container) -> &Component
where
    Container: Has<Component>,
{
    container.access()
}

/// Helper to give access to a component via a turbofish-friendly, infix syntax.
#[derive(Debug)]
pub struct Accessor<Component>(std::marker::PhantomData<Component>);

impl<Component> Accessor<Component> {
    /// Accesses a component from its container.
    ///
    /// This function simply delegates to the trait's method, but it might be
    /// interesting for those who prefer turbofish to annotate types combined with an infix notation.
    pub fn from<'c, Container>(&self, container: &'c Container) -> &'c Component
    where
        Container: Has<Component>,
    {
        container.access()
    }
}

/// Constructs a proxy from which one may access a component from its container via a turbofish-friendly, infix syntax.
pub fn access<Component>() -> Accessor<Component> {
    Accessor(std::marker::PhantomData)
}

/// Implements [`Has`] for a container which can give access to a component.
///
/// # Example
///
/// ```
/// struct Env {
///   host: Host,
/// }
///
/// struct Host(String);
///
/// haz::impl_has_for_named_component!(Env, Host, host);
/// ```
#[macro_export]
macro_rules! impl_has_for_named_component {
    ($container_type:ty, $component_type:ty, $component_name:ident) => {
        impl haz::Has<$component_type> for $container_type {
            fn access(&self) -> &$component_type {
                &self.$component_name
            }
        }
    };
}
