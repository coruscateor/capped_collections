use std::fmt::Debug;


pub struct Iter<'a, K: 'a, V: 'a>
{

    slice_iter: std::slice::Iter<'a, Option<(K, V)>>

}

impl<'a, K, V> Iter<'a, K, V>
{

    pub fn new(slice_iter: std::slice::Iter<'a, Option<(K, V)>>) -> Self
    {

        Self
        {

            slice_iter

        }

    }

}

impl<'a, K, V> Iterator for Iter<'a, K, V>
{

    type Item = &'a (K, V);

    fn next(&mut self) -> Option<Self::Item>
    {

        //Loop to skip any empty elements.

        loop
        {

            let next_slice_iter = self.slice_iter.next();

            if let Some(key_value_opt) = next_slice_iter
            {

                if let Some(key_value_ref) = key_value_opt
                {

                    return Some(key_value_ref);

                }

            }
            else
            {
                
                break;

            }
            
        }
        
        None

    }

}

impl<'a, K, V> Debug for Iter<'a, K, V>
    where K: Debug, V: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Iter").field("slice_iter", &self.slice_iter).finish()
    }

}

impl<'a, K, V> Default for Iter<'a, K, V>
    where K: Default, V: Default
{

    fn default() -> Self {
        Self { slice_iter: Default::default() }
    }

}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V>
{
}
