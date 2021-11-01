# haz

> A thin abstraction over polymorphic environments.

[<img alt="github" src="https://img.shields.io/badge/github-rvarago/haz-rs?style=for-the-badge&logo=github" height="20">](https://github.com/rvarago/haz-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/haz.svg?style=for-the-badge&logo=rust" height="20">](https://crates.io/crates/haz)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-haz?style=for-the-badge" height="20">](https://docs.rs/haz)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/rvarago/haz-rs/CI/main?style=for-the-badge" height="20">](https://github.com/rvarago/haz-rs/actions?query=branch%3Amain)

## Motivation

Consider a scenario where we want to pass some data to a couple of functions.

Perhaps we've got a type `Config` representing our app's configuration which
wraps a bunch of data (e.g. `Host`, `Port`, `Verbosity`, `Restriction`, and maybe a bunch of extra fields):

```rust
struct Config {
  host: Host,
  port: Port,
  verbosity: Verbosity,
  restriction: Restriction,
  // ...
}
```

We might want to pass this data around to a couple of functions which would then use the relevant fields to take some action:

```rust
fn do_something_with_host_port_verbosity(...) {
  // ...
}

// ...
```

We could pass a reference to the whole `Config` to each function:

```rust
fn do_something_with_host_port_verbosity(cfg: &Config) {
  //...
}

// ...
```

Perhaps not every function needs to know about every single field and we might want to avoid such an unnecessary coupling.

One way to go about it is to explicitly pass each field to each function that requires them:

```rust
fn do_something_with_host_port_verbosity(host: &Host, port: &Port, restriction: &Restriction) {
  // ...
}

// ...
```

However at usage site it might get tedious since we need to pass each field individually:

```rust
let cfg = read_config();
do_something_with_host_port_verbosity(&cfg.host, &cfg.port, &cfg.verbosity);
```

## I can haz data?

The idea behind haz is to help in achieving both:

- Don't unnecessarily pass data to functions that don't require access to it
- Don't require passing each field individually

Everything floats around the thin trait `Has<Component>`:

```rust
trait Has<Component> {
    fn access(&self) -> &Component;
}
```

By implementing `Has<Component>` for some type `Container`, we're stating that `Container` can yield read-only access to `Component`.

Equipped with this trait and assuming we have implemented `Has<Host>`, `Has<Port>`, `Has<Verbosity>`, etc (maybe with `impl_has_for_named_component`)
for `Config`, we may leverage it as:

```rust
fn do_something_with_host_port_verbosity<C>(cfg: &C)
where
  C: Has<Host> + Has<Port> + Has<Verbosity> {
  //...
}

// ...
```

We've managed to explicitly state exactly what we as `do_something_with_host_port_verbosity` require and hence will get access to that _but nothing else_.

At usage site, it would look like:

```rust
let cfg = read_config();
do_something_with_host_port_verbosity(&cfg);
```

We're simply passing a reference to `Config` and not each field individually.

In summary, we've managed to achieve both of ours goals:

- Don't unnecessarily pass data to functions that don't require access to it
- Don't require passing each field individually
