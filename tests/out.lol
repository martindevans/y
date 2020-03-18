Program {
    imports: [],
    constants: [
        Constant {
            field: FieldDefinition {
                name: "true",
                typename: "number",
            },
            value: ConstNumber(
                "1",
            ),
        },
        Constant {
            field: FieldDefinition {
                name: "false",
                typename: "number",
            },
            value: ConstNumber(
                "0",
            ),
        },
        Constant {
            field: FieldDefinition {
                name: "pi",
                typename: "number",
            },
            value: ConstNumber(
                "3.141",
            ),
        },
        Constant {
            field: FieldDefinition {
                name: "tau",
                typename: "number",
            },
            value: ConstNumber(
                "6.282",
            ),
        },
        Constant {
            field: FieldDefinition {
                name: "e",
                typename: "number",
            },
            value: ConstNumber(
                "2.718",
            ),
        },
        Constant {
            field: FieldDefinition {
                name: "max_num",
                typename: "number",
            },
            value: ConstNumber(
                "9223372036854775.807",
            ),
        },
        Constant {
            field: FieldDefinition {
                name: "min_num",
                typename: "number",
            },
            value: Negate(
                ConstNumber(
                    "9223372036854775.808",
                ),
            ),
        },
    ],
    enums: [],
    structs: [],
    ranges: [
        RangeDefinition {
            name: "positive",
            base: "number",
            expression: GreaterThan(
                FieldAccess(
                    "positive",
                ),
                ConstNumber(
                    "0",
                ),
            ),
        },
        RangeDefinition {
            name: "positive_or_zero",
            base: "number",
            expression: GreaterThanOrEq(
                FieldAccess(
                    "positive",
                ),
                ConstNumber(
                    "0",
                ),
            ),
        },
        RangeDefinition {
            name: "negative",
            base: "number",
            expression: LessThan(
                FieldAccess(
                    "negative",
                ),
                ConstNumber(
                    "0",
                ),
            ),
        },
        RangeDefinition {
            name: "negative_or_zero",
            base: "number",
            expression: LessThanOrEq(
                FieldAccess(
                    "negative",
                ),
                ConstNumber(
                    "0",
                ),
            ),
        },
        RangeDefinition {
            name: "integer",
            base: "number",
            expression: Equals(
                Multiply(
                    Divide(
                        FieldAccess(
                            "integer",
                        ),
                        ConstNumber(
                            "1000",
                        ),
                    ),
                    ConstNumber(
                        "1000",
                    ),
                ),
                FieldAccess(
                    "integer",
                ),
            ),
        },
        RangeDefinition {
            name: "natural",
            base: "integer",
            expression: GreaterThan(
                FieldAccess(
                    "natural",
                ),
                ConstNumber(
                    "0",
                ),
            ),
        },
        RangeDefinition {
            name: "square",
            base: "number",
            expression: Is(
                Call(
                    "sqrt",
                    [
                        FieldAccess(
                            "square",
                        ),
                    ],
                ),
                "integer",
            ),
        },
    ],
    callables: [
        CallableDefinition {
            name: "parse_base10_char",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "input",
                        typename: "string",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "output",
                        typename: "number",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "counter",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: None,
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "c",
                        typename: "string",
                    },
                    Subtract(
                        FieldAccess(
                            "input",
                        ),
                        Negate(
                            Negate(
                                FieldAccess(
                                    "input",
                                ),
                            ),
                        ),
                    ),
                ),
                DeclareAssign(
                    FieldDefinition {
                        name: "d",
                        typename: "number",
                    },
                    Multiply(
                        ConstNumber(
                            "3",
                        ),
                        Bracket(
                            Add(
                                Add(
                                    Bracket(
                                        GreaterThan(
                                            FieldAccess(
                                                "c",
                                            ),
                                            ConstNumber(
                                                "1",
                                            ),
                                        ),
                                    ),
                                    Bracket(
                                        GreaterThan(
                                            FieldAccess(
                                                "c",
                                            ),
                                            ConstNumber(
                                                "4",
                                            ),
                                        ),
                                    ),
                                ),
                                Bracket(
                                    GreaterThan(
                                        FieldAccess(
                                            "c",
                                        ),
                                        ConstNumber(
                                            "7",
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                Assign(
                    "output",
                    Add(
                        FieldAccess(
                            "output",
                        ),
                        Multiply(
                            Bracket(
                                Subtract(
                                    Add(
                                        FieldAccess(
                                            "d",
                                        ),
                                        Bracket(
                                            GreaterThan(
                                                FieldAccess(
                                                    "c",
                                                ),
                                                FieldAccess(
                                                    "d",
                                                ),
                                            ),
                                        ),
                                    ),
                                    Bracket(
                                        LessThan(
                                            FieldAccess(
                                                "c",
                                            ),
                                            FieldAccess(
                                                "d",
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            Exponent(
                                ConstNumber(
                                    "10",
                                ),
                                PostIncrement(
                                    "counter",
                                ),
                            ),
                        ),
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "parse_base16_char",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "input",
                        typename: "string",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "output",
                        typename: "number",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "counter",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: None,
            statements: [
                DeclareConst(
                    FieldDefinition {
                        name: "x",
                        typename: "string",
                    },
                    ConstString(
                        "FDB97531",
                    ),
                ),
                DeclareConst(
                    FieldDefinition {
                        name: "y",
                        typename: "string",
                    },
                    ConstString(
                        "FEBA7632",
                    ),
                ),
                DeclareAssign(
                    FieldDefinition {
                        name: "c",
                        typename: "string",
                    },
                    Subtract(
                        FieldAccess(
                            "input",
                        ),
                        Negate(
                            Negate(
                                FieldAccess(
                                    "input",
                                ),
                            ),
                        ),
                    ),
                ),
                Assign(
                    "output",
                    Add(
                        FieldAccess(
                            "output",
                        ),
                        Multiply(
                            Bracket(
                                Add(
                                    Add(
                                        Multiply(
                                            ConstNumber(
                                                "4",
                                            ),
                                            Bracket(
                                                Add(
                                                    Add(
                                                        Bracket(
                                                            GreaterThan(
                                                                FieldAccess(
                                                                    "c",
                                                                ),
                                                                ConstNumber(
                                                                    "3",
                                                                ),
                                                            ),
                                                        ),
                                                        Bracket(
                                                            GreaterThan(
                                                                FieldAccess(
                                                                    "c",
                                                                ),
                                                                ConstNumber(
                                                                    "7",
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    Bracket(
                                                        GreaterThan(
                                                            FieldAccess(
                                                                "c",
                                                            ),
                                                            ConstString(
                                                                "B",
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Bracket(
                                            GreaterThan(
                                                FieldAccess(
                                                    "x",
                                                ),
                                                Subtract(
                                                    FieldAccess(
                                                        "x",
                                                    ),
                                                    FieldAccess(
                                                        "c",
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    Multiply(
                                        ConstNumber(
                                            "2",
                                        ),
                                        Bracket(
                                            GreaterThan(
                                                FieldAccess(
                                                    "y",
                                                ),
                                                Subtract(
                                                    FieldAccess(
                                                        "y",
                                                    ),
                                                    FieldAccess(
                                                        "c",
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            Exponent(
                                ConstNumber(
                                    "16",
                                ),
                                PostIncrement(
                                    "counter",
                                ),
                            ),
                        ),
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "assert",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "bool",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "msg",
                        typename: "string",
                    },
                    copy: false,
                },
            ],
            return_type: None,
            statements: [
                If(
                    Not(
                        FieldAccess(
                            "a",
                        ),
                    ),
                    [
                        ExternalAssign(
                            "assert_fail_msg",
                            FieldAccess(
                                "msg",
                            ),
                        ),
                    ],
                    [],
                ),
            ],
            attributes: [
                Attribute {
                    name: "cfg",
                    parameters: [
                        ConstString(
                            "test",
                        ),
                    ],
                },
            ],
        },
        CallableDefinition {
            name: "assert_eq",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "any",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "b",
                        typename: "any",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "msg",
                        typename: "string",
                    },
                    copy: false,
                },
            ],
            return_type: None,
            statements: [
                Call(
                    "assert",
                    [
                        Equals(
                            FieldAccess(
                                "a",
                            ),
                            FieldAccess(
                                "b",
                            ),
                        ),
                        FieldAccess(
                            "msg",
                        ),
                    ],
                ),
            ],
            attributes: [
                Attribute {
                    name: "cfg",
                    parameters: [
                        ConstString(
                            "test",
                        ),
                    ],
                },
            ],
        },
        CallableDefinition {
            name: "assert_neq",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "any",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "b",
                        typename: "any",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "msg",
                        typename: "string",
                    },
                    copy: false,
                },
            ],
            return_type: None,
            statements: [
                Call(
                    "assert",
                    [
                        NotEquals(
                            FieldAccess(
                                "a",
                            ),
                            FieldAccess(
                                "b",
                            ),
                        ),
                        FieldAccess(
                            "msg",
                        ),
                    ],
                ),
            ],
            attributes: [
                Attribute {
                    name: "cfg",
                    parameters: [
                        ConstString(
                            "test",
                        ),
                    ],
                },
            ],
        },
        CallableDefinition {
            name: "abs",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: Some(
                "number",
            ),
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "r",
                        typename: "number",
                    },
                    ConstNumber(
                        "0",
                    ),
                ),
                Emit(
                    "r = abs a",
                ),
                Return(
                    FieldAccess(
                        "r",
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "sqrt",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: Some(
                "number",
            ),
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "r",
                        typename: "number",
                    },
                    ConstNumber(
                        "0",
                    ),
                ),
                Emit(
                    "r = sqrt a",
                ),
                Return(
                    FieldAccess(
                        "r",
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "sin",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: Some(
                "number",
            ),
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "r",
                        typename: "number",
                    },
                    ConstNumber(
                        "0",
                    ),
                ),
                Emit(
                    "r = sin a",
                ),
                Return(
                    FieldAccess(
                        "r",
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "cos",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: Some(
                "number",
            ),
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "r",
                        typename: "number",
                    },
                    ConstNumber(
                        "0",
                    ),
                ),
                Emit(
                    "r = cos a",
                ),
                Return(
                    FieldAccess(
                        "r",
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "tan",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: Some(
                "number",
            ),
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "r",
                        typename: "number",
                    },
                    ConstNumber(
                        "0",
                    ),
                ),
                Emit(
                    "r = tan a",
                ),
                Return(
                    FieldAccess(
                        "r",
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "asin",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: Some(
                "number",
            ),
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "r",
                        typename: "number",
                    },
                    ConstNumber(
                        "0",
                    ),
                ),
                Emit(
                    "r = asin a",
                ),
                Return(
                    FieldAccess(
                        "r",
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "acos",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: Some(
                "number",
            ),
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "r",
                        typename: "number",
                    },
                    ConstNumber(
                        "0",
                    ),
                ),
                Emit(
                    "r = acos a",
                ),
                Return(
                    FieldAccess(
                        "r",
                    ),
                ),
            ],
            attributes: [],
        },
        CallableDefinition {
            name: "atan",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "a",
                        typename: "number",
                    },
                    copy: false,
                },
            ],
            return_type: Some(
                "number",
            ),
            statements: [
                DeclareAssign(
                    FieldDefinition {
                        name: "r",
                        typename: "number",
                    },
                    ConstNumber(
                        "0",
                    ),
                ),
                Emit(
                    "r = atan a",
                ),
                Return(
                    FieldAccess(
                        "r",
                    ),
                ),
            ],
            attributes: [],
        },
    ],
    main: Some(
        MainDefinition {
            statements: [
                CompilePanic(
                    "contents of `main` block remain unimplemented",
                ),
            ],
        },
    ),
}