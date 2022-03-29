#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Tree
{
    name: String,
    children: Vec<Tree>
}

impl Tree
{
    pub fn new(name: &str, children: Vec<Tree>) -> Self
    { Tree { name: name.into(), children } }

    pub fn singleton(name: &str) -> Self
    { Tree::new(name, vec![]) }

    pub fn name(&self) -> &str
    { &self.name }

    pub fn children(&self) -> &[Tree]
    { &self.children }
}
