use crate::ptr::Ptr;

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

/// Statement kind.
#[derive(Debug)]
pub enum StmtKind {
    Block(Stmts),
    ConstDecl(Ptr<ConstDecl>),
    Expr(Ptr<Expr>),
    FunctionDecl(Ptr<FunctionDecl>),
    Print(Ptr<Print>),
    VarDecl(Ptr<VarDecl>),
}

/// A collection of statements.
pub type Stmts = Vec<Stmt>;

/// Nexus fundamental type kind.
#[derive(Debug)]
pub enum TypeKind {
    Bool,
    Number,
    String,
}

#[derive(Debug)]
pub struct ConstDecl {
    pub id: String,
    pub typeid: TypeKind,
    pub value: Literal,
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub id: String,
    pub args: Option<FunctionArgs>,
    pub ret_type: Option<TypeKind>,
    pub body: Stmt, // A block statement.
}

#[derive(Debug)]
pub struct FunctionArg {
    pub id: String,
    pub typeid: TypeKind,
}

pub type FunctionArgs = Vec<FunctionArg>;

#[derive(Debug)]
pub struct VarDecl {
    pub id: String,
    pub mutable: bool,
    pub typeid: Option<TypeKind>,
    pub value: Option<Expr>,
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
    Binary,
    Empty,
    Group,
    Literal,
    Unary,
}

#[derive(Debug)]
pub struct Group {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Node {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Print {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Literal {
    pub kind: LiteralKind,
}

#[derive(Debug)]
pub enum LiteralKind {
    Bool(bool),
    Number(f64),
    String(String),
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
