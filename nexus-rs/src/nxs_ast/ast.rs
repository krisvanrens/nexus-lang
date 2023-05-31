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

/// General statement representation.
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
#[derive(Debug)]
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

impl fmt::Display for StmtKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO: Create macro for this? Or use EnumIter from Strum.
            StmtKind::Assignment(x) => write!(f, "AssignmentStmt {{ {x} }}"),
            StmtKind::Block(x) => write!(f, "BlockStmt {{ {x} }}"),
            StmtKind::Connect(x) => write!(f, "ConnectStmt {{ {x} }}"),
            StmtKind::ConstDecl(x) => write!(f, "ConstDeclStmt {{ {x} }}"),
            StmtKind::Expr(x) => write!(f, "ExprStmt {{ {x} }}"),
            StmtKind::FunctionDecl(x) => write!(f, "FunctionDeclStmt {{ {x} }}"),
            StmtKind::Print(x) => write!(f, "PrintStmt {{ {x} }}"),
            StmtKind::Return(x) => write!(f, "ReturnStmt {{ {x} }}"),
            StmtKind::UseDecl(x) => write!(f, "UseDeclStmt {{ {x} }}"),
            StmtKind::VarDecl(x) => write!(f, "VarDeclStmt {{ {x} }}"),
        }
    }
}

// TODO: Use newtype proper (see: https://github.com/apolitical/impl-display-for-vec)
#[derive(Debug)]
pub struct Stmts(pub Vec<Stmt>);

impl Stmts {
    pub fn new() -> Self {
        Stmts(Vec::<Stmt>::new())
    }

    pub fn push(&mut self, s: Stmt) {
        self.0.push(s)
    }

    pub fn inner(&mut self) -> &mut Vec<Stmt> {
        &mut self.0
    }
}

impl fmt::Display for Stmts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        write!(f, "TODO")
    }
}

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

impl fmt::Display for FunctionDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FunctionDecl {{ {} ({}) -> {} }}",
            self.id,
            match self.args {
                Some(_) => "..TODO..".to_owned(),
                None => "".to_owned(),
            },
            match &self.ret_type {
                Some(t) => format!("{t}"),
                None => "unknown".to_owned(),
            }
        )
    }
}

/// Function argument.
#[derive(Debug)]
pub struct FunctionArg {
    pub id: String,
    pub typeid: TypeKind,
}

impl fmt::Display for FunctionArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
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

impl fmt::Display for VarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VarDecl {{ {} {}: {} = {} }}",
            self.id,
            if self.mutable { "mut " } else { "" },
            match &self.typeid {
                Some(t) => format!("{t}"),
                None => "unknown".to_owned(),
            },
            match &self.value {
                Some(v) => format!("{v}"),
                None => "unknown".to_owned(),
            }
        )
    }
}

/// Using declaration.
#[derive(Debug)]
pub struct UseDecl {
    pub filename: Expr,
}

impl fmt::Display for UseDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UseDecl {}", self.filename)
    }
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
#[derive(Debug)]
pub enum ExprKind {
    Binary(Ptr<BinaryExpr>),
    Block(Ptr<BlockExpr>),
    Empty(),
    FuncCall(Ptr<FuncCall>),
    Group(Ptr<Expr>),
    Literal(Ptr<Literal>),
    Range(Ptr<Range>),
    Unary(Ptr<UnaryExpr>),
    Var(Ptr<Var>),
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::FuncCall(x) => write!(f, "FuncCallExpr {{ {x} }}"),
            ExprKind::Literal(x) => write!(f, "LiteralExpr {{ {x} }}"),
            ExprKind::Range(x) => write!(f, "RangeExpr {{ {x} }}"),
            ExprKind::Var(x) => write!(f, "VarExpr {{ {x} }}"),
            _ => write!(f, "TODO"),
        }
    }
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
pub struct FuncCall {
    pub id: String,
    pub args: Vec<Expr>,
}

impl fmt::Display for FuncCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FuncCall {{ {} (..TODO..) }}", self.id)
    }
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

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        write!(f, "Assignment TODO")
    }
}

/// Connect statement.
#[derive(Debug)]
pub struct Connect {
    pub source: Expr,
    pub sink: Expr,
}

impl fmt::Display for Connect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        write!(f, "Connect TODO")
    }
}

/// Print statement.
#[derive(Debug)]
pub struct Print {
    pub expr: Expr,
}

impl fmt::Display for Print {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        write!(f, "Print TODO")
    }
}

/// Return statement.
#[derive(Debug)]
pub struct Return {
    pub expr: Expr,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        write!(f, "Return TODO")
    }
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
#[derive(Debug)]
pub enum LiteralKind {
    Bool(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralKind::Bool(x) => write!(f, "Bool {{ {x} }}"),
            LiteralKind::Number(x) => write!(f, "Number {{ {x} }}"),
            LiteralKind::String(x) => write!(f, "String {{ {x} }}"),
        }
    }
}

/// Range expression.
#[derive(Debug)]
pub struct Range {
    pub kind: RangeKind,
    pub start: Expr,
    pub end: Expr,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Range {{ {} {} {} }}", self.start, self.kind, self.end)
    }
}

/// Range kind.
#[derive(Debug)]
pub enum RangeKind {
    Exclusive,
    Inclusive,
}

impl fmt::Display for RangeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RangeKind::Exclusive => "..",
                RangeKind::Inclusive => "..=",
            }
        )
    }
}

/// Variable expression.
#[derive(Debug)]
pub struct Var {
    pub id: String,
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var {{ {} }}", self.id)
    }
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
