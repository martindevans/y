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
    ranges: [],
    callables: [
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
    ],
    main: Some(
        MainDefinition {
            statements: [
                Call(
                    "assert",
                    [
                        FieldAccess(
                            "true",
                        ),
                        ConstString(
                            "true",
                        ),
                    ],
                ),
                Call(
                    "assert_eq",
                    [
                        ConstNumber(
                            "1",
                        ),
                        ConstNumber(
                            "1",
                        ),
                        ConstString(
                            "1 == 1",
                        ),
                    ],
                ),
                Call(
                    "assert_neq",
                    [
                        ConstNumber(
                            "1",
                        ),
                        ConstNumber(
                            "2",
                        ),
                        ConstString(
                            "1 != 2",
                        ),
                    ],
                ),
            ],
        },
    ),
}