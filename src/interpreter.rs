use std::{collections::HashMap, f64::consts};

use crate::{ast::Node, parser::Parser};

#[derive(Default)]
pub struct Interpreter {
    consts: HashMap<String, f64>,
    funcs: HashMap<String, (String, Node)>,
    temp_consts: Vec<(String, f64)>,
}

impl Interpreter {
    fn functions(&self, func: String) -> anyhow::Result<impl Fn(f64) -> f64> {
        Ok(match func.as_str() {
            "sqrt" => |x: f64| x.sqrt(),
            "ln" => |x: f64| x.ln(),
            "abs" => |x: f64| x.abs(),
            "cos" => |x: f64| x.cos(),
            "sin" => |x: f64| x.sin(),
            "tan" => |x: f64| x.tan(),
            "log" => |x: f64| x.log10(),
            _ => {

                anyhow::bail!("invalid function name: {}", func)
            }
        })
    }

    fn constants(&self, con: String) -> anyhow::Result<f64> {
        match con.as_str() {
            "pi" => Ok(consts::PI),
            "e" => Ok(consts::E),
            _ => {
                if let Some(val) = self.consts.get(&con) {
                    Ok(*val)
                } else {
                    anyhow::bail!("invalid constant name: {}", con)
                }
            }
        }
    }

    pub fn run(&mut self, text: String) -> anyhow::Result<f64> {
        let mut parser = Parser::new(text)?;
        let node = parser.calc()?;
        self.step(node)
    }

    fn step(&mut self, node: Node) -> anyhow::Result<f64> {
        Ok(match node {
            Node::Number(num) => num,
            Node::Expr(node1, op, node2) => match op {
                crate::ast::Operator::Plus => self.step(*node1)? + self.step(*node2)?,
                crate::ast::Operator::Minus => self.step(*node1)? - self.step(*node2)?,
                crate::ast::Operator::Mult => self.step(*node1)? * self.step(*node2)?,
                crate::ast::Operator::Div => self.step(*node1)? / self.step(*node2)?,
                crate::ast::Operator::Pow => self.step(*node1)?.powf(self.step(*node2)?),
            },
            Node::Function(func, node) => self.functions(func)?(self.step(*node)?),
            Node::AssignConst(name, expr) => {
                let val = self.step(*expr)?;
                self.consts.insert(name, val);
                val
            }
            Node::AssignFunc(name, var, body) => {
                self.funcs.insert(name, (var, *body));
                0.
            }
            Node::Const(const_name) => self.constants(const_name)?,
        })
    }
}
