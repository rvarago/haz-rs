#![allow(dead_code)]

use haz::{access, access_from, Has};

fn main() {
    let env = Env {
        host: Host("localhost".into()),
        port: Port(8080),
        verbosity: Verbosity::High,
        restriction: Restriction::Enforcing,
    };

    run_with(env);
}

fn run_with<E>(env: E)
where
    E: Has<Host> + Has<Port> + Has<Verbosity>,
{
    // Accessing via trait-method + type annotation.
    let host: &Host = env.access();

    // Accessing via free-standing function + turbofish.
    let port = access_from::<Port, _>(&env);

    // Accessing via free-standing function + turbofish + infix.
    let verbosity = access::<Verbosity>().from(&env);

    println!(
        "host: {:?}, port: {:?}, verbosity: {:?}",
        host, port, verbosity
    )
}

#[derive(Debug)]
struct Env {
    host: Host,
    port: Port,
    verbosity: Verbosity,
    restriction: Restriction,
}

#[derive(Debug)]
struct Host(String);

#[derive(Debug)]
struct Port(u16);

#[derive(Debug)]
enum Verbosity {
    Low,
    High,
}

#[derive(Debug)]
enum Restriction {
    Permissive,
    Enforcing,
}

// Implementing manually.
impl Has<Host> for Env {
    fn access(&self) -> &Host {
        &self.host
    }
}

// Implementing with a macro.
haz::impl_has_for_named_component!(Env, Port, port);

haz::impl_has_for_named_component!(Env, Verbosity, verbosity);
