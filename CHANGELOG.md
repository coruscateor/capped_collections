# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (1_/03/2026)

### Added

commit c144fc991809533f02a1126a4503aa51659b10d4

- Added the no_std feature.

commit 590a2a78f498b9ed630eb26c95c8b23861057afb

- Added a CappedVec reset test to the readme.

commit fc99263ef5fe9de0957fe3c1b594337c06fcba54

-- Re-added the length check to the clear method of the CappedVec struct (removed earlier (same version)).

Verify

- Added the clear_empty test function to the capped_vec_tests module.

- Added the push_front_pop_back test function to the capped_vec_deque_tests module.

- Added the capped_vec_deque_tests sub-module to the capped_vec_deque module.



### Changed

commit c144fc991809533f02a1126a4503aa51659b10d4

- Updated the minimum expected version of the serde dependency to 1.0.228.

- Replaced std module references with the core module references in various places in the project.

- Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the crate root module.

commit fc99263ef5fe9de0957fe3c1b594337c06fcba54

-- Continued work on the CappedVecDeque struct.



### Deprecated



### Removed



### Fixed

commit 590a2a78f498b9ed630eb26c95c8b23861057afb

- Fixed the tests in the readme.



### Security



## Version 0.1.0 (12/03/2025)

- Initial release


