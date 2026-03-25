# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (25/03/2026)

### Added

- Added the no_std feature.

- Added a CappedVec reset test to the readme.

- Added the clear_empty test function to the capped_vec_tests module.

- Added the push_front_pop_back test function to the capped_vec_deque_tests module.

- Added the capped_vec_deque_tests sub-module to the capped_vec_deque module.

- Added the push_insert_remove, first_last, try_index_try_index_mut, contains, iterators and basic_slices test functions to the capped_vec_tests module.

- Added the push_then_clear, push_then_reset, push_then_clear_completely and the push_then_clear_then_reset_then_clear_completely test functions to the capped_vec_tests module.

- Added the accessorise dependency.

- Added a has_capacity method to the CappedVec sturct.

- Added the reset and clear_completely methods to the CappedVec struct implementation.
    
- Added the capped_queue public module along with its CappedQueue struct.

- Added the delegate dependency.
    
- Added the capped_map public module containing CappedMap and Iter structs.
    
- Added the capped_set public module containing a CappedSet struct.

- Added the inc_dec dependency.

- Added the package authors field.

- Added the capped_vec_deque public module containing a CappedVecDeque struct.



### Changed

- Updated the minimum expected version of the serde dependency to 1.0.228.

- Replaced std module references with the core module references in various places in the project.

- Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the crate root module.

- Renamed the try_mut_index method to try_index_mut in the CappedVec struct implementation.

- Renamed the as_mut_slice method to as_slice_mut in the CappedVec struct implementation.

- Updated the delegate dependency minimum version to 0.13.5.

- Re-exported the capped_vec_deque::CappedVecDeque, capped_map::CappedMap, capped_queue::CappedQueue, capped_set::CappedSet and the capped_vec::CappedVec structs to the create module scope.

- Disabled the CappedVecIterator and the CappedVecIteratorMut returning versions of iter and iter_mut of the CappedVec implementation and replaced them with versions that return core iterator objects (Iter and IterMut).

- Disabled the CappedVecIterator and CappedVecIteratorMut structs.

- Updated the readme

- Updated the package keywords and categories.
    
- Ran cargo update

- Updated the minimum expected version of the serde dependency to 1.0.228.

- Replaced std module references with the core module references in various places in the project.

- Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the crate root module.

--- FUTURE COMMIT ---

--- Updated the package version string to "0.2.0".



### Removed

- Removed the corlib dependency.




### Fixed

- Fixed the tests in the readme.

- Fixed: The try_index and the try_index_mut methods of the the CappedVec implementation now test if the provided index is currently less than the length rather the capacity or the last valid index.

- Fixed: The as_slice and as_slice_mut now use the length rather than the last valid index when creating slices.



## Version 0.1.0 (12/03/2025)

- Initial release


