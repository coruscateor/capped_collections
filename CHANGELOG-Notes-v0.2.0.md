commit c144fc991809533f02a1126a4503aa51659b10d4 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Mar 17 18:41:56 2026 +1300

    - Updated the minimum expected version of the serde dependency to 1.0.228.
    
    - Added the no_std feature.
    
    - Replaced std module references with the core module references in various places in the project.
    
    - Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the crate root module.

commit 590a2a78f498b9ed630eb26c95c8b23861057afb -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Mar 16 18:52:46 2026 +1300

    - Fixed the tests in the readme.
    
    - Added a CappedVec reset test to the readme.

commit fc99263ef5fe9de0957fe3c1b594337c06fcba54 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Mar 13 17:15:38 2026 +1300

    - Re-added the length check to the clear method of the CappedVec struct (removed earlier (same version)).
    
    - Added the clear_empty test function to the capped_vec_tests module.
    
    - Continued work on the CappedVecDeque struct.
    
    - Added the push_front_pop_back test function to the capped_vec_deque_tests module.
    
    - Added the capped_vec_deque_tests sub-module to the capped_vec_deque module.

commit 5319d3c122756598629cd767ce2e9f07d5dba269
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Mar 11 20:13:14 2026 +1300

    - Fixed: The try_index and the try_index_mut methods of the the CappedVec struct now test if the provided index is currently less than the length rather the capacity or the last valid index.
    
    - Renamed the try_mut_index method to try_index_mut in the CappedVec struct implementation.
    
    - Fixed: The iter, iter_mut, as_slice and as_slice_mut now provide the length rather than the last valid index when creating slices.
    
    - Renamed the as_mut_slice method to as_slice_mut in the CappedVec struct implementation.
    
    - Started working on range indexing, but decided to shelve it for now.
    
    - Added the push_insert_remove, first_last, try_index_try_index_mut, contains, iterators and basic_slices test functions to the capped_vec_tests module.

commit 4dcf28b8f599fb73bb50c82e3cbed97dbffd72d1
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Mar 11 14:36:22 2026 +1300

    - Renamed the CappedDeque struct to CappedVecDeque and updated the rest of the project accordingly.
    
    - Added the remove_entry and remove methods to the CappedMap struct.
    
    - Continued work on the CappedVec struct.
    
    - Added the push_then_clear, push_then_reset, push_then_clear_completely and the push_then_clear_then_reset_then_clear_completely test functions to the capped_vec_tests module.

commit ac5eabf269161cf0e5237230cf3486b7324c93fe
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Mar 3 14:41:54 2026 +1300

    - Updated the delegate dependency minimum version to 0.13.5.
    
    - Added the accessorise dependency.
    
    - Added the paste dependency.
    
    - Added a has_capacity method to the CappedVec sturct.

commit bf84a37e2bc5504fb6efc2654334d2940af98436
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Feb 24 19:40:25 2026 +1300

    - Continued work on the CappedDeque, CappedMap, CappedQueue and CappedSet structs.
    
    - Added the reset and clear_completely methods to the CappedVec struct implementation.

commit 694a86a70789856a39c68d90086511d102693cb5
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Jul 17 16:52:52 2025 +1200

    - Updated the delegate dependency to version 0.13.4.
    
    - Added pop_front, pop_back, clear, front, front_mut, back, back_mut and front_is_back methods to the CappedDeque struct implementation.
    
    - Added the capped_queue public module along with its CappedQueue struct.
    
    - Re-exported the capped_deque::CappedDeque, capped_map::CappedMap, capped_queue::CappedQueue, capped_set::CappedSet and the capped_vec::CappedVec structs in the create module scope.

commit 44fd5220283b59a9c82978218bbf04177c741120
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jul 15 17:39:41 2025 +1200

    - Replaced all references to the std library with the core library in the module containing the CappedVec struct.
    
    - Added the capped_deque module and the CappedDeque struct to this module.

commit 8742d2797f080c4e5000523553c73ac9cb4aa216
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jul 8 16:57:17 2025 +1200

    - Added the delegate dependency.
    
    - Added the capped_map public module containing CappedMap and Iter structs.
    
    - Added the capped_set public module containing a CappedSet struct.

commit fdd8dd978b283f620cfdfe295c8949f00fc79757
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jul 4 18:07:22 2025 +1200

    - Disabled the corlib dependency. Replaced it with the inc_dec crate.
    
    - Disabled the CappedVecIterator and the CappedVecIteratorMut returning versions of iter and iter_mut of the CappedVec struct and replaced them with versions that return core iterator objects (Iter and IterMut).
    
    - Disabled the CappedVecIterator and CappedVecIteratorMut structs.
    
    - Added the capped_vec public module and moved the CappedVec struct and its tests into it.

commit fb400360dd50f6c8f7e8af2c99e3b315e10a9be0
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jul 2 17:53:22 2025 +1200

    Updated the version string "0.2.0-alpha".
