# <p align="center">`bluebird`</p>

<p align="center">
    <a href="https://github.com/codereport/bluebird/issues" alt="contributions welcome">
        <img src="https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat" /></a>
    <a href="https://lbesson.mit-license.org/" alt="MIT license">
        <img src="https://img.shields.io/badge/License-MIT-blue.svg" /></a>    
    <a href="https://rust-lang.org/">
        <img src="https://img.shields.io/badge/Rust-2021-ff69b4.svg"/></a>
    <a href="https://github.com/codereport?tab=followers" alt="GitHub followers">
        <img src="https://img.shields.io/github/followers/codereport.svg?style=social&label=Follow" /></a>
    <a href="https://GitHub.com/codereport/bluebird/stargazers/" alt="GitHub stars">
        <img src="https://img.shields.io/github/stars/codereport/bluebird.svg?style=social&label=Star" /></a>
    <a href="https://twitter.com/code_report" alt="Twitter">
        <img src="https://img.shields.io/twitter/follow/code_report.svg?style=social&label=@code_report" /></a>
</p>

`bluebird` is a [Rust](https://rust-lang.org/) library that provides SKI combinators from [Combinatory Logic](https://combinatorylogic.com/) and common unary and binary functions that are often used with these combinators. It is the spritual equivalent of the C++ [`blackbird` library](https://github.com/codereport/blackbird).

How to use with Cargo:
```toml
[dependencies]
bluebird = "0.2.0"
```
How to use in your crate:
```rs
use bluebird;
```
Current combinators and functions provided:

* `phi1!`
* `_L_`
* `_MAX_`
* `_MIN_`
* `_MUL_`
* `_PLUS_`
* `_R_`
