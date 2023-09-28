use std::collections::HashMap;

use crate::{
    lexer::TokenKind,
    parser::{Ast, Binary, Binding},
};

#[derive(Debug, Clone, Copy)]
pub enum ConstValue {
    Number(i32),
}

#[derive(Debug)]
pub struct Scope<'a> {
    level: usize,
    items: HashMap<&'a str, ConstValue>,
}

impl<'a> Scope<'a> {
    fn add(&mut self, identifier: &'a str, value: ConstValue) {
        self.items.insert(identifier, value);
    }
}

pub type ConstScopes<'a> = Vec<Scope<'a>>;

#[derive(Debug, Clone)]
struct ScopeRef {
    last: Option<Box<ScopeRef>>,
    current: usize,
}

pub struct ConstEvaluator<'a> {
    current_level: usize,
    scopes: ConstScopes<'a>,
    current_scope: Option<ScopeRef>,
}

impl<'a> ConstEvaluator<'a> {
    pub fn new() -> Self {
        Self {
            current_level: 0,
            scopes: Vec::new(),
            current_scope: None,
        }
    }

    pub fn eval(mut self, root: &'a Box<Ast<'a>>) -> ConstScopes<'a> {
        self.visit(root);

        let ConstEvaluator { scopes, .. } = self;
        scopes
    }

    fn new_scope(&mut self) {
        self.scopes.push(Scope {
            level: self.current_level,
            items: HashMap::new(),
        });
        self.current_level += 1;

        self.current_scope = Some(ScopeRef {
            last: self.current_scope.take().map(|sr| Box::new(sr)),
            current: self.scopes.len() - 1,
        });
    }

    fn close_scope(&mut self) {
        self.current_level -= 1;

        if let Some(last) = &self.current_scope.as_ref().unwrap().last {
            self.current_scope = Some(*last.clone());
        }
    }

    fn current_scope(&self) -> usize {
        self.current_scope.as_ref().unwrap().current
    }

    fn visit(&mut self, item: &'a Box<Ast<'a>>) {
        match &**item {
            Ast::Binding(b) => self.binding(b),
            Ast::Body(items) => self.body(items),
            _ => {}
        }
    }

    fn binding(&mut self, binding: &'a Binding) {
        let expr_value = self.eval_expr(&binding.expr);

        let index = self.current_scope();
        self.scopes[index].add(binding.identifier.lexeme.unwrap(), expr_value);
    }

    fn body(&mut self, items: &'a Vec<Box<Ast<'a>>>) {
        self.new_scope();

        for item in items {
            self.visit(item);
        }

        self.close_scope();
    }

    fn find_binding_value(&self, identifier: &'a str) -> Option<ConstValue> {
        for scope in self.scopes.iter().rev() {
            if scope.level > self.current_level {
                continue;
            }

            if let Some(v) = scope.items.get(identifier) {
                return Some(*v);
            }
        }

        None
    }

    fn eval_expr(&self, item: &Box<Ast<'a>>) -> ConstValue {
        match &**item {
            Ast::Literal(lit) => ConstValue::Number(lit.lexeme.unwrap().parse::<i32>().unwrap()),
            Ast::Binary(bin) => self.eval_binary_op(bin),
            Ast::Identifier(id) => self.find_binding_value(id.lexeme.unwrap()).unwrap(),
            e => panic!("Unknown item in eval_expr '{e:?}'"),
        }
    }

    fn eval_binary_op(&self, binary: &Binary) -> ConstValue {
        let rhs = self.eval_expr(&binary.rhs);
        let lhs = self.eval_expr(&binary.lhs);

        match (lhs, rhs) {
            (ConstValue::Number(l), ConstValue::Number(r)) => {
                self.eval_num_binary(binary.op.kind, l, r)
            }
            e => panic!("Unknown value in evaluation '{e:?}'"),
        }
    }

    fn eval_num_binary(&self, op: TokenKind, lhs: i32, rhs: i32) -> ConstValue {
        match op {
            TokenKind::Plus => ConstValue::Number(lhs + rhs),
            TokenKind::Minus => ConstValue::Number(lhs - rhs),
            TokenKind::Star => ConstValue::Number(lhs * rhs),
            TokenKind::Slash => ConstValue::Number(lhs / rhs),
            _ => panic!("Unknown operator '{op:?}'"),
        }
    }
}
