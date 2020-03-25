#[derive(Debug, Clone)]
pub struct Program {
    pub lines: Vec<Line>
}

#[derive(Debug, Clone)]
pub struct Line {
    pub label: Option<String>,
    pub statements: StatementList
}

#[derive(Debug, Clone)]
pub struct StatementList {
    pub statements: Vec<Statement>
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Identifier, Expression),
    CompoundAssignment(Identifier, Op, Expression),
    Empty(),
    ExpressionWrapper(Expression),
    Goto(Expression),
    GotoLabel(String),
    If(Expression, Box<StatementList>, Box<StatementList>)
}

#[derive(Debug, Clone)]
pub enum Expression {
    ConstantNumber(String),        //todo: use yolol_number
    ConstantString(String),
    VariableAccess(Identifier),

    ACos(Box<Expression>),
    ASin(Box<Expression>),
    ATan(Box<Expression>),
    Sqrt(Box<Expression>),
    Cosine(Box<Expression>),
    Sine(Box<Expression>),
    Tangent(Box<Expression>),
    Brackets(Box<Expression>),
    Abs(Box<Expression>),
    Negate(Box<Expression>),
    Not(Box<Expression>),
    PostDecrement(Identifier),
    PostIncrement(Identifier),
    PreDecrement(Identifier),
    PreIncrement(Identifier),
    
    Add(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    EqualTo(Box<Expression>, Box<Expression>),
    Exponent(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    GreaterThanEqualTo(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    LessThanEqualTo(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>)
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub external: bool,
}