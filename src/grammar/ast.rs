#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub constants: Vec<Constant>,
    pub enums: Vec<EnumDefinition>,
    pub structs: Vec<StructDefinition>,
    pub ranges: Vec<RangeDefinition>,
    pub callables: Vec<CallableDefinition>,
    pub main: Option<Main>,
}

impl Program {
    pub fn combine(mut self, mut b: Self, namespace: Option<String>) -> Self {

        if let Some(ns) = namespace {
            b.apply_namespace(&ns);
        }

        self.imports.append(&mut b.imports);
        self.constants.append(&mut b.constants);
        self.enums.append(&mut b.enums);
        self.structs.append(&mut b.structs);
        self.ranges.append(&mut b.ranges);
        self.callables.append(&mut b.callables);

        self.main = self.main.or(b.main);

        return self;
    }

    pub fn clear_imports(mut self) -> Self {
        self.imports.clear();
        return self;
    }

    fn apply_namespace(&mut self, ns: &str) {

        fn apply<T, FA, FG>(ns: &str, items: &mut Vec<T>, get: FG, app: FA)
            where FG: Fn(&T) -> String,
                  FA: Fn(&mut T, String) -> ()
        {
            for i in items.iter_mut() {
                let n = get(&i);
                app(i, format!("{}:{}", ns, n));
            }
        }


        apply(ns, &mut self.constants, |x| x.field.name.clone(), |a, b| a.field.name = b);
        apply(ns, &mut self.enums, |x| x.name.clone(), |a, b| a.name = b);
        apply(ns, &mut self.structs, |x| x.name.clone(), |a, b| a.name = b);
        apply(ns, &mut self.ranges, |x| x.name.clone(), |a, b| a.name = b);
        apply(ns, &mut self.callables, |x| x.name.clone(), |a, b| a.name = b);
    }
}

#[derive(Clone, Debug)]
pub struct Import {
    pub path: String,
    pub namespace: Option<String>
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub field: FieldDefinition,
    pub value: Expression
}

#[derive(Debug, Clone)]
pub struct EnumDefinition {
    pub name: String,
    pub base: String,
    pub items: Vec<EnumItemDefinition>
}

#[derive(Debug, Clone)]
pub struct EnumItemDefinition {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>
}

#[derive(Debug, Clone)]
pub struct RangeDefinition {
    pub name: String,
    pub base: String,
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub enum CallType {
    Proc,
    Macro
}

#[derive(Debug, Clone)]
pub struct CallableDefinition {
    pub name: String,
    pub call_type: CallType,
    pub parameters: Vec<ParameterDefinition>,
    pub return_type: Option<String>,
    pub statements: Vec<InnerStatement>,
    pub attributes: Vec<Attribute>
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ParameterDefinition {
    pub field: FieldDefinition,
    pub copy: bool
}

#[derive(Debug, Clone)]
pub struct Main {
    pub statements: Vec<OuterStatement>
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FieldDefinition {
    pub name: String,
    pub typename: TypeName
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TypeName {
    pub typename: String
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub parameters: Vec<Expression>
}

#[derive(Debug, Clone)]
pub enum OuterStatement {
    //Loop(Vec<OuterStatement>),
    Line(Vec<InnerStatement>, Option<String>),
    Inner(InnerStatement),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum InnerStatement {
    CompilePanic(String, usize),

    Emit(String),

    Call(String, Vec<Expression>),
    If(Expression, Vec<InnerStatement>, Vec<InnerStatement>),

    Assign(Vec<String>, Expression),
    DeclareAssign(FieldDefinition, Expression),
    DeclareConst(FieldDefinition, Expression),
    ExternalAssign(String, Expression),

    Return(Expression),
    Goto(String)
}

#[derive(Debug, Clone)]
pub enum Expression {
    CompilePanic(String, usize),

    ConstNumber(String),
    ConstString(String),
    FieldAccess(Vec<String>),
    ExternalFieldAccess(String),
    Negate(Box<Expression>),
    Not(Box<Expression>),
    Call(String, Vec<Expression>),

    Is(Box<Expression>, String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulus(Box<Expression>, Box<Expression>),
    Exponent(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Bracket(Box<Expression>),

    GreaterThan(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    GreaterThanOrEq(Box<Expression>, Box<Expression>),
    LessThanOrEq(Box<Expression>, Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),

    PostIncrement(String),
    PostDecrement(String),
    PreIncrement(String),
    PreDecrement(String),

    Constructor(TypeName, Vec<(String, Expression)>),
}