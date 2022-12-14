use interpreter_proc_macros::Interpretable;

use crate::{DeclarationStack, InterpreterError, InterpreterValue, VariableStack};

#[derive(Debug)]
pub struct Block(pub Vec<BlockItem>);

#[derive(Debug)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Statement),
    Directive(Directive),
}

#[derive(Debug, Interpretable)]
pub enum Declaration {
    Variable(Vec<Variable>),
    Function(Function),
    Class(Class),
    Namespace(Namespace),
}

#[derive(Debug)]
pub struct Variable {
    pub identifier: Identifier,
    pub expression: Option<Expression>,
}

#[derive(Debug)]
pub struct Function {
    pub identifier: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: Vec<BlockItem>,
}

#[derive(Debug)]
pub struct Parameter {
    pub identifier: Identifier,
    pub expression: Option<Expression>, //TODO: should be constant expression
}

#[derive(Debug)]
pub struct Class {
    pub identifier: Identifier,
    pub extends: Option<Name>,
    pub public_items: Vec<ClassItem>,
    pub private_items: Vec<ClassItem>,
    pub protected_items: Vec<ClassItem>,
}

#[derive(Debug)]
pub enum ClassItem {
    Constructor(Constructor),
    StaticDeclaration(Declaration),
    Declaration(Declaration),
    Directive(Directive),
}

#[derive(Debug)]
pub struct Constructor {
    pub parameters: Vec<Parameter>,
    pub super_parameters: Vec<Expression>,
    pub body: Vec<BlockItem>,
}

#[derive(Debug)]
pub struct Namespace {
    pub identifier: Identifier,
    pub body: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum Statement {
    Block(Block),
    Assignment(Assignment),
    Expression(Expression),
    Condition(Condition),
    ForLoop(ForLoop),
    WhileDoLoop(WhileLoop),
    DoWhileLoop(WhileLoop),
    Break,
    Continue,
    Return(Option<Expression>),
    Throw(Expression),
    TryCatch(TryCatch),
}

#[derive(Debug)]
pub struct Assignment {
    pub left_hand_side: LeftHandSide,
    pub operation: AssignmentOperator,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum LeftHandSide {
    Name(Name),
    Item(ItemAccess),
    Member(MemberAccess),
}

#[derive(Debug)]
pub enum AssignmentOperator {
    Equals,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
}

#[derive(Debug)]
pub struct Condition {
    pub expression: Expression,
    pub do_then: Box<Statement>,
    pub do_else: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct ForLoop {
    pub loop_var: Option<ForLoopVariable>,
    pub expression: Expression,
    pub do_for_each: Box<Statement>,
}

#[derive(Debug)]
pub enum ForLoopVariable {
    Identifier(Identifier),
    Name(Name),
}

#[derive(Debug)]
pub struct WhileLoop {
    pub expression: Expression,
    pub do_while: Box<Statement>,
}

#[derive(Debug)]
pub struct TryCatch {
    pub do_try: Box<Statement>,
    pub error: Identifier,
    pub do_catch: Box<Statement>,
}

#[derive(Debug, Interpretable)]
pub enum Directive {
    Use(UseDirective),
}

#[derive(Debug)]
pub struct UseDirective {
    pub source: Option<Name>,
    pub imports: Vec<Import>,
}

#[derive(Debug)]
pub enum Import {
    Namespace(Name),
    Name {
        name: Name,
        alias: Option<Identifier>,
    },
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Identifier(pub String); //TODO: _, <letter>, (<letter/digit/_>)* that are not keywords

#[derive(Debug, PartialEq, Eq)]
pub struct Name(pub Vec<Identifier>);

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Group(Group),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    FunctionCall(FunctionCall),
    ItemAccess(ItemAccess),
    MemberAccess(MemberAccess),
    Name(Name),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Null,
    Boolean(bool),
    Integer(i32),
    Real(f64),
    String(String),
    Array(Vec<Expression>),
    Dictionary(Vec<(String, Expression)>), //Key is String or Identifier
    Lambda,                                //TODO: later :)
}

#[derive(Debug, PartialEq)]
pub enum Group {
    Rounded(Box<Expression>),
    Square(Box<Expression>),
    Curly(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct UnaryOperation {
    pub expression: Box<Expression>,
    pub operator: UnaryOperator,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: BinaryOperator,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    RDiv,
    IDiv,
    Mod,
    Pow,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    And,
    Or,
    Xor,
    Range,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub expression: Box<Expression>,
    pub arguments: Vec<FunctionCallArgument>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCallArgument {
    pub identifier: Option<Identifier>,
    pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ItemAccess {
    pub set: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct MemberAccess {
    pub expression: Box<Expression>,
    pub identifier: Identifier,
}
