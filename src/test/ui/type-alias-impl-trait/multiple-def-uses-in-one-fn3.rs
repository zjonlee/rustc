// https://github.com/rust-lang/rust/issues/73481
// This test used to cause unsoundness, since one of the two possible
// resolutions was chosen at random instead of erroring due to conflicts.

#![feature(min_type_alias_impl_trait)]

type X<A: ToString + Clone, B: ToString + Clone> = impl ToString;

fn f<A: ToString + Clone, B: ToString + Clone>(a: A, b: B) -> (X<A, B>, X<B, A>) {
    (a, b)
}

fn g<A: ToString + Clone, B: ToString + Clone>(a: A, b: B) -> (X<A, B>, X<A, B>) {
    //~^ ERROR concrete type differs from previous defining opaque type
    (a, b)
    //~^ ERROR mismatched types
}

fn main() {}
