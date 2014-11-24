use osmformat::PrimitiveBlock;
use osmformat::PrimitiveGroup;
use groups;
use mdo;
use mdo::iter::bind;
use std::slice;

pub type SimpleNodes<'a> = mdo::iter::UnboxedFlatMap<&'a PrimitiveGroup, slice::Items<'a, PrimitiveGroup>, groups::SimpleNodes<'a>, FnSimpleNodes<'a>>;

struct FnSimpleNodes<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::SimpleNodes<'a> for FnSimpleNodes<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::SimpleNodes<'a>
    {
        groups::simple_nodes(group, self.block)
    }
}

pub fn simple_nodes<'a>(block: &'a PrimitiveBlock) -> SimpleNodes<'a> {
    bind(block.get_primitivegroup().iter(), FnSimpleNodes { block: block })
}

pub type DenseNodes<'a> = mdo::iter::UnboxedFlatMap<&'a PrimitiveGroup, slice::Items<'a, PrimitiveGroup>, groups::DenseNodes<'a>, FnDenseNodes<'a>>;

struct FnDenseNodes<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::DenseNodes<'a> for FnDenseNodes<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::DenseNodes<'a>
    {
        groups::dense_nodes(group, self.block)
    }
}

pub fn dense_nodes<'a>(block: &'a PrimitiveBlock) -> DenseNodes<'a> {
    bind(block.get_primitivegroup().iter(), FnDenseNodes { block: block })
}
