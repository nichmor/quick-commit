A `Rust`y re-implementation of `pre-commit` framework.

The goal of this framework is:

* combine the best developer experience from `pre-commit`, `husky` and `cargo-husky`.
* leverage `pixi` and conda-forge ecosystem to provide best-spoke enviroments with all system requirements necessary for your    pre-commit. No need to have python/go/other language installed.
* to be `quick`ly-fast: all pre-commit's checks will be executed in parallel.
* backward-compability with all existing pre-commits hooks - just plug'n'play.