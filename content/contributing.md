+++
title = "Contributing"
page_template = "page.html"
+++

Everyone is welcome to contribute to transformrs.

- Bugs can be reported via GitHub issues at [transformrs](https://github.com/rikhuijzer/transformrs/issues).
- Changes to the source code or documentation can be sent via GitHub pull requests to [transformrs](https://github.com/rikhuijzer/transformrs/pulls).
- Changes to this website can be sent via GitHub pull requests to [www-transformrs](https://github.com/rikhuijzer/www-transformrs/pulls).

<br>

## Contributing to the Source Code

As stated above, patches can be sent via GitHub PRs to [transformrs](https://github.com/rikhuijzer/transformrs/pulls).

To develop locally, you can clone the repository and run

```sh
$ cargo test
```

or if you have [`cargo-watch`](https://github.com/watchexec/cargo-watch) installed, you can run

```sh
$ cargo watch -x test
```

to automatically run the tests when you make changes.
This does require the right API keys to be set up.
Depending on what API keys you have, you can either run only the tests that you care about, or make the changes and send the patch in with a request to run the tests in CI.
The CI has access to many API keys, so feel free to ask for the test to be run there.
