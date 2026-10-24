#![allow(non_snake_case)]

use crate::QueryStorage;

macro_rules! impl_query_storage {
    (
        $first_ty:ident : $first_idx:tt
        $(, $ty:ident : $idx:tt)+
        $(,)?
    ) => {
        impl<'w, $first_ty, $($ty),+> QueryStorage<'w> for ($first_ty, $($ty),+)
        where $first_ty: QueryStorage<'w>,
              $($ty: QueryStorage<'w>),+
        {
            type Item = ($first_ty::Item, $($ty::Item),+);

            fn len(&self) -> usize {
                self.$first_idx.len()
            }

            fn entity(&self, dense: usize) -> u32 {
                self.$first_idx.entity(dense)
            }

            fn contains(&self, entity: u32) -> bool {
                self.$first_idx.contains(entity) $ (&& self.$idx.contains(entity))+
            }

            fn get(&mut self, dense: usize) -> Option<Self::Item> {
                let entity = self.$first_idx.entity(dense);

                Some((
                    self.$first_idx.get(dense)?,
                    $(self.$idx.get_by_entity(entity)?),+
                ))
            }

            fn get_by_entity(&mut self, entity: u32) -> Option<Self::Item> {
                Some((
                    self.$first_idx.get_by_entity(entity)?,
                    $(self.$idx.get_by_entity(entity)?),+
                ))
            }
        }
    };
}

impl_query_storage!(A: 0, B: 1);
impl_query_storage!(A: 0, B: 1, C: 2);
impl_query_storage!(A: 0, B: 1, C: 2, D: 3);
impl_query_storage!(A: 0, B: 1, C: 2, D: 3, E: 4);
impl_query_storage!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
impl_query_storage!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
impl_query_storage!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
impl_query_storage!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
impl_query_storage!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
