use super::ast::*;

// C parser grammar: https://github.com/vickenty/lang-c/blob/master/grammar.rustpeg

#[derive(Debug)]
enum Type {
    Enum(EnumDefinition),
    Struct(StructDefinition),
    Range(RangeDefinition)
}

fn split_types(types : Vec<Type>) -> (Vec<EnumDefinition>, Vec<StructDefinition>, Vec<RangeDefinition>) {
    let mut enums = Vec::new();
    let mut structs = Vec::new();
    let mut ranges = Vec::new();

    for t in types.into_iter() {
        match t {
            Type::Enum(x) => enums.push(x),
            Type::Struct(x) => structs.push(x),
            Type::Range(x) => ranges.push(x)
        }
    }

    return (enums, structs, ranges);
}

peg::parser!{

    pub grammar y_parser() for str {

        pub rule program() -> Program
            = __ i:import()* __ t:typedef()* __ con:constant()* __ c:callable()* __ m:main()? __
            {
                let (enums, structs, ranges) = split_types(t);

                Program {
                    imports: i,
                    constants: con,
                    enums: enums,
                    structs: structs,
                    ranges: ranges,
                    callables: c,
                    main: m
                }
            }

        rule import() -> Import
            = "import" __ "\"" p:path() "\"" __ n:("in" __ n:identifier() { n })? __ ";" __
            { Import { path: p, namespace: n } }
            / expected!("Import Definition")


        rule typedef() -> Type
            = "type" __ t:(
                e:enumdef() { Type::Enum(e) }
                / s:structdef() { Type::Struct(s)}
                / r:rangedef() { Type::Range(r) }
            ) __
            { t }

        rule constant() -> Constant
            = "const" __ f:field() __ "=" __ e:expression() __ ";" __
            { Constant {
                field: f,
                value: e
            } }

        rule enumdef() -> EnumDefinition
            = "enum" __ "<" __ b:identifier() __ ">" __ n:identifier() __ "{" __ e:(enum_item() ** ("," __)) __ "}"
            { EnumDefinition { name: n, base: b, items: e } }

        rule enum_item() -> EnumItemDefinition
            = i:identifier() __ "(" __ "todo_enum_value" __ ")"
            { EnumItemDefinition { name: i } }

        rule rangedef() -> RangeDefinition
            = "range" __ "<" __ b:identifier() __ ">" __ n:identifier() __ "->" __  e:expression() __ ";"
            { RangeDefinition { name: n, base: b, expression: e } }

        rule structdef() -> StructDefinition
            = "struct" __ i:identifier() __ "{" __ f:(field() ** ("," __)) __ ","? __ "}"
            { StructDefinition { name: i, fields: f } }



        rule callable() -> CallableDefinition
            = at:attributes()? __ "def" __ c:call_type() __ n:identifier() __ a:arglist() __ r:("->" __ i:identifier() { i })? __ "{" __ s:statement_list() __ "}" __
            { CallableDefinition {
                call_type: c,
                name: n,
                parameters: a,
                return_type: r,
                statements: s,
                attributes: at.unwrap_or(Vec::new())
            } }

        rule arglist() -> Vec<ParameterDefinition>
            = "(" __ a:(arg() ** ("," __)) __ ")"
            { a }
        
        rule arg() -> ParameterDefinition
            = c:"copy "? f:field()
            { ParameterDefinition {
                field: f,
                copy: c.is_some()
            }}

        rule call_type() -> CallType
            = "proc" { CallType::Proc }
            / "macro" { CallType::Macro }
            / expected!("Expected call type specifier")

        rule main() -> Main
            = "main" __ "{" __ s:outer_statement_list() __ "}"
            { Main {
                statements: s
            }}


        rule outer_statement_list() -> Vec<OuterStatement>
            = s:(outer_statement() ** (";" __)) ";"
            { s }
            / __
            { Vec::<OuterStatement>::new() }

        rule outer_statement() -> OuterStatement
            = "line" __ id:("(" id:identifier() ")" { id })? __ "{" __ l:statement_list() __ "}"
            { OuterStatement::Line(l, id) }
            //"loop" __ "{" __ l:outer_statement_list() __ "}"
            //{ OuterStatement::Loop(l) }
            / "@" i:identifier()
            { OuterStatement::Label(i) }
            / s:statement()
            { OuterStatement::Inner(s) }

        rule statement_list() -> Vec<InnerStatement>
            = s:(statement() ** (";" __)) ";"
            { s }
            / __
            { Vec::<InnerStatement>::new() }

        rule statement() -> InnerStatement
            = p:position!() "panic" __ "(" __ m:string() __ ")"
            { InnerStatement::CompilePanic(m, p) }
            / "if" __ "(" __ c:expression() __ ")" __ "{" __ t:statement_list() __ "}" f:(__ "else" __ "{" __ f:statement_list() __ "}" { f })?
            { InnerStatement::If(c, t, f.unwrap_or(Vec::new())) }
            / "return" __ e:expression()
            { InnerStatement::Return(e) }
            / "emit" __ "{" __ s:string() __ "}"
            { InnerStatement::Emit(s) }
            / "goto" __ i:identifier()
            { InnerStatement::Goto(i) }
            / i:identifier() __ "(" __ a:(expression() ** ("," __)) __ ")"
            { InnerStatement::Call(i, a) }
            / "var" __ f:field() __ "=" __ e:expression()
            { InnerStatement::DeclareAssign(f, e) }
            / "const" __ f:field() __ "=" __ e:expression()
            { InnerStatement::DeclareConst(f, e) }
            / i:field_access() __ "=" __ e:expression()
            { InnerStatement::Assign(i, e) }
            / ":" i:identifier() __ "=" __ e:expression()
            { InnerStatement::ExternalAssign(i, e) }
            / i:field_access() __ "+=" __ e:expression()
            { InnerStatement::Assign(i.clone(), Expression::Add(Box::new(Expression::FieldAccess(i)), Box::new(e)))}

        rule field_access() -> Vec<String> =
            s:identifier() f:("." f:(f:identifier() ** "." { f }) { f })?
            {
                let mut v = f.unwrap_or(Vec::new());
                v.insert(0, s);
                v
            }

        rule attributes() -> Vec<Attribute>
            = "[" __ a:(attribute() ** ("," __)) "]"
            { a } 

        rule attribute() -> Attribute
            = n:identifier() __ "(" __ a:(expression() ** ("," __)) __ ")"
            { Attribute { name: n, parameters: a } }


        rule expression() -> Expression
            = precedence!{
                p:position!() "panic" __ "(" __ m:string() __ ")" { Expression::CompilePanic(m, p) }
                --
                x:(@) __ "&" "&"? __ y:@ { Expression::And(Box::new(x), Box::new(y)) }
                --
                x:(@) __ "|" "|"? __ y:@ { Expression::Or(Box::new(x), Box::new(y)) }
                --
                x:(@) __ "is" __ y:identifier() { Expression::Is(Box::new(x), y) }
                --
                x:(@) __ "==" __ y:@ { Expression::Equals(Box::new(x), Box::new(y)) }
                x:(@) __ "!=" __ y:@ { Expression::NotEquals(Box::new(x), Box::new(y)) }
                --
                x:(@) __ ">" __ y:@ { Expression::GreaterThan(Box::new(x), Box::new(y)) }
                x:(@) __ "<" __ y:@ { Expression::LessThan(Box::new(x), Box::new(y)) }
                x:(@) __ ">=" __ y:@ { Expression::GreaterThanOrEq(Box::new(x), Box::new(y)) }
                x:(@) __ "<=" __ y:@ { Expression::LessThanOrEq(Box::new(x), Box::new(y)) }
                --
                x:(@) __ "+" __ y:@ { Expression::Add(Box::new(x), Box::new(y)) }
                x:(@) __ "-" __ y:@ { Expression::Subtract(Box::new(x), Box::new(y)) }
                --
                x:(@) __ "*" __ y:@ { Expression::Multiply(Box::new(x), Box::new(y)) }
                x:(@) __ "/" __ y:@ { Expression::Divide(Box::new(x), Box::new(y)) }
                x:(@) __ "%" __ y:@ { Expression::Modulus(Box::new(x), Box::new(y)) }
                --
                x:(@) __ "^" __ y:@ { Expression::Exponent(Box::new(x), Box::new(y)) }
                --
                "-" x:expression() { Expression::Negate(Box::new(x)) }
                "!" x:expression() { Expression::Not(Box::new(x)) }
                --
                i:identifier() __ "++" { Expression::PostIncrement(i) }
                i:identifier() __ "--" { Expression::PostDecrement(i) }
                "++" __ i:identifier() { Expression::PreIncrement(i) }
                "--" __ i:identifier() { Expression::PreDecrement(i) }
                --
                t:identifier() __ "{" __ c:(constructor_field() ** ("," __)) __ ","? __ "}" { Expression::Constructor(TypeName { typename: t }, c) }
                n:number() { Expression::ConstNumber(n) }
                s:string() { Expression::ConstString(s) }
                ":" i:identifier() { Expression::ExternalFieldAccess(i) }
                "(" __ e:expression() __ ")" { Expression::Bracket(Box::new(e)) }
                n:identifier() __ "(" __ e:(e:expression() ** ("," __) { e }) __ ")" { Expression::Call(n, e) }
                i:field_access() { Expression::FieldAccess(i) }
            }

        rule constructor_field() -> (String, Expression)
            = i:identifier() __ ":" __ e:expression()
            { (i, e) }

        rule identifier() -> String
            = i:$(['A'..='Z' | 'a'..='z' | '_']['A'..='Z' | 'a'..='z' | '0'..='9' | '_']*)
            { i.to_string() }
            / expected!("Identifier")

        rule path() -> String
            = p:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '\\' | '/' | ':' | '.' | '_' | ' ']+)
            { p.to_string().replace("\\", "/") }
            / expected!("Path")



        rule field() -> FieldDefinition
            = n:identifier() __ ":" __ t:identifier()
            { FieldDefinition { name: n, typename: TypeName { typename: t } } }

        rule string() -> String
            = "\"" s:$((!"\"" [_])*) "\""
            { s.to_string() }

        rule number() -> String
            = p:$("-"? ['0'..='9']+ ("." ['0'..='9']+)?)
            { p.to_string() }



        rule newline()
            = "\r"? "\n"

        rule comment()
            = "//" (!newline() [_])* newline()
            / "/*" (!"*/" [_])* "*/"

        rule whitespace()
            = " "
            / "\t"
            / newline()

        rule __()
            = quiet!{ (comment() / whitespace())* }
        


        rule uint() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() }
        
        pub rule list() -> Vec<u64>
            = "[" l:uint() ** "," "]" { l }
    }

}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn parse_file() {
        let all = fs::read_to_string("tests/all.y").unwrap();

        for l in all.lines() {
            println!("{:?}", l);
        }

        let prog = y_parser::program(&all).unwrap();

        assert_eq!(3, prog.imports.len());
        assert_eq!("file.y", prog.imports[0].path);
        assert_eq!("x/y/z.y", prog.imports[1].path);
        assert_eq!("lib/number_parser.y", prog.imports[2].path);
    }
}