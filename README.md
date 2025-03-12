<div align="center">

# Capped Collections

[![Crates.io](https://img.shields.io/crates/v/mapage_lib)](https://crates.io/crates/mapage_lib)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](#license)
[![Downloads](https://img.shields.io/crates/d/mapage_lib)](https://crates.io/crates/mapage_lib)
[![Docs](https://docs.rs/mapage_lib/badge.svg)](https://docs.rs/mapage_lib/latest/mapage_lib/)
[![Twitch Status](https://img.shields.io/twitch/status/coruscateor)](https://www.twitch.tv/coruscateor)

[X](https://twitter.com/Coruscateor) | 
[Twitch](https://www.twitch.tv/coruscateor) | 
[Youtube](https://www.youtube.com/@coruscateor) | 
[Mastodon](https://mastodon.social/@Coruscateor) | 
[GitHub](https://github.com/coruscateor) | 
[GitHub Sponsors](https://github.com/sponsors/coruscateor)

Collections with compile-time set capacities.

</div>

## Examples:

### CappedVec

```rust

    //Initialising and pushing values

    let mut capped_vec = CappedVec::<i32, 5>::new();

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    assert_eq!(capped_vec.len(), 4);

```

```rust

    //Pushing then poping

    let mut capped_vec = CappedVec::<i32, 5>::new();
    
    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.pop();

    capped_vec.pop();

    assert_eq!(capped_vec.len(), 0);

```

```rust

    //Pushing then iterating the contents.

    let mut capped_vec = CappedVec::<i32, 5>::new();

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    let mut i = 1;

    for item in capped_vec.iter()
    {

        assert_eq!(*item, i);

        i.pp();

    }

```

## Todo:

- Add more documentation
- Add code examples
- Add more tests
- Clean-up the code
- Add more collection types.

## Code Style

This project uses a coding style the emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust
fn foo()
{

    bar();

}

```

Not this:

```rust
fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.



