use graphviz_rust::cmd::{Format, CommandArg};
use graphviz_rust::dot_generator::*;
use graphviz_rust::printer::{PrinterContext};
use graphviz_rust::{dot_structures::*, exec};

use crate::tree::Tree;

pub fn print_tree(tree: &Tree)
{
    let mut g = graph!(strict di id!("t"));
    let mut idx = 0;

    dfs_tree(tree, &mut idx, &mut g);
    // println!("{}", g.print(&mut PrinterContext::default()));
    exec(g, PrinterContext::default(), vec![
        CommandArg::Format(Format::Svg),
        CommandArg::Output("out.svg".into())
    ]).expect("Couldn't write graph");
}

pub fn dfs_tree(tree: &Tree, idx: &mut usize, graph: &mut Graph) -> String
{
    let node_id = format!("node_{}", idx);
    *idx += 1;
    let label = format!("\"{}\"", tree.name());

    graph.add_stmt(stmt!(node!(node_id;attr!("label",label))));
    tree.children().iter()
        .for_each(|child| {
            let other_id = dfs_tree(child, idx, graph);

            graph.add_stmt(stmt!(edge!(node_id!(node_id) => node_id!(other_id))));
        });
    node_id
}
