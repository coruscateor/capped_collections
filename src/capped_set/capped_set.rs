use core::slice::Iter;

use crate::capped_vec::CappedVec;

use delegate::delegate;
use inc_dec::IncDecSelf;

pub struct CappedSet<T, const N: usize>
    where T: Default + PartialEq
{

   capped_vec: CappedVec<T, N>

}

impl<T, const N: usize> CappedSet<T, N>
    where T: Default + PartialEq
{

    pub fn new() -> Self
    {

        Self
        {

            capped_vec: CappedVec::new()

        }

    }

    pub fn insert(&mut self, value: T) -> Option<T>
    {

        if self.capped_vec.contains(&value)
        {

            return Some(value);

        }

        self.capped_vec.push(value)

    }

    pub fn remove(&mut self, value: T) -> bool
    {

        let mut index: usize = 0;

        for item in self.capped_vec.iter()
        {

            if *item == value
            {

                return self.capped_vec.remove(index).is_some();

            }

            index.pp();

        }

        false

    }

    pub fn take(&mut self, value: T) -> Option<T>
    {

        let mut index: usize = 0;

        for item in self.capped_vec.iter()
        {

            if *item == value
            {

                return self.capped_vec.remove(index)

            }

            index.pp();

        }

        None

    }

    delegate! {
        to self.capped_vec {

            pub fn len(&self) -> usize;

            pub const fn capacity(&self) -> usize;

            pub fn is_full(&self) -> bool;

            pub fn is_empty(&self) -> bool;

            pub fn iter(&self) -> Iter<'_, T>;

            pub fn clear(&mut self);

            pub fn contains(&self, val_ref: &T) -> bool;

        }

    }
    
}