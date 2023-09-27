# typomania

The typomania project is a port to Rust of the excellent [`typogard`][typogard],
originally by a team led by Matthew Taylor at the University of Kansas and
published alongside the [_Defending Against Package Typosquatting_][paper]
paper, and adapted by [Dan Gardner][dangardner] for crates.io specifically.

Rather than being hard coded to a specific registry, this crate provides the
same set of primitives that `typogard` uses to detect potential typosquatting as
a reusable library that can be adapted to any registry by implementing the
traits provided in this crate.

## Features

* `rayon` (enabled by default): enables `Harness::check`, which provides
  functionality to check many packages in parallel using Rayon.

## Examples

### Fake registry

A basic example is provided in [`examples/registry.rs`](examples/registry.rs)
that fakes a registry and then matches packages against it. To see it operate
with some packages that generate potential typosquats, try:

```bash
cargo run --example registry -- -t abc,foo,foo-2 foo2 abd
```

### crates.io

An example project that uses this crate to analyse a crates.io database dump can
be found at [`typomania-crates`][typomania-crates].

## [Code of Conduct][code-of-conduct]

The Rust Foundation has adopted a Code of Conduct that we expect project 
participants to adhere to. Please read 
[the full text][code-of-conduct]
so that you can understand what actions will and will not be tolerated.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Licenses

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with documentation portions covered by the
Creative Commons Attribution 4.0 International license..

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), 
[LICENSE-documentation](LICENSE-documentation), and 
[COPYRIGHT](COPYRIGHT) for details.

You can also read more under the Foundation's [intellectual property
policy][ip-policy].

## Other Policies

You can read about other Rust Foundation policies in the footer of the
Foundation [website][foundation-website].

[code-of-conduct]: https://foundation.rust-lang.org/policies/code-of-conduct/
[dangardner]: https://github.com/dangardner/typogard
[foundation-website]: https://foundation.rust-lang.org
[ip-policy]: https://foundation.rust-lang.org/policies/intellectual-property-policy/
[media-guide and trademark]: https://foundation.rust-lang.org/policies/logo-policy-and-media-guide/
[paper]: https://dl.acm.org/doi/10.1007/978-3-030-65745-1_7
[rust-foundation]: https://foundation.rust-lang.org/
[typogard]: https://github.com/mt3443/typogard
[typomania-crates]: https://github.com/rustfoundation/typomania-crates
