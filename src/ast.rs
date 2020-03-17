#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub constants: Vec<Constant>,
    pub enums: Vec<EnumDefinition>,
    pub structs: Vec<StructDefinition>,
    pub ranges: Vec<RangeDefinition>,
    pub callables: Vec<CallableDefinition>,
    pub main: Option<MainDefinition>,
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

#[derive(Debug)]
pub struct Constant {
    pub field: FieldDefinition,
    pub value: Expression
}

#[derive(Debug)]
pub struct EnumDefinition {
    pub name: String,
    pub base: String,
    pub items: Vec<EnumItemDefinition>
}

#[derive(Debug)]
pub struct EnumItemDefinition {
    pub name: String,
}

#[derive(Debug)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>
}

#[derive(Debug)]
pub struct RangeDefinition {
    pub name: String,
    pub base: String,
}

#[derive(Debug)]
pub enum CallType {
    Proc,
    Macro
}

#[derive(Debug)]
pub struct CallableDefinition {
    pub name: String,
    pub call_type: CallType,
    pub parameters: Vec<ParameterDefinition>,
    pub return_type: Option<String>,
    pub statements: Vec<Statement>,
    pub attributes: Vec<Attribute>
}

#[derive(Debug)]
pub struct ParameterDefinition {
    pub field: FieldDefinition,
    pub copy: bool
}

#[derive(Debug)]
pub struct MainDefinition {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub struct FieldDefinition {
    pub name: String,
    pub typename: String
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub parameters: Vec<Expression>
}

#[derive(Debug)]
pub enum Statement {
    Todo,
    Call(String, Vec<Expression>),
    If(Expression, Vec<Statement>, Vec<Statement>),
    Assign(String, Expression),
    ExternalAssign(String, Expression),
}

#[derive(Debug)]
pub enum Expression {
    ConstNumber(String),
    ConstString(String),
    FieldAccess(String),
    ExternalFieldAccess(String),
    Negate(Box<Expression>),
    Not(Box<Expression>),

    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Exponent(Box<Expression>, Box<Expression>),
    Bracket(Box<Expression>),

    Equals(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
}