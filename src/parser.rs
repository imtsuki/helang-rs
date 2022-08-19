use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ir::*;

#[derive(Parser)]
#[grammar = "helang.pest"]
struct HelangParser;

pub fn parse() -> Result<()> {
    let pairs = HelangParser::parse(
        Rule::program,
        r#"
        u8 a = 1 | 2 | 3
        u8 b = 4
        a = 3 | 4 | 5
        b[1] = 3
        b[2 | 3] = 4
        print a
        print b[2]
        print b[3 | 4]
        4
        1 | 2 | 3
    "#,
    )?;
    let mut stmts = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::declStmt => {
                let stmt = parse_decl_stmt(pair)?;
                stmts.push(stmt);
            }
            Rule::assignStmt => {
                let stmt = parse_assign_stmt(pair)?;
                stmts.push(stmt);
            }
            Rule::printStmt => {
                let stmt = parse_print_stmt(pair)?;
                stmts.push(stmt);
            }
            Rule::exprStmt => {
                let stmt = parse_expr_stmt(pair)?;
                stmts.push(stmt);
            }
            _ => (),
        }
    }
    println!("{:#?}", stmts);
    Ok(())
}

fn parse_decl_stmt(pair: Pair<Rule>) -> Result<Stmt> {
    let mut iter = pair.into_inner();
    let ident = iter.next().unwrap();
    let ident = parse_ident(ident)?;
    let literal = iter.next().unwrap();
    let literal = parse_literal(literal)?;

    Ok(Stmt::Decl(ident, literal))
}

fn parse_assign_stmt(pair: Pair<Rule>) -> Result<Stmt> {
    let mut iter = pair.into_inner();
    let expr = iter.next().unwrap();
    let expr = parse_expr(expr)?;
    let literal = iter.next().unwrap();
    let literal = parse_literal(literal)?;
    Ok(Stmt::Assign(expr, literal))
}

fn parse_print_stmt(pair: Pair<Rule>) -> Result<Stmt> {
    let mut iter = pair.into_inner();
    let expr = iter.next().unwrap();
    let expr = parse_expr(expr)?;
    Ok(Stmt::Print(expr))
}

fn parse_expr_stmt(pair: Pair<Rule>) -> Result<Stmt> {
    let mut iter = pair.into_inner();
    let expr = iter.next().unwrap();
    let expr = parse_expr(expr)?;
    Ok(Stmt::Print(expr))
}

fn parse_expr(pair: Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::ident => {
            let ident = parse_ident(pair)?;
            Ok(Expr::Ident(ident))
        }
        Rule::indexExpr => {
            let mut iter = pair.into_inner();
            let ident = iter.next().unwrap();
            let ident = parse_ident(ident)?;
            let literal = iter.next().unwrap();
            let literal = parse_literal(literal)?;
            Ok(Expr::Index(ident, literal))
        }
        Rule::number | Rule::array => {
            let literal = parse_literal(pair)?;
            Ok(Expr::Lit(literal))
        }
        _ => Err(anyhow::anyhow!("unexpected expr: {:?}", pair.as_str())),
    }
}

fn parse_ident(pair: Pair<Rule>) -> Result<Ident> {
    if let Rule::ident = pair.as_rule() {
        let text = pair.as_str();
        Ok(Ident(text.to_string()))
    } else {
        Err(anyhow::anyhow!("expected ident"))
    }
}

fn parse_literal(pair: Pair<Rule>) -> Result<Literal> {
    match pair.as_rule() {
        Rule::number => {
            let text = pair.as_str();
            let number = text.parse::<i64>()?;
            Ok(Literal::Number(number))
        }
        Rule::array => {
            let mut iter = pair.into_inner();
            let mut array = Vec::new();
            while let Some(pair) = iter.next() {
                let text = pair.as_str();
                let number = text.parse::<i64>()?;
                array.push(number);
            }
            Ok(Literal::Array(array))
        }
        _ => Err(anyhow::anyhow!("expected literal")),
    }
}