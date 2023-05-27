use core::fmt;
use strum_macros::Display;

use super::ptr::Ptr;

/// AST node evaluation trait, used for simple evaluation.
///
/// Because recursive evaluation via traits is not possible, we use open recursion here
///  using the 'eval_expr' callable for subexpression evaluation.
pub trait Eval<R, E> {
    fn eval<F>(&self, eval_expr: F) -> R
    where
        F: FnMut(&E) -> R;
}

/// Statement representation.
#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stmt[{}]", self.kind)
    }
}

/// Statement kind.
#[derive(Debug, Display)]
pub enum StmtKind {
    Assignment(Ptr<Assignment>),
    Block(Stmts),
    Connect(Ptr<Connect>),
    ConstDecl(Ptr<ConstDecl>),
    Expr(Ptr<Expr>),
    FunctionDecl(Ptr<FunctionDecl>),
    Print(Ptr<Print>),
    Return(Ptr<Return>),
    UseDecl(Ptr<UseDecl>),
    VarDecl(Ptr<VarDecl>),
}

/// A collection of statements.
pub type Stmts = Vec<Stmt>;

/// Nexus fundamental type kind.
#[derive(Debug, Display)]
pub enum TypeKind {
    Bool,
    Group,
    Node,
    Number,
    String,
}

/// Constant declaration.
#[derive(Debug)]
pub struct ConstDecl {
    pub id: String,
    pub typeid: TypeKind,
    pub value: Expr,
}

impl fmt::Display for ConstDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Const({}: {})", self.typeid, self.value)
    }
}

/// Function declaration.
#[derive(Debug)]
pub struct FunctionDecl {
    pub id: String,
    pub args: Option<FunctionArgs>,
    pub ret_type: Option<TypeKind>,
    pub body: Stmt, // A block statement.
}

/// Function argument.
#[derive(Debug)]
pub struct FunctionArg {
    pub id: String,
    pub typeid: TypeKind,
}

/// A collection of function arguments.
pub type FunctionArgs = Vec<FunctionArg>;

/// Variable declaration.
#[derive(Debug)]
pub struct VarDecl {
    pub id: String,
    pub mutable: bool,
    pub typeid: Option<TypeKind>,
    pub value: Option<Expr>,
}

/// Using declaration.
#[derive(Debug)]
pub struct UseDecl {
    pub filename: Expr,
}

/// General expression representation.
#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

/// Expression kind.
#[derive(Debug, Display)]
pub enum ExprKind {
    Binary(Ptr<BinaryExpr>),
    Block(Ptr<BlockExpr>),
    FuncCall(Ptr<FuncCallExpr>),
    Group(Ptr<Expr>),
    Literal(Ptr<Literal>),
    Unary(Ptr<UnaryExpr>),
    Var(Ptr<Var>),
}

/// Binary expression.
#[derive(Debug)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

/// Binary operator.
#[derive(Debug, Display)]
pub enum BinaryOp {
    And,
    Divide,
    Dot,
    Eq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Multiply,
    NotEq,
    Or,
    Plus,
    Remainder,
    Subtract,
}

/// Blocking expression.
#[derive(Debug)]
pub struct BlockExpr {
    pub body: Stmt,
}

/// Function call expression.
#[derive(Debug)]
pub struct FuncCallExpr {
    pub id: String,
    pub args: Vec<Expr>,
}

/// Unary expression.
#[derive(Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Expr,
}

/// Unary operator.
#[derive(Debug, Display)]
pub enum UnaryOp {
    Bang,
    Group,
    Minus,
    Node,
    Plus,
}

/// Assignment statement.
#[derive(Debug)]
pub struct Assignment {
    pub lhs: Expr,
    pub rhs: Expr,
}

/// Connect statement.
#[derive(Debug)]
pub struct Connect {
    pub source: Expr,
    pub sink: Expr,
}

/// Print statement.
#[derive(Debug)]
pub struct Print {
    pub expr: Expr,
}

/// Return statement.
#[derive(Debug)]
pub struct Return {
    pub expr: Expr,
}

/// Literal value.
#[derive(Debug)]
pub struct Literal {
    pub kind: LiteralKind,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

/// Literal value kind.
#[derive(Debug, Display)]
pub enum LiteralKind {
    Bool(bool),
    Number(f64),
    String(String),
}

/// Variable expression.
#[derive(Debug)]
pub struct Var {
    pub id: String,
}

/// Trait to enable self-evaluation.
trait Evaluate: Sized {
    fn evaluate<R>(&self) -> R
    where
        Self: Eval<R, Self>;
}

impl<E> Evaluate for E
where
    E: Sized,
{
    fn evaluate<R>(&self) -> R
    where
        Self: Eval<R, Self>,
    {
        self.eval(|e| e.evaluate())
    }
}

#[test]
fn evaluate_literals() {
    // TODO
    //let l1 = BooleanLiteral { value: true };
    //let l2 = NumberLiteral { value: 3.1415 };
    //let l3 = StringLiteral {
    //    value: "Hello 123".to_string(),
    //};

    //assert_eq!(l1.evaluate::<bool>(), true);
    //assert!(l2.evaluate::<f64>() - 3.1415 < 0.001);
    //assert_eq!(l3.evaluate::<String>(), "Hello 123".to_string());
}
