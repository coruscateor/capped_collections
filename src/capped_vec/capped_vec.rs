use core::panic;

use std::{array, ascii::escape_default, fmt::Display, mem, ops::{Index, IndexMut}, slice::{Iter, IterMut}};

//Disabled

//use corlib::inc_dec::*;

use inc_dec::*;

use std::fmt::Debug;

use std::mem::take;

#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

///
/// An std::vec::Vec-like type with a compile-time maximum size.
/// 
/// Uses a stack allocated array to store pushed items internally. 
/// 
pub struct CappedVec<T, const N: usize>
    where T: Default //+ Copy
{

    array: [T; N],
    len: usize          //The length of valid items.

}

impl<T, const N: usize> CappedVec<T, N>
    where T: Default //+ Copy
{

    pub fn new() -> Self
    {

        Self
        {

            array: array::from_fn(|_| T::default()), //[T::default(); N],
            len: 0

        }

    }

    /*
    pub fn with_capacity<const N2: usize>() -> CappedVec<T, N2>
    {

        CappedVec::<T, N2>::new()

    }
    */

    pub fn push(&mut self, value: T) -> Option<T>
    {

        let next_last_index = self.len;

        if next_last_index >= self.capacity()
        {

            //Can't fit

            return Some(value);

        }

        self.array[next_last_index] = value;

        self.len.pp();

        None

    }

    pub fn pop(&mut self) -> Option<T>
    {

        if self.len == 0
        {

            return None;

        }

        let last_index = self.len - 1;

        let poped = take(&mut self.array[last_index]);

        self.len = last_index;

        Some(poped)

    }

    pub fn len(&self) -> usize
    {

        self.len

    }

    pub const fn capacity(&self) -> usize
    {

        self.array.len()

    }

    pub fn last_index(&self) -> Option<usize>
    {

        let len = self.len;

        if len == 0
        {

            None

        }
        else
        {

            Some(len - 1)
            
        }

    }

    //Should array elements by mutable by reference? No.

    pub fn try_index(&mut self, index: usize) -> Option<&T>
    {

        if index < self.capacity()
        {

            Some(&self.array[index])

        }
        else
        {

            None
            
        }

    }

    pub fn try_mut_index(&mut self, index: usize) -> Option<&mut T>
    {

        if self.len == 0
        {

            return None;

        }

        let last_index = self.len - 1;

        if index <= last_index
        {

            Some(&mut self.array[index])

        }
        else
        {

            None
            
        }

    }

    pub fn is_full(&self) -> bool
    {

        if self.capacity() == 0
        {

            //An array with no capacity can never be full.

            return false;

        }

        self.len() == self.capacity()

    }

    pub fn is_empty(&self) -> bool
    {

        self.len() == 0

    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T>
    {

        let last_index;

        if self.len > 1
        {

            last_index = self.len - 1;

        }
        else
        {

            return self.array[..].iter();
            
        }

        self.array[..last_index].iter()

    }

    //Disabled

    /*
    pub fn iter<'a>(&'a self) -> CappedVecIterator<'a, T>
    {

        let last_index;

        if self.len > 1
        {

            last_index = self.len - 1;

        }
        else
        {

            return CappedVecIterator::new(self.array[..].iter());
            
        }

        CappedVecIterator::new(self.array[..last_index].iter())

    }
    */

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T>
    {

        let last_index;

        if self.len > 1
        {

            last_index = self.len - 1;

        }
        else
        {

            return self.array[..].iter_mut();
            
        }

        self.array[..last_index].iter_mut()

    }

    //Disabled

    /*
    pub fn iter_mut<'a>(&'a mut self) -> CappedVecIteratorMut<'a, T>
    {

        let last_index;

        if self.len > 1
        {

            last_index = self.len - 1;

        }
        else
        {

            return CappedVecIteratorMut::new(self.array[..].iter_mut());
            
        }

        CappedVecIteratorMut::new(self.array[..last_index].iter_mut())

    }
    */

    pub fn insert(&mut self, index: usize, item: T) -> Option<T>
    {

        if self.len == 0
        {

            //No room

            Some(item)

        }
        else if index <= self.len && !self.is_full()
        {

            let mut current_index = self.len - 1;

            //Move all items including that at the specified index to the right.

            while current_index >= index
            {

                let current_item = mem::take(&mut self.array[current_index]);

                self.array[current_index + 1] = current_item;

                current_index.mm();

            }

            //Finally insert the item at the specified index and increment the length. 

            self.array[index] = item;

            self.len.pp();

            None

        }
        else
        {

            Some(item)

        }

    }

    pub fn remove(&mut self, index: usize) -> Option<T>
    {

        if self.len == 0
        {

            None

        }
        else if index < self.len
        {

            let removed_item = mem::take(&mut self.array[index]);

            //Move all items to the left to close the gap.

            let mut current_index = index + 1;

            while current_index < self.len
            {

                let current_item = mem::take(&mut self.array[current_index]);

                self.array[current_index - 1] = current_item;

                current_index.pp();

            }

            self.len.mm();

            Some(removed_item)

        }
        else
        {

            None
            
        }

    }

    pub fn clear(&mut self)
    {

        if self.len > 0
        {

            let last_index = self.len - 1;

            for item in self.array[..last_index].iter_mut()
            {
    
                *item = T::default();
    
            }

            self.len = 0;

        }

    }

    pub fn first(&self) -> Option<&T>
    {

        if self.len == 0
        {

            None

        }
        else
        {

            Some(&self.array[0])
            
        }

    }

    pub fn first_mut(&mut self) -> Option<&mut T>
    {

        if self.len == 0
        {

            None

        }
        else
        {

            Some(&mut self.array[0])
            
        }

    }

    pub fn last(&self) -> Option<&T>
    {

        if self.len == 0
        {

            None

        }
        else
        {

            Some(&self.array[self.len - 1])
            
        }

    }

    pub fn last_mut(&mut self) -> Option<&mut T>
    {

        if self.len == 0
        {

            None

        }
        else
        {

            Some(&mut self.array[self.len - 1])
            
        }

    }

    /*
    pub fn as_slice(&self) -> &[&T]
    {

        let array = [&T; self.len];



    }
    */

    pub fn as_slice(&self) -> &[T]
    {

        if self.len == 0
        {

            &self.array[..]

        }
        else
        {
            let last_index = self.len - 1;

            &self.array[..last_index]
            
        }
        
    }

    pub fn as_mut_slice(&mut self) -> &mut [T]
    {

        if self.len == 0
        {

            self.array[..].as_mut()

        }
        else
        {
            let last_index = self.len - 1;

            self.array[..last_index].as_mut()
            
        }
        
    }

    pub fn contains(&self, val_ref: &T) -> bool
        where T: PartialEq
    {

        for item in self.iter()
        {

            if item == val_ref
            {

                return true;

            }

        }

        false

    }

}

impl<T, const N: usize> Index<usize> for CappedVec<T, N>
    where T: Default //+ Copy
{

    //type Output = Option<&T>;

    type Output = T;

    fn index(&self, index: usize) -> &Self::Output
    {

        if index >= self.len
        {

            panic!("Error: The provided index is out of bounds")

        }

        &self.array[index]

        /*
        let len = self.len;

        if len == 0
        {

            &None

        }
        else
        {

            if index < len
            {
    
                &Some(&self.array[index])
    
            }
            else
            {
                
                &None
    
            }

        }
        */

    }

}

impl<T, const N: usize> IndexMut<usize> for CappedVec<T, N>
    where T: Default //+ Copy
{

    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {

        if index >= self.len
        {

            panic!("Error: The provided index is out of bounds")

        }

        &mut self.array[index]

        /*
        let panic_message: &'static str = "Error: Invalid index";

        let len = self.len;

        if len == 0
        {

            panic!("{}", panic_message)

            //&None

        }
        else
        {
        
            if index < len //self.last_index
            {

                &mut self.array[index]

            }
            else
            {

                panic!("{}", panic_message)
                
                //panic!("Error: Invalid index")

                //&mut None

            }

        }
        */

    }

}

impl<T, const N: usize> Display for CappedVec<T, N>
    where T: Display + Default //+ Copy
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        let last_index;

        if self.capacity() == 0 || self.len() == 0
        {

            last_index = 0;

        }
        else
        {

            last_index = self.len() - 1;

        }

        let mut current_index = 0;

        f.write_str("[")?;

        for item in self.iter()
        {

            write!(f, "{}", item)?;

            if current_index < last_index
            {

                f.write_str(", ")?;

            }
            else
            {

                break;
                
            }

            current_index.pp();

        }

        f.write_str("]")?;

        Ok(())
        
    }

}

impl<T, const N: usize> Debug for CappedVec<T, N>
    where T: Debug + Default //+ Copy
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CappedVec").field("array", &self.array).field("len", &self.len).finish()
    }

}

impl<T, const N: usize> Clone for CappedVec<T, N>
    where T: Clone + Default //+ Copy
{

    fn clone(&self) -> Self {
        Self { array: self.array.clone(), len: self.len.clone() }
    }
    
}


impl<T, const N: usize> Copy for CappedVec<T, N>
    where T: Default + Copy
{

}

#[cfg(feature = "serde")]
impl<T, const N: usize> Serialize for CappedVec<T, N>
    where T: Default  + Serialize //+ Copy
{

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {

        let mut seq = serializer.serialize_seq(Some(self.len))?;

        let valid_range = self.as_slice();

        for item in valid_range
        {

            seq.serialize_element(item)?;

        }

        seq.end()

        //serializer.serialize_str(self.as_str())

    }

}

//CappedVecIterator

//Disabled

/*
///
/// The iterator for CappedVec which provides immutable references.
/// 
pub struct CappedVecIterator<'a, T>
{

    opt_iter: Iter<'a, T>

}

impl<'a, T> CappedVecIterator<'a, T>
{

    pub fn new(opt_iter: Iter<'a, T>) -> Self
    {

        Self
        {

            opt_iter

        }

    }

}

impl<'a, T> Iterator for CappedVecIterator<'a, T>
{

    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {

        match self.opt_iter.next()
        {

            Some(val) =>
            {

                Some(val)

            }
            None => None

        }

    }

}
*/

//Disabled

/*
///
/// The iterator for CappedVec which provides mutable references.
/// 
pub struct CappedVecIteratorMut<'a, T>
{

    opt_iter_mut: IterMut<'a, T>

}

impl<'a, T> CappedVecIteratorMut<'a, T>
{

    pub fn new(opt_iter_mut: IterMut<'a, T>) -> Self
    {

        Self
        {

            opt_iter_mut

        }

    }

}

impl<'a, T> Iterator for CappedVecIteratorMut<'a, T>
{

    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {

        match self.opt_iter_mut.next()
        {

            Some(val) =>
            {

                Some(val)

            }
            None => None
            
        }

    }

}
*/
