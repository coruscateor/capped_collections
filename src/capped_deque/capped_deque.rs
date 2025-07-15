use core::panic;

use core::{array, ascii::escape_default, fmt::Display, mem, ops::{Index, IndexMut}, slice::{Iter, IterMut}};

//Disabled

//use corlib::inc_dec::*;

use inc_dec::*;

use core::fmt::Debug;

use core::mem::take;

#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

pub struct CappedDeque<T, const N: usize>
    where T: Default
{

    array: [T; N],
    len: usize,
    front_index: usize,
    back_index: usize

}

impl<T, const N: usize> CappedDeque<T, N>
    where T: Default
{

    pub fn new() -> Self
    {

        Self
        {

            array: array::from_fn(|_| T::default()),
            len: 0,
            front_index: 0,
            back_index: 0

        }

    }

    pub fn len(&self) -> usize
    {

        self.len

    }

    pub const fn capacity(&self) -> usize
    {

        self.array.len()

    }

    pub const fn has_capacity(&self) -> bool
    {

        self.capacity() > 0

    }

    const fn last_capacity_index(&self) -> Option<usize>
    {

        if self.has_capacity()
        {

            return Some(self.capacity() - 1);

        }

        None

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

        self.len == 0

    }

    //In terms of queue position the front index increases and wraps and and the back index decreases and wraps as values are pushed.

    pub fn push_front(&mut self, value: T) -> Option<T>
    {

        if let Some(last_capacity_index) = self.last_capacity_index()
        {

            //Has the front index wrapped?

            if self.front_index < self.back_index
            {

                //Minus one to get the element count.

                let gap = (self.back_index - self.front_index) - 1;

                if gap > 0
                {

                    let next_front_index = self.front_index + 1;

                    self.array[next_front_index] = value;

                    self.len.pp();

                    return None;

                }

            }
            else if self.front_index > self.back_index
            {

                let next_front_index = self.front_index + 1;       

                if next_front_index <= last_capacity_index
                {

                    self.array[next_front_index] = value;

                    self.len.pp();

                    return None;

                }
                else
                {

                    //Can the front index be wrapped?

                    if self.back_index > 0
                    {

                        self.front_index = 0;

                        self.array[self.front_index] = value;

                        self.len.pp();

                        return None;

                    }

                    //If not then the array is full.
                    
                }
                
            }
            else if self.front_index == self.back_index
            {

                if self.front_index < last_capacity_index
                {

                    self.front_index.pp();

                    self.array[self.front_index] = value;

                    self.len.pp();

                    return None;

                }
                else if self.front_index == last_capacity_index && self.is_empty()
                {

                    self.front_index = last_capacity_index;

                    self.array[self.front_index] = value;

                    self.len.pp();

                    return None;
                    
                }

            }

        }

        Some(value)

    }

    pub fn push_back(&mut self, value: T) -> Option<T>
    {

        if self.has_capacity()
        {

            if self.back_index <= self.front_index
            {

                if self.back_index == 0
                {

                    if self.front_index > 0
                    {

                        let gap = self.capacity() - (self.front_index + 1);

                        if gap > 0
                        {

                            //Wrap the back index.

                            self.back_index = self.capacity() - 1;

                            self.array[self.back_index] = value;

                            self.len.pp();

                            return None;

                        }

                    }
                    else if self.is_empty()
                    {

                        self.array[self.back_index] = value;

                        self.len.pp();

                        return None;
                        
                    }

                }

            }
            else if self.back_index > self.front_index
            {

                let gap = (self.back_index - self.front_index) - 1;

                if gap > 0
                {

                    self.array[self.back_index.mm()] = value;

                    self.len.pp();

                    return None;

                }

            }

        }

        Some(value)

    }

    //push_front

    //push_back

    //pop_front

    //pop_back

    //pop_front_if

    //pop_back_if

    //swap_remove_front

    //swap_reomove_back

    //insert

    //remove

    //get

    //get_mut
    
    //swap

    //iter

    //iter_mut

    //range

    //range_mut

    //drain

    //clear

    //contains

    //front

    //front_mut

    //back

    //back_mut

}