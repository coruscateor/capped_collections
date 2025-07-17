
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

use delegate::delegate;

use crate::CappedDeque;

//use crate::capped_deque::CappedDeque;

pub struct CappedQueue<T, const N: usize>
    where T: Default
{

    capped_deque: CappedDeque<T, N>

}

impl<T, const N: usize> CappedQueue<T, N>
    where T: Default
{

    pub fn new() -> Self
    {

        Self
        {

            capped_deque: CappedDeque::new()

        }

    }

    delegate!
    {

        to self.capped_deque
        {

            pub fn len(&self) -> usize;

            pub const fn capacity(&self) -> usize;

            pub const fn has_capacity(&self) -> bool;

            pub fn is_full(&self) -> bool;

            pub fn is_empty(&self) -> bool;

            #[call(push_front)]
            pub fn push(&mut self, value: T) -> Option<T>;

            #[call(pop_back)]
            pub fn pop(&mut self) -> Option<T>;

            pub fn clear(&mut self);

            pub fn front(&self) -> Option<&T>;

            pub fn front_mut(&mut self) -> Option<&mut T>;

            pub fn back(&self) -> Option<&T>;

            pub fn back_mut(&mut self) -> Option<&mut T>;

            #[call(front_is_back)]
            pub fn has_one(&self) -> bool;

        }

    }

    /*
    pub fn push_front(&mut self, value: T) -> Option<T>
    {

        self.capped_deque.push_front(value)

    }
    */

}