use std::{fmt, ops};
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
        write!(f, "{}", self.kind)
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

/// A collection of statements.
///
/// By way of the orphan rule, we are not allowed to implement a foreign trait on a foreign type.
/// That's why we use the newtype pattern here, and introduce a single-field tuple.
#[derive(Debug)]
pub struct Stmts(pub Vec<Stmt>);

impl Stmts {
    pub fn new() -> Self {
        Stmts(Vec::<Stmt>::new())
    }
}

impl Default for Stmts {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Deref for Stmts {
    type Target = Vec<Stmt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Stmts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Stmts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.0.is_empty() {
                "(empty)".to_owned()
            } else {
                self.0
                    .iter()
                    .map(|s| format!("{s}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            }
        )
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
            "FunctionDecl {{ {} ({}) -> {} {{ {} }} }}",
            self.id,
            match &self.args {
                Some(a) => format!("{a}"),
                None => "".to_owned(),
            },
            match &self.ret_type {
                Some(t) => format!("{t}"),
                None => "unknown".to_owned(),
            },
            self.body
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
        write!(f, "Arg {{ {} : {} }}", self.id, self.typeid)
    }
}

/// A collection of function arguments.
///
/// By way of the orphan rule, we are not allowed to implement a foreign trait on a foreign type.
/// That's why we use the newtype pattern here, and introduce a single-field tuple.
#[derive(Debug)]
pub struct FunctionArgs(pub Vec<FunctionArg>);

impl FunctionArgs {
    pub fn new() -> Self {
        FunctionArgs(Vec::new())
    }
}

impl Default for FunctionArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Deref for FunctionArgs {
    type Target = Vec<FunctionArg>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for FunctionArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for FunctionArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.0.is_empty() {
                "(empty)".to_owned()
            } else {
                self.0
                    .iter()
                    .map(|s| format!("{s}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            }
        )
    }
}

/// Variable declaration.
#[derive(Debug)]
pub struct VarDecl {
    pub id: Expr,
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
    For(Ptr<For>),
    FuncCall(Ptr<FuncCall>),
    Group(Ptr<Expr>),
    If(Ptr<If>),
    Literal(Ptr<Literal>),
    Range(Ptr<Range>),
    Ref(Ptr<Ref>),
    Unary(Ptr<UnaryExpr>),
    Var(Ptr<Var>),
    While(Ptr<While>),
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Binary(x) => write!(f, "BinaryExpr {{ {x} }}"),
            ExprKind::Block(x) => write!(f, "BlockExpr {{ {x} }}"),
            ExprKind::Empty() => write!(f, "EmptyExpr"),
            ExprKind::For(x) => write!(f, "ForExpr {{ {x} }}"),
            ExprKind::FuncCall(x) => write!(f, "FuncCallExpr {{ {x} }}"),
            ExprKind::Group(x) => write!(f, "GroupExpr {{ ( {x} ) }}"),
            ExprKind::If(x) => write!(f, "IfExpr {{ {x} }}"),
            ExprKind::Literal(x) => write!(f, "LiteralExpr {{ {x} }}"),
            ExprKind::Range(x) => write!(f, "RangeExpr {{ {x} }}"),
            ExprKind::Ref(x) => write!(f, "RefExpr {{ {x} }}"),
            ExprKind::Unary(x) => write!(f, "UnaryExpr {{ {x} }}"),
            ExprKind::Var(x) => write!(f, "VarExpr {{ {x} }}"),
            ExprKind::While(x) => write!(f, "WhileExpr {{ {x} }}"),
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

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
    }
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

impl fmt::Display for BlockExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

/// For expression.
#[derive(Debug)]
pub struct For {
    pub id: String,
    pub expr: Expr,
    pub body: Expr,
}

impl fmt::Display for For {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "for {{ {} }} in {{ {} }} do {{ {} }}",
            self.id, self.expr, self.body
        )
    }
}

/// Function call expression.
#[derive(Debug)]
pub struct FuncCall {
    pub id: String,
    pub args: Vec<Expr>,
}

impl fmt::Display for FuncCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FuncCall {{ {} ({}) }}",
            self.id,
            self.args
                .iter()
                .map(|a| format!("{a}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// Unary expression.
#[derive(Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Expr,
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.op, self.expr)
    }
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
        write!(f, "Assignment {{ {} = {} }}", self.lhs, self.rhs)
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
        write!(f, "Connect {{ {} -> {} }}", self.source, self.sink)
    }
}

/// Print statement.
#[derive(Debug)]
pub struct Print {
    pub expr: Expr,
}

impl fmt::Display for Print {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Print {{ {} }}", self.expr)
    }
}

/// Return statement.
#[derive(Debug)]
pub struct Return {
    pub expr: Expr,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return {{ {} }}", self.expr)
    }
}

/// If expression.
#[derive(Debug)]
pub struct If {
    pub expr: Expr,
    pub body_then: Expr,
    pub body_else: Option<Expr>,
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if {{ {} }} then {{ {} }}{}",
            self.expr,
            self.body_then,
            if let Some(e) = &self.body_else {
                format!(" else {{ {} }}", e)
            } else {
                "".to_owned()
            }
        )
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
            LiteralKind::String(x) => write!(f, "String {{ \"{x}\" }}"),
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

/// Ref expression.
#[derive(Debug)]
pub struct Ref {
    pub expr: Expr,
}

impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ref {{ {} }}", self.expr)
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

/// While expression.
#[derive(Debug)]
pub struct While {
    pub expr: Expr,
    pub body: Expr,
}

impl fmt::Display for While {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while {{ {} }} do {{ {} }}", self.expr, self.body)
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
