
pub mod ast {
    #[derive(Debug, Clone)]
    pub enum UnaryOp {
        Neg,
    }
    #[derive(Debug, Clone)]
    pub enum BinaryOp {
        Plus,
        Minus,
        Multiply,
        Divide,
        Modulo,
    }

    #[derive(Debug, Clone)]
    pub enum CompOp {
        Less,
        LessEq,
        Greater,
        GreaterEq,
        Equal,
    }

    #[derive(Debug, Clone)]
    pub enum Expression {
        Function(Func),
        UnaryOperation(UnaryOp, Box<Expression>),
        BinaryOperation(BinaryOp, Box<Expression>, Box<Expression>),
        Variable(String),
        Number(f64),
        Word(String),
    }

    #[derive(Debug, Clone)]
    pub enum Argumnet {
        List(Vec<Func>),
        Comp(CompOp, Box<Expression>, Box<Expression>),
        Expr(Box<Expression>),
    }

    #[derive(Debug, Clone)]
    pub enum Value {
        Number(f64),
        Word(String),
        Boolean(bool),
        Instructions(Vec<Func>)
    }

    #[derive(Debug, Clone)]
    pub struct Func {
        pub name: String,
        pub args: Vec<Argumnet>
    }

    #[derive(Debug, Clone)]
    pub struct FuncDeclaration {
        pub name: String,
        pub args: Vec<String>,
        pub instructions: Vec<Func>,
    }

    #[derive(Debug, Clone)]
    pub struct Logo {
        pub functions: Vec<FuncDeclaration>,
        pub instructions: Vec<Func>,
    }

    pub fn cons<A>(mut v: Vec<A>, x: A) -> Vec<A> {
        v.push(x);
        v
    } 
}