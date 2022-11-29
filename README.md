# DVC Tree Parser

[DVC]: https://dvc.org
[afc]: https://github.com/mdekstrand/astral-filing-cabinet/

This package provides the ability to parse pipelines and stage files from [DVC][] (Data Version Control) projects.  It is not a reimplementation of DVC;
it only serves to allow Rust projects to parse and read data, particularly the identities of generated artifacts, from DVC repositories.

Feature status:

- [x] Read `.dvc` files
- [x] Read `dvc.yaml` pipeline files
- [x] Read `dvc.lock` lock files
- [ ] Support `foreach` template stages
  - [x] with bare string items
  - [ ] with compound items
- [ ] Interpolate `vars`
- [ ] Interpolate `params.yaml`
- [ ] Support additional params files

We probably do not need to worry about params dependencies, because this is not intended to re-implement DVC.

Support for manipulating DVC artifacts — fetching, checking out, checking status, etc. — goes in the [Astral Filing Cabinet][afc], which uses this crate to parse DVC trees for their artifact data.
