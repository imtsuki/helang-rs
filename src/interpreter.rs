use std::collections::HashMap;
use std::fmt;

use anyhow::{anyhow, bail, Result};

use crate::ir::*;

pub struct Interpreter {
    globals: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
enum Value {
    Number(i64),
    Array(Vec<i64>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Array(a) => {
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", v)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
enum VarRef {
    Ident(String),
    Index(String, usize),
    Slice(String, Vec<usize>),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }

    pub fn eval(&mut self, stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn eval_stmt(&mut self, stmt: Stmt) -> Result<()> {
        match stmt {
            Stmt::Decl(ident, literal) => {
                let value = self.eval_literal(literal)?;
                self.globals.insert(ident.0.clone(), value);
            }
            Stmt::Assign(assignee, literal) => {
                let value = self.eval_literal(literal)?;
                match assignee {
                    Expr::Index(ident, index) => {
                        let index = self.eval_literal(index)?;
                        match (index, value) {
                            (Value::Number(index), Value::Number(value)) => {
                                if let Some(Value::Array(array)) = self.globals.get_mut(&ident.0) {
                                    if index == 0 {
                                        array.fill(value);
                                    } else {
                                        array
                                            .get_mut(index as usize - 1)
                                            .map(|v| *v = value)
                                            .ok_or(anyhow!(
                                                "index {} out of bounds {}",
                                                index,
                                                array.len()
                                            ))?;
                                    }
                                } else {
                                    bail!("assigning to non-array");
                                }
                            }
                            (Value::Array(indices), Value::Number(value)) => {
                                if let Some(Value::Array(array)) = self.globals.get_mut(&ident.0) {
                                    for index in indices {
                                        array
                                            .get_mut(index as usize - 1)
                                            .map(|v| *v = value)
                                            .ok_or(anyhow!(
                                                "index {} out of bounds {}",
                                                index,
                                                array.len()
                                            ))?;
                                    }
                                } else {
                                    bail!("assigning to non-array");
                                }
                            }
                            _ => bail!("incompatible types"),
                        }
                    }
                    Expr::Ident(ident) => {
                        self.globals.insert(ident.0.clone(), value);
                    }
                    Expr::Lit(_) => bail!("cannot assign value to literal"),
                }
            }
            Stmt::Print(expr) => {
                let value = self.eval_expr(expr)?;
                println!("{}", value);
            }
            Stmt::Test5G => {
                println!("很残念，你的电脑并没有配备 5G 芯片。");
            }
        }
        Ok(())
    }

    fn eval_expr(&self, expr: Expr) -> Result<Value> {
        match expr {
            Expr::Lit(literal) => self.eval_literal(literal),
            Expr::Index(ident, literal) => {
                let var_ref = match literal {
                    Literal::Number(index) => VarRef::Index(ident.0, index as usize),
                    Literal::Array(indices) => VarRef::Slice(
                        ident.0,
                        indices.into_iter().map(|index| index as usize).collect(),
                    ),
                };
                self.eval_var_ref(var_ref)
            }
            Expr::Ident(ident) => {
                let var_ref = VarRef::Ident(ident.0);
                self.eval_var_ref(var_ref)
            }
        }
    }

    fn eval_literal(&self, literal: Literal) -> Result<Value> {
        match literal {
            Literal::Number(n) => Ok(Value::Number(n)),
            Literal::Array(array) => Ok(Value::Array(array)),
        }
    }

    fn eval_var_ref(&self, var_ref: VarRef) -> Result<Value> {
        let ident = match var_ref {
            VarRef::Ident(ref ident) => ident,
            VarRef::Index(ref ident, _) => ident,
            VarRef::Slice(ref ident, _) => ident,
        };
        if let Some(value) = self.globals.get(ident) {
            match var_ref {
                VarRef::Ident(_) => Ok(value.clone()),
                VarRef::Index(_, index) => match value {
                    Value::Array(array) => {
                        if index == 0 {
                            Ok(Value::Array(array.clone()))
                        } else {
                            Ok(Value::Number(*array.get(index - 1).ok_or(anyhow!(
                                "index {} out of bounds {}",
                                index,
                                array.len()
                            ))?))
                        }
                    }
                    _ => bail!("indexing non-array"),
                },
                VarRef::Slice(_, indices) => match value {
                    Value::Array(array) => {
                        let mut result = Vec::new();
                        for index in indices {
                            result.push(*array.get(index - 1).ok_or(anyhow!(
                                "index {} out of bounds {}",
                                index,
                                array.len()
                            ))?);
                        }
                        Ok(Value::Array(result))
                    }
                    _ => bail!("cannot slice non-array"),
                },
            }
        } else {
            bail!("undefined variable: {}", ident)
        }
    }
}
