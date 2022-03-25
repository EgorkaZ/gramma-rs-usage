use crate::ast::{Tree, Action};

pub fn format<'a>(curr: &Tree) -> String
{
    format_rec(curr, 0)
}

fn format_rec<'a>(curr: &'a Tree, depth: usize) -> String
{
    let indent: String = std::iter::repeat(' ').take(depth * 4).collect();
    match curr {
        Tree::Primal(prim) => format!("{indent}{prim}"),
        Tree::BinOp(op, l, r) => format!("{indent}({l} {op} {r})",
                l=format_rec(l, 0), r=format_rec(r, 0)),
        Tree::UnOp(op, sub) => format!("{indent}{op}{sub}",
                sub=format_rec(sub, depth + 1)),
        Tree::IfThenElse(c, t, Some(e))
            => format!("{indent}if {c} {{\n{t}\n{indent}}} else {{\n{e}\n{indent}}}",
                c=format_rec(c, 0), t=format_rec(t, depth + 1), e=format_rec(e, depth +  1)),
        Tree::IfThenElse(c, t, None)
            => format!("{indent}if {c} {{\n{t}\n{indent}}}",
            c=format_rec(c, 0), t=format_rec(t, depth + 1)),
        Tree::Let(name, init, expr)
            => format!("{indent}let {name} = {init};\n{expr}",
                init=format_rec(init, 0), expr=format_rec(expr, depth)),
        Tree::Action(Action::Print(expr))
            => format!("{indent}println!(\"{{}}\", {expr});",
                expr=format_rec(expr, 0)),
        Tree::Look(Action::Print(expr), then)
            => format!("{indent}println!(\"{{}}\", {expr});\n{then}",
                expr=format_rec(expr, 0), then=format_rec(then, depth)),
        Tree::FunCall(fun, arg)
            => format!("{fun}({arg})",
                fun=format_rec(fun, depth), arg=format_rec(arg, 0)),
    }
}
