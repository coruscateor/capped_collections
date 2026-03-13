
use super::CappedVecDeque;

#[test]
fn push_front_pop_back()
{

    let mut capped_vec_deque = CappedVecDeque::<i32, 5>::new();

    assert_eq!(capped_vec_deque.len(), 0);

    let opt_val = capped_vec_deque.push_front(1);

    assert_eq!(opt_val, None);

    let opt_val = capped_vec_deque.push_front(2);

    assert_eq!(opt_val, None);

    let opt_val = capped_vec_deque.push_front(3);

    assert_eq!(opt_val, None);

    let opt_val = capped_vec_deque.push_front(4);

    assert_eq!(opt_val, None);

    assert_eq!(capped_vec_deque.len(), 4);

    let opt_val = capped_vec_deque.pop_back();

    assert_eq!(opt_val, Some(4));

    let opt_val = capped_vec_deque.pop_back();

    assert_eq!(opt_val, Some(3));

    let opt_val = capped_vec_deque.pop_back();

    assert_eq!(opt_val, Some(2));

    let opt_val = capped_vec_deque.pop_back();

    assert_eq!(opt_val, Some(1));

    let opt_val = capped_vec_deque.pop_back();

    assert_eq!(opt_val, None);

    assert_eq!(capped_vec_deque.len(), 0);

}