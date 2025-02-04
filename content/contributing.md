+++
title = "Contributing"
page_template = "page.html"
+++

Everyone is welcome to contribute to transformrs.

- Bugs can be reported via GitHub issues at [rikhuijzer/transformrs](https://github.com/rikhuijzer/transformrs/issues).
- Changes to the source code or documentation can be sent via GitHub pull requests to [rikhuijzer/transformrs](https://github.com/rikhuijzer/transformrs/pulls).
- Changes to this website can be sent via GitHub pull requests to [rikhuijzer/transformrs.org](https://github.com/rikhuijzer/transformrs.org/pulls).

<br>

## Contributing to the Source Code

As stated above, patches can be sent via GitHub PRs to [rikhuijzer/transformrs](https://github.com/rikhuijzer/transformrs/pulls).

To develop locally, you can clone the repository and run

```sh
$ cargo test
```

or if you have [`cargo-watch`](https://github.com/watchexec/cargo-watch) installed, you can run

```sh
$ cargo watch -x test
```

to automatically run the tests when you make changes.
