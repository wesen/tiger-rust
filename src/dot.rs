use std::io::Write;

use ast;
use ast::Exp::*;
use ast::Var::*;
use symbol;

fn label_var(v: &ast::Var, symbol_table: &symbol::SymbolTable) -> String {
    match v {
        &SimpleVar(s, _) => format!("SimpleVar({})", symbol_table.name(&s)),
        &FieldVar(_, s, _) => format!("FieldVar[{}]", symbol_table.name(&s)),
        &SubscriptVar(_, ref e, _) => String::from("SubscriptVar"),
    }
}

fn label_tree(tree: &ast::Exp, symbol_table: &symbol::SymbolTable) -> String {
    match tree {
        &VarExp(ref var) => label_var(&**var, symbol_table),
        &NilExp => String::from("NilExp"),
        &IntExp(i) => format!("IntExp({})", i),
        &StringExp(ref s, _) => format!("StringExp({})", s),
        &OpExp { op, .. } => format!("OpExp({:?})", op),

        &IfExp { .. } => String::from("IfExp"),

        _ => String::from("??"),
    }
}

type NodeId = u64;

trait Node {
    fn node_id(&self) -> NodeId;
    fn neighbors(&self) -> Vec<NodeId>;
}

impl Node for ast::Exp {
    fn node_id(&self) -> NodeId {
        (self as *const ast::Exp) as NodeId
    }

    fn neighbors(&self) -> Vec<NodeId> {
        match self {
            &VarExp(ref var) => var.neighbors(),

            &IfExp { ref test, ref then_, ref else_, .. } => {
                let mut v = vec![test.node_id(), then_.node_id()];
                if let &Some(ref else_) = else_ {
                    v.push(else_.node_id());
                }
                v
            }

            _ => vec![],
        }
    }
}

impl Node for ast::Var {
    fn node_id(&self) -> NodeId {
        (self as *const ast::Var) as NodeId
    }

    fn neighbors(&self) -> Vec<NodeId> {
        match self {
            &FieldVar(ref v, _, _) => vec![v.node_id()],
            &SubscriptVar(ref v, ref e, _) => vec![v.node_id(), e.node_id()],
            _ => vec![]
        }
    }
}

pub fn render_var<W>(out: &mut W, var: &ast::Var, symbol_table: &symbol::SymbolTable)
    where W: Write {
    let id = var.node_id();
    writeln!(out, r#"nd_{:x} [label="{}"]"#, id,
             label_var(var, symbol_table)).unwrap();
    for &l in var.neighbors().iter() {
        writeln!(out, r#"nd_{:x} -> nd_{:x};"#, id, l);
    }
}

pub fn render_ast<W>(out: &mut W, tree: &ast::Exp, symbol_table: &symbol::SymbolTable)
    where W: Write
{
    writeln!(out, r#"nd_{:x} [label="{}"]"#,
             tree.node_id(),
             label_tree(tree, symbol_table)).unwrap();
    let id = tree.node_id();
    for &l in tree.neighbors().iter() {
        writeln!(out, r#"nd_{:x} -> nd_{:x};"#, id, l);
    }

    match tree {
        &VarExp(ref v) => match v.as_ref() {
            &FieldVar(ref v, _, _) => render_var(out, v, symbol_table),
            &SubscriptVar(ref v, ref e, _) => {
                render_var(out, v, symbol_table);
                render_ast(out, e, symbol_table);
            },
            _ => (),
        },
        &IfExp { ref test, ref then_, ref else_, .. } => {
            render_ast(out, test.as_ref(), symbol_table);
            render_ast(out, then_.as_ref(), symbol_table);
            if let &Some(ref else_) = else_ {
                render_ast(out, else_, symbol_table);
            }
        },
        _ => ()
    }
}

