use crate::capped_vec::CappedVec;

use delegate::delegate;

use std::slice::{Iter, IterMut};

pub struct CappedMap<K, V, const N: usize>
    where K: PartialEq
{

   capped_vec: CappedVec<Option<(K, V)>, N>

}

impl<K, V, const N: usize> CappedMap<K, V, N>
    where K: PartialEq
{

    pub fn new() -> Self
    {

        Self
        {

            capped_vec: CappedVec::new()

        }

    }

    pub fn contains_key(&self, k: &K) -> bool
    {

        for item in self.capped_vec.iter()
        {

            if let Some(key_value) = item
            {

                if key_value.0 == *k
                {

                    return true;

                }
                
            }

        }

        false

    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    {

        if self.contains_key(&k)
        {

            return Some(v);

        }

        let push_result = self.capped_vec.push(Some((k, v)));

        let push_result = push_result.flatten();

        if let Some(key_value) = push_result
        {

            return Some(key_value.1);

        }

        None

    }

    pub fn get(&self, k: &K) -> Option<&V>
    {

        for item in self.capped_vec.iter()
        {

            if let Some(key_value) = item
            {

                if key_value.0 == *k
                {

                    return Some(&key_value.1);

                }

            }

        }

        None

    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V>
    {

        for item in self.capped_vec.iter_mut()
        {

            if let Some(key_value) = item
            {

                if key_value.0 == *k
                {

                    return Some(&mut key_value.1);

                }

            }

        }

        None

    }

    pub fn iter<'a>(&'a self) -> super::Iter<'a, K, V>
    {

        super::Iter::new(self.capped_vec.iter())

    }

    pub fn first(&self) -> Option<&(K, V)>
    {

        if let Some(first_opt) = self.capped_vec.first()
        {
            
            if let Some(first) = first_opt
            {

                return Some(first);
                
            }

        }

        None

    }

    pub fn last(&self) -> Option<&(K, V)>
    {

        if let Some(last_opt) = self.capped_vec.last()
        {

            if let Some(last) = last_opt
            {

                return Some(last);
                
            }

        }

        None

    }

    delegate! {
        to self.capped_vec {

            pub fn len(&self) -> usize;

            pub const fn capacity(&self) -> usize;

            pub fn is_full(&self) -> bool;

            pub fn is_empty(&self) -> bool;

            pub fn clear(&mut self);

        }

    }

}