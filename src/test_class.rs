use gdnative::prelude::*;
use crate::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TestClass;

#[methods]
impl TestClass {
    fn new(_owner: TRef<Node>) -> TestClass {
        TestClass
    }

    // Remember to annotate functions with `#[export]`
    // you want it available in godot
    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        gd_print!(owner, p, "Created");
    }
}