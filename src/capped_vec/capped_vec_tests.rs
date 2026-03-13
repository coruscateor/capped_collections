//use corlib::inc_dec::IncDecSelf;

use inc_dec::IncDecSelf;

//use crate::CappedVec;

use super::CappedVec;

#[test]
fn push()
{

    //Initialising and pushing values

    let mut capped_vec = CappedVec::<i32, 5>::new(); //CappedVec::with_capacity::<5>();

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    assert_eq!(capped_vec.len(), 4);

}

#[test]
fn push2_pop2()
{

    //Pushing then poping

    let mut capped_vec = CappedVec::<i32, 5>::new();
    
    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.pop();

    capped_vec.pop();

    assert_eq!(capped_vec.len(), 0);

}

#[test]
fn push_iter()
{

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

}

#[test]
fn clear_empty()
{

    let mut capped_vec = CappedVec::<i32, 5>::new();

    capped_vec.clear();

    assert_eq!(capped_vec.len(), 0);
    
}

#[test]
fn push_then_clear()
{

    let mut capped_vec = CappedVec::<i32, 5>::new();

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.clear();

    assert_eq!(capped_vec.len(), 0);
    
}

#[test]
fn push_then_reset()
{

    let mut capped_vec = CappedVec::<i32, 5>::new();

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.reset();

    assert_eq!(capped_vec.len(), 0);
    
}

#[test]
fn push_then_clear_completely()
{

    let mut capped_vec = CappedVec::<i32, 5>::new();

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.clear_completely();

    assert_eq!(capped_vec.len(), 0);
    
}

#[test]
fn push_then_clear_then_reset_then_clear_completely()
{

    let mut capped_vec = CappedVec::<i32, 5>::new();

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.clear();

    assert_eq!(capped_vec.len(), 0);

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.reset();

    assert_eq!(capped_vec.len(), 0);

    capped_vec.push(1);

    capped_vec.push(2);
    
    capped_vec.clear_completely();

    assert_eq!(capped_vec.len(), 0);

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.push(5);

     assert_eq!(capped_vec.len(), 5);
    
}

#[test]
fn push_insert_remove()
{

    let mut capped_vec = CappedVec::<i32, 10>::new();

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    assert_eq!(capped_vec.insert(1, 5), None);

    assert_eq!(capped_vec[1], 5);

    assert_eq!(capped_vec[4], 4);

    assert_eq!(capped_vec[0], 1);

    assert_eq!(capped_vec.len(), 5);

    assert_eq!(capped_vec.remove(3), Some(3));

    assert_eq!(capped_vec[3], 4);

    assert_eq!(capped_vec[0], 1);

    assert_eq!(capped_vec.len(), 4);

}

#[test]
fn first_last()
{

    let mut capped_vec = CappedVec::<i32, 10>::new();

    assert_eq!(capped_vec.first(), None);

    assert_eq!(capped_vec.first_mut(), None);

    assert_eq!(capped_vec.last(), None);

    assert_eq!(capped_vec.last_mut(), None);

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.push(5);

    capped_vec.push(6);

    assert_eq!(capped_vec.first(), Some(&1));

    assert_eq!(capped_vec.first_mut(), Some(&mut 1));

    assert_eq!(capped_vec.last(), Some(&6));

    assert_eq!(capped_vec.last_mut(), Some(&mut 6));

}

#[test]
fn try_index_try_index_mut()
{

    let mut capped_vec = CappedVec::<i32, 10>::new();

    assert_eq!(capped_vec.try_index(0), None);

    assert_eq!(capped_vec.try_index_mut(0), None);

    assert_eq!(capped_vec.try_index(9), None);

    assert_eq!(capped_vec.try_index_mut(9), None);

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.push(5);

    capped_vec.push(6);

    assert_eq!(capped_vec.try_index(0), Some(&1));

    assert_eq!(capped_vec.try_index_mut(0), Some(&mut 1));

    assert_eq!(capped_vec.try_index(1), Some(&2));

    assert_eq!(capped_vec.try_index_mut(1), Some(&mut 2));

    assert_eq!(capped_vec.try_index(2), Some(&3));

    assert_eq!(capped_vec.try_index_mut(2), Some(&mut 3));

    assert_eq!(capped_vec.try_index(3), Some(&4));

    assert_eq!(capped_vec.try_index_mut(3), Some(&mut 4));

    assert_eq!(capped_vec.try_index(4), Some(&5));

    assert_eq!(capped_vec.try_index_mut(4), Some(&mut 5));

    assert_eq!(capped_vec.try_index(5), Some(&6));

    assert_eq!(capped_vec.try_index_mut(5), Some(&mut 6));

    assert_eq!(capped_vec.try_index(6), None);

    assert_eq!(capped_vec.try_index_mut(6), None);

}

#[test]
fn contains()
{

    let mut capped_vec = CappedVec::<i32, 10>::new();

    assert_eq!(capped_vec.contains(&0), false);

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.push(5);

    capped_vec.push(6);

    assert_eq!(capped_vec.contains(&1), true);

    assert_eq!(capped_vec.contains(&2), true);

    assert_eq!(capped_vec.contains(&3), true);

    assert_eq!(capped_vec.contains(&4), true);

    assert_eq!(capped_vec.contains(&5), true);

    assert_eq!(capped_vec.contains(&6), true);

    assert_eq!(capped_vec.contains(&7), false);

    assert_eq!(capped_vec.contains(&0), false);

}

#[test]
fn iterators()
{

    let mut capped_vec = CappedVec::<i32, 10>::new();

    let iter = capped_vec.iter();

    assert_eq!(iter.count(), 0);

    let iter = capped_vec.iter_mut();

    assert_eq!(iter.count(), 0);

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.push(5);

    capped_vec.push(6);

    let iter = capped_vec.iter();

    assert_eq!(iter.count(), 6);

    let iter = capped_vec.iter_mut();

    assert_eq!(iter.count(), 6);

}

#[test]
fn basic_slices()
{

    let mut capped_vec = CappedVec::<i32, 10>::new();

    let slice = capped_vec.as_slice();

    assert_eq!(slice.len(), 0);

    let slice = capped_vec.as_slice_mut();

    assert_eq!(slice.len(), 0);

    capped_vec.push(1);

    capped_vec.push(2);

    capped_vec.push(3);

    capped_vec.push(4);

    capped_vec.push(5);

    capped_vec.push(6);

    let slice = capped_vec.as_slice();

    assert_eq!(slice.len(), 6);

    let slice = capped_vec.as_slice_mut();

    assert_eq!(slice.len(), 6);

}

//Disabled

/*

#[test]
fn slices()
{

    let mut capped_vec = CappedVec::<i32, 10>::new();

    let all = capped_vec[..];



}

*/

