use osmformat::PrimitiveBlock;
use osmformat::PrimitiveGroup;
use groups;
use mdo;
use mdo::iter::bind;
use std::slice;

pub type OsmObjs<'a> = mdo::iter::UnboxedFlatMap<&'a PrimitiveGroup, slice::Items<'a, PrimitiveGroup>, groups::OsmObjs<'a>, FnOsmObjs<'a>>;

struct FnOsmObjs<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::OsmObjs<'a> for FnOsmObjs<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::OsmObjs<'a>
    {
        groups::iter(group, self.block)
    }
}

pub fn iter<'a>(block: &'a PrimitiveBlock) -> OsmObjs<'a> {
    bind(block.get_primitivegroup().iter(), FnOsmObjs { block: block })
}

pub type Nodes<'a> = mdo::iter::UnboxedFlatMap<&'a PrimitiveGroup, slice::Items<'a, PrimitiveGroup>, groups::Nodes<'a>, FnNodes<'a>>;

struct FnNodes<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::Nodes<'a> for FnNodes<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Nodes<'a>
    {
        groups::nodes(group, self.block)
    }
}

pub fn nodes<'a>(block: &'a PrimitiveBlock) -> Nodes<'a> {
    bind(block.get_primitivegroup().iter(), FnNodes { block: block })
}

pub type Ways<'a> = mdo::iter::UnboxedFlatMap<&'a PrimitiveGroup, slice::Items<'a, PrimitiveGroup>, groups::Ways<'a>, FnWays<'a>>;

struct FnWays<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::Ways<'a> for FnWays<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Ways<'a>
    {
        groups::ways(group, self.block)
    }
}

pub fn ways<'a>(block: &'a PrimitiveBlock) -> Ways<'a> {
    bind(block.get_primitivegroup().iter(), FnWays { block: block })
}

pub type Relations<'a> = mdo::iter::UnboxedFlatMap<&'a PrimitiveGroup, slice::Items<'a, PrimitiveGroup>, groups::Relations<'a>, FnRelations<'a>>;

struct FnRelations<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::Relations<'a> for FnRelations<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Relations<'a>
    {
        groups::relations(group, self.block)
    }
}

pub fn relations<'a>(block: &'a PrimitiveBlock) -> Relations<'a> {
    bind(block.get_primitivegroup().iter(), FnRelations { block: block })
}
