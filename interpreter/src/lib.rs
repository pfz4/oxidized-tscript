use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use ast::{
    BinaryOperator, Block, BlockItem, Class, Declaration, Expression, Function, FunctionCall,
    FunctionCallArgument, Identifier, LeftHandSide, Name, Namespace, Statement, UseDirective,
    Variable,
};

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub tscript);

mod ast;
mod test;

type DeclarationStack<'a> = Vec<HashMap<&'a Identifier, DeclStackItem<'a>>>;
type VariableStack<'a> = Vec<HashMap<&'a Identifier, ()>>;

pub enum DeclStackItem<'a> {
    Function(&'a Function),
    Class(&'a Class),
    Namespace(&'a Namespace),
}

#[derive(Debug)]
pub enum InterpreterError {
    ConflictWithPreviousDeclaration(Identifier),
    OperationNotPossible(String),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TValue {
    Null,
    Boolean(bool),
    Number(i32),
    Real(f64),
    String(String),
    Array(Vec<TValue>),
    Dictionary(HashMap<String, TValue>),
    Object,
}

fn run() {
    let program: Vec<BlockItem> = vec![
        BlockItem::Declaration(Declaration::Variable(vec![Variable {
            identifier: Identifier("test".to_string()),
            expression: Some(Expression::Literal(ast::Literal::Integer(69))),
        }])),
        BlockItem::Statement(Statement::Expression(Expression::FunctionCall(
            ast::FunctionCall {
                expression: Box::new(Expression::Name(Name(vec![Identifier(
                    "print".to_string(),
                )]))),
                arguments: vec![FunctionCallArgument {
                    identifier: None,
                    expression: Expression::Name(ast::Name(vec![Identifier("test".to_string())])),
                }],
            },
        ))),
    ];
    let mut declaration_stack: DeclarationStack = vec![];
    let mut variable_stack: VariableStack = vec![];

    Block(program)
        .interpret(&mut declaration_stack, &mut variable_stack)
        .unwrap();
}

pub type InterpreterValue = Option<TValue>;
pub trait Interpretable {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError>;
}

impl TValue {
    fn binary_operate(
        &self,
        other: &Self,
        operator: &BinaryOperator,
    ) -> Result<Self, InterpreterError> {
        match (self, other, operator) {
            (l, r, BinaryOperator::Eq) => Ok(Self::Boolean(l == r)),
            (l, r, BinaryOperator::Neq) => Ok(Self::Boolean(l != r)),
            (l, r, BinaryOperator::Lt) => Ok(Self::Boolean(l < r)),
            (l, r, BinaryOperator::Gt) => Ok(Self::Boolean(l > r)),
            (l, r, BinaryOperator::Leq) => Ok(Self::Boolean(l <= r)),
            (l, r, BinaryOperator::Geq) => Ok(Self::Boolean(l >= r)),

            _ => Err(InterpreterError::OperationNotPossible(format!(
                "cannot {:?} '{:?}' and '{:?}'",
                operator, self, other,
            ))),
        }
    }
}

impl Interpretable for Block {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        declaration_stack.push(HashMap::new());
        let declaration_map = declaration_stack.last_mut().unwrap();
        variable_stack.push(HashMap::new());

        // Parse Declarations
        for decl in self.0.iter().filter_map(|x| match x {
            BlockItem::Declaration(Declaration::Variable(_)) => None,
            BlockItem::Declaration(d) => Some(d),
            _ => None, //this step only parses declarations (except variables)
        }) {
            decl.interpret(declaration_stack, variable_stack)?;
        }

        for block_item in self.0.iter() {
            match block_item {
                BlockItem::Declaration(Declaration::Variable(vars)) => {
                    vars.interpret(declaration_stack, variable_stack)?;
                }
                BlockItem::Declaration(_) => {} //handled previously
                BlockItem::Statement(statement) => {
                    statement.interpret(declaration_stack, variable_stack)?;
                }
                BlockItem::Directive(directive) => {
                    directive.interpret(declaration_stack, variable_stack)?;
                }
            }
        }

        variable_stack.pop();
        declaration_stack.pop();
        Ok(None)
    }
}

impl Interpretable for UseDirective {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        todo!()
    }
}

impl Interpretable for Function {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        let declaration_map = declaration_stack.last_mut().unwrap();
        match declaration_map.contains_key(&self.identifier) {
            false => {
                declaration_map.insert(&self.identifier, DeclStackItem::Function(self));
                Ok(None)
            }
            true => Err(InterpreterError::ConflictWithPreviousDeclaration(
                self.identifier.clone(),
            )),
        }
    }
}

impl Interpretable for Class {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        let declaration_map = declaration_stack.last_mut().unwrap();
        match declaration_map.contains_key(&self.identifier) {
            false => {
                declaration_map.insert(&self.identifier, DeclStackItem::Class(self));
                Ok(None)
            }
            true => Err(InterpreterError::ConflictWithPreviousDeclaration(
                self.identifier.clone(),
            )),
        }
    }
}

impl Interpretable for Namespace {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        let declaration_map = declaration_stack.last_mut().unwrap();
        match declaration_map.contains_key(&self.identifier) {
            false => {
                declaration_map.insert(&self.identifier, DeclStackItem::Namespace(self));
                Ok(None)
            }
            true => Err(InterpreterError::ConflictWithPreviousDeclaration(
                self.identifier.clone(),
            )),
        }
    }
}

impl Interpretable for Vec<Variable> {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        println!("{:?}", self);
        let variable_map = variable_stack.last_mut().unwrap();
        for var in self {
            match variable_map.contains_key(&var.identifier) {
                false => {
                    variable_map.insert(&var.identifier, ());
                }
                true => {
                    return Err(InterpreterError::ConflictWithPreviousDeclaration(
                        var.identifier.clone(),
                    ))
                }
            }
        }
        Ok(None)
    }
}

impl Interpretable for Statement {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        match self {
            Self::Block(block) => block.interpret(declaration_stack, variable_stack),
            Self::Expression(expr) => expr.interpret(declaration_stack, variable_stack),
            _ => todo!(),
        }
    }
}

impl Interpretable for Expression {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        match self {
            Self::FunctionCall(funccall) => funccall.interpret(declaration_stack, variable_stack),
            _ => todo!(),
        }
    }
}

impl Interpretable for FunctionCall {
    fn interpret<'a, 'b>(
        &'a self,
        declaration_stack: &'b mut DeclarationStack<'a>,
        variable_stack: &'b mut VariableStack<'a>,
    ) -> Result<InterpreterValue, InterpreterError> {
        if *self.expression == Expression::Name(Name(vec![Identifier("print".to_string())])) {
            println!("Hello World: {:?}", self.arguments);
        } else {
            todo!();
        }
        Ok(None)
    }
}
