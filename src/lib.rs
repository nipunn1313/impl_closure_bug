#![feature(conservative_impl_trait)]

pub struct S {}
impl S {
    pub fn return_iter<'a>(&'a self, inp: &'a Vec<Option<&'a String>>) -> impl 'a + Iterator<Item=&'a String> {
        inp.iter().filter_map(move |x| {
            x.map(|inner| {
                inner
            })
        })
    }
}
