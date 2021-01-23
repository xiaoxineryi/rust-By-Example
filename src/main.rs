use crate::Display::DisplayExample::{display, display2, display3};
use crate::Enums::linkedList::listTest;
use crate::FlowOfControl::Iter::{iter, into_iter, iter_mut};
use crate::FlowOfControl::Match::{M, iflet};
use crate::Fn::Closures::{capture, f};


mod Display;
mod Enums;
mod FlowOfControl;
mod Fn;

fn main() {
    f()
}