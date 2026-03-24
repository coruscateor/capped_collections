# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (2_/03/2026)

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

- Added the push_insert_remove, first_last, try_index_try_index_mut, contains, iterators and basic_slices test functions to the capped_vec_tests module.

commit 4dcf28b8f599fb73bb50c82e3cbed97dbffd72d1

-- Added the remove_entry and remove methods to the CappedMap struct.

Added in this version.

- Added the push_then_clear, push_then_reset, push_then_clear_completely and the push_then_clear_then_reset_then_clear_completely test functions to the capped_vec_tests module.

commit ac5eabf269161cf0e5237230cf3486b7324c93fe

- Added the accessorise dependency.

-- Added the paste dependency.

Removed

- Added a has_capacity method to the CappedVec sturct.

commit bf84a37e2bc5504fb6efc2654334d2940af98436

- Added the reset and clear_completely methods to the CappedVec struct implementation.

commit 694a86a70789856a39c68d90086511d102693cb5

- Added pop_front, pop_back, clear, front, front_mut, back, back_mut and front_is_back methods to the CappedDeque struct implementation.
    
- Added the capped_queue public module along with its CappedQueue struct.

commit 8742d2797f080c4e5000523553c73ac9cb4aa216

- Added the delegate dependency.
    
- Added the capped_map public module containing CappedMap and Iter structs.
    
- Added the capped_set public module containing a CappedSet struct.

commit fdd8dd978b283f620cfdfe295c8949f00fc79757

- Added the inc_dec dependency.

commit 0d6fd4d08c0a661f80d281741dc5369299ab108b

- Added the package authors field.

commit c144fc991809533f02a1126a4503aa51659b10d4

- Added the no_std feature.



### Changed

commit c144fc991809533f02a1126a4503aa51659b10d4

- Updated the minimum expected version of the serde dependency to 1.0.228.

- Replaced std module references with the core module references in various places in the project.

- Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the crate root module.

commit fc99263ef5fe9de0957fe3c1b594337c06fcba54

-- Continued work on the CappedVecDeque struct.

commit 5319d3c122756598629cd767ce2e9f07d5dba269

- Renamed the try_mut_index method to try_index_mut in the CappedVec struct implementation.

- Renamed the as_mut_slice method to as_slice_mut in the CappedVec struct implementation.

-- Started working on range indexing, but decided to shelve it for now.

commit 4dcf28b8f599fb73bb50c82e3cbed97dbffd72d1

-- Renamed the CappedDeque struct to CappedVecDeque and updated the rest of the project accordingly.

Added in this version.

-- Continued work on the CappedVec struct.

commit ac5eabf269161cf0e5237230cf3486b7324c93fe

-- Updated the delegate dependency minimum version to 0.13.5.

Added in this version.

Latest

commit ac5eabf269161cf0e5237230cf3486b7324c93fe

- Updated the delegate dependency minimum version to 0.13.5.

commit bf84a37e2bc5504fb6efc2654334d2940af98436

-- Continued work on the CappedDeque, CappedMap, CappedQueue and CappedSet structs.

commit 694a86a70789856a39c68d90086511d102693cb5

-- Updated the delegate dependency to version 0.13.4.

-- Re-exported the capped_deque::CappedDeque, capped_map::CappedMap, capped_queue::CappedQueue, capped_set::CappedSet and the capped_vec::CappedVec structs in the create module scope.

- Re-exported the capped_vec_deque::CappedVecDeque, capped_map::CappedMap, capped_queue::CappedQueue, capped_set::CappedSet and the capped_vec::CappedVec structs in the create module scope.

commit fdd8dd978b283f620cfdfe295c8949f00fc79757

-- Disabled the corlib dependency. Replaced it with the inc_dec crate.

Added inc_dec

Removed corlib later

- Disabled the CappedVecIterator and the CappedVecIteratorMut returning versions of iter and iter_mut of the CappedVec struct and replaced them with versions that return core iterator objects (Iter and IterMut).

- Disabled the CappedVecIterator and the CappedVecIteratorMut returning versions of iter and iter_mut of the CappedVec implementation and replaced them with versions that return core iterator objects (Iter and IterMut).

- Disabled the CappedVecIterator and CappedVecIteratorMut structs.

commit fb400360dd50f6c8f7e8af2c99e3b315e10a9be0

-- Updated the version string "0.2.0-alpha".

commit db583c7aa18bd9a5c0084e680dea7f57becb065b

- Updated the readme

commit 0d6fd4d08c0a661f80d281741dc5369299ab108b

-- Updated the package version string to "0.2.0-beta".

- Updated the package keywords and categories.

-- Assembled the changlog notes and prepared the changelog to have notes added to it.
    
- Ran cargo update

commit c144fc991809533f02a1126a4503aa51659b10d4

- Updated the minimum expected version of the serde dependency to 1.0.228.

- Replaced std module references with the core module references in various places in the project.

- Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the crate root module.

--- FUTURE COMMIT ---

--- Updated the package version string to "0.2.0".



### Deprecated



### Removed

commit 0d6fd4d08c0a661f80d281741dc5369299ab108b

- Removed the corlib dependency.

-- Removed the paste dependency.

Added in this version.



### Fixed

commit 590a2a78f498b9ed630eb26c95c8b23861057afb

- Fixed the tests in the readme.

commit 5319d3c122756598629cd767ce2e9f07d5dba269

-- Fixed: The try_index and the try_index_mut methods of the the CappedVec struct now test if the provided index is currently less than the length rather the capacity or the last valid index.

- Fixed: The try_index and the try_index_mut methods of the the CappedVec implementation now test if the provided index is currently less than the length rather the capacity or the last valid index.

-- Fixed: The iter, iter_mut, as_slice and as_slice_mut now provide the length rather than the last valid index when creating slices.

- Fixed: The as_slice and as_slice_mut now use the length rather than the last valid index when creating slices.

iter and iter_mut methods of the CappedVec implementation were added in this version.



### Security



## Version 0.1.0 (12/03/2025)

- Initial release


