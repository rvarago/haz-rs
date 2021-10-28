#![allow(dead_code)]

use haz::{access_from, Has};

fn main() {
    let env = Env {
        host: Host("localhost".into()),
        port: Port(8080),
        verbosity: Verbosity::High,
    };

    run_with(env);
}

fn run_with<E>(env: E)
where
    E: Has<Host> + Has<Port>,
{
    // Accessing via trait-method + type annotation.
    let host: &Host = env.access();

    // Accessing via free-standing function + turbofish.
    let port = access_from::<Port, _>(&env);

    println!("host: {:?}, port: {:?}", host, port)
}

#[derive(Debug)]
struct Env {
    host: Host,
    port: Port,
    verbosity: Verbosity,
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

// Implementing manually.
impl Has<Host> for Env {
    fn access(&self) -> &Host {
        &self.host
    }
}

// Implementing with a macro.
haz::impl_has_for_named_component!(Env, Port, port);
