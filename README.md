# floatd 🛟 🐛 🖥️ ![Build Status] [![Apache Badge]][Apache Link] [![MIT Badge]][MIT Link]

Tiny crate that provides a "FloatD" trait, i.e. num_traits::Float + Debug + Display.
This is likely to be subsumed into a successor crate once my project is more mature.
For now, this is the most convenient place to keep the trait.

According to the rust api guidelines chapter on [futureproofing](https://rust-lang.github.io/api-guidelines/future-proofing.html)
specifying Display and Debug as trait bounds is bad practice.

I think my downstream use case for this crate (guaranteeing a struct's underlying data can be formatted into an associated error type)
falls under exception 1 listed in the guidelines. Your mileage may vary, though.

## Compatibility

This is a no-std crate by default, with std support behind a feature flag.

## Dev Dependencies

The pre-commit config relies on [casey/just: 🤖 Just a command runner](https://github.com/casey/just)

## License

Licensed under either of

* [Apache License, Version 2.0][Apache Link]
* [MIT license][MIT LINK]

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[Build Status]: https://github.com/rileyleff/floatd/actions/workflows/rust.yml/badge.svg
[MIT Badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[MIT Link]: https://opensource.org/licenses/MIT
[Apache Badge]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[Apache Link]: https://opensource.org/licenses/Apache-2.0
