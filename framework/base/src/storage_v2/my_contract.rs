use core::ops::DerefMut;

use super::{my_struct::MyStructA, root_field::RootFieldMut, *};
use crate::{api, const_key, key::*, storage};

const_key! {KeyA, "a"}
const_key! {KeyB, "b"}
const_key! {KeyMy, "my"}

struct ContractObj;

trait ContractTrait: ContractBase {
    storage! {KeyA, root_a, root_a_mut, SingleValueStorage}

    storage! {KeyB, root_b, root_b_mut, BorrowedStorage}

    storage! {KeyMy, root_my, root_my_mut, MyStructA}

    fn my_endpoint(&mut self) {
        let mut st = self.root_a_mut();
        // let mut st2 = self.root_a_mut(); // won't compile 👍
        st.set("first".to_owned());
    }

    fn my_view(&self) {
        // let st_a = self.storage_ref(KeyA);
        let st_a = self.root_a();
        let mut st_b = self.root_b(); // 2nd borrow, allowed
        let st_b_ref = &st_b;
        // st_a.set("boom".to_owned()); // won't compile 👍
        // println!("storage: {}", st_a.get());
        // println!("storage: {}", st_b.get());
        // st_b.value = "boom".to_owned(); // won't compile 👍
    }

    fn my_endpoint_2(&mut self) {
        let (mut my, mut st_a, mut st_b) =
            self.split_dont_call_3(Self::root_my_mut, Self::root_a_mut, Self::root_b_mut);
        // = borrow!(self, root_my_mut, root_a_mut, root_b_mut);

        // let mut st_a = self.root_a_mut();
        // let mut st_b = self.root_b_mut();
        st_a.set("hello".to_owned());
        st_b.value = "world".to_owned();
        let mut new_item = my.vec_of_b.push();
        new_item.b1.value = "my.vec_of_b[0].b1".to_owned();
        new_item.b2.push().value = "my.vec_of_b[0].b2[0]".to_owned();

        my.vec_of_b.push().b1.value = "my.vec_of_b[1].b1".to_owned();
    }
}

impl ContractBase for ContractObj {}
impl ContractTrait for ContractObj {}

pub fn do_stuff() {
    let mut contract = ContractObj;
    contract.my_endpoint();
    contract.my_view();
    contract.my_endpoint_2();
    contract.my_view();

    api::dump_storage();
}
