use corlib::inc_dec::IncDecSelf;

use crate::CappedVec;

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
