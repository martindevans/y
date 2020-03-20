Program {
    imports: [],
    constants: [],
    enums: [],
    structs: [
        StructDefinition {
            name: "pid_constants",
            fields: [
                FieldDefinition {
                    name: "p",
                    typename: "number",
                },
                FieldDefinition {
                    name: "i",
                    typename: "number",
                },
                FieldDefinition {
                    name: "d",
                    typename: "number",
                },
                FieldDefinition {
                    name: "r",
                    typename: "number",
                },
            ],
        },
        StructDefinition {
            name: "pid_state",
            fields: [
                FieldDefinition {
                    name: "previous_error",
                    typename: "number",
                },
                FieldDefinition {
                    name: "previous_derivative",
                    typename: "number",
                },
                FieldDefinition {
                    name: "integrated_error",
                    typename: "number",
                },
            ],
        },
        StructDefinition {
            name: "pid",
            fields: [
                FieldDefinition {
                    name: "constants",
                    typename: "pid_constants",
                },
                FieldDefinition {
                    name: "state",
                    typename: "pid_state",
                },
            ],
        },
    ],
    ranges: [],
    callables: [
        CallableDefinition {
            name: "pid_update",
            call_type: Macro,
            parameters: [
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "pid",
                        typename: "pid",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "target",
                        typename: "number",
                    },
                    copy: false,
                },
                ParameterDefinition {
                    field: FieldDefinition {
                        name: "measurement",
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
                        name: "error",
                        typename: "number",
                    },
                    Subtract(
                        FieldAccess(
                            [
                                "target",
                            ],
                        ),
                        FieldAccess(
                            [
                                "measurement",
                            ],
                        ),
                    ),
                ),
                Assign(
                    [
                        "pid",
                        "integrated_error",
                    ],
                    Add(
                        FieldAccess(
                            [
                                "pid",
                                "integrated_error",
                            ],
                        ),
                        FieldAccess(
                            [
                                "error",
                            ],
                        ),
                    ),
                ),
                DeclareAssign(
                    FieldDefinition {
                        name: "dedt",
                        typename: "number",
                    },
                    Add(
                        Multiply(
                            Bracket(
                                Subtract(
                                    FieldAccess(
                                        [
                                            "pid",
                                            "state",
                                            "previous_error",
                                        ],
                                    ),
                                    FieldAccess(
                                        [
                                            "error",
                                        ],
                                    ),
                                ),
                            ),
                            Bracket(
                                Subtract(
                                    ConstNumber(
                                        "1",
                                    ),
                                    FieldAccess(
                                        [
                                            "pid",
                                            "r",
                                        ],
                                    ),
                                ),
                            ),
                        ),
                        Multiply(
                            FieldAccess(
                                [
                                    "pid",
                                    "state",
                                    "previous_derivative",
                                ],
                            ),
                            FieldAccess(
                                [
                                    "pid",
                                    "r",
                                ],
                            ),
                        ),
                    ),
                ),
                Assign(
                    [
                        "pid",
                        "state",
                        "previous_derivative",
                    ],
                    FieldAccess(
                        [
                            "dedt",
                        ],
                    ),
                ),
                Assign(
                    [
                        "pid",
                        "state",
                        "previous_error",
                    ],
                    FieldAccess(
                        [
                            "error",
                        ],
                    ),
                ),
                Return(
                    Add(
                        Add(
                            Multiply(
                                FieldAccess(
                                    [
                                        "pid",
                                        "p",
                                    ],
                                ),
                                FieldAccess(
                                    [
                                        "error",
                                    ],
                                ),
                            ),
                            Multiply(
                                FieldAccess(
                                    [
                                        "pid",
                                        "i",
                                    ],
                                ),
                                FieldAccess(
                                    [
                                        "pid",
                                        "integrated_error",
                                    ],
                                ),
                            ),
                        ),
                        Multiply(
                            FieldAccess(
                                [
                                    "pid",
                                    "d",
                                ],
                            ),
                            FieldAccess(
                                [
                                    "dedt",
                                ],
                            ),
                        ),
                    ),
                ),
            ],
            attributes: [],
        },
    ],
    main: Some(
        MainDefinition {
            statements: [
                Inner(
                    DeclareAssign(
                        FieldDefinition {
                            name: "controller",
                            typename: "pid",
                        },
                        Constructor(
                            [
                                (
                                    "constants",
                                    Constructor(
                                        [
                                            (
                                                "p",
                                                ConstNumber(
                                                    "1",
                                                ),
                                            ),
                                            (
                                                "i",
                                                ConstNumber(
                                                    "0.1",
                                                ),
                                            ),
                                            (
                                                "d",
                                                ConstNumber(
                                                    "0.01",
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            ],
                        ),
                    ),
                ),
                Loop(
                    [
                        Line(
                            [
                                ExternalAssign(
                                    "output",
                                    Call(
                                        "pid_update",
                                        [
                                            FieldAccess(
                                                [
                                                    "controller",
                                                ],
                                            ),
                                            ExternalFieldAccess(
                                                "target",
                                            ),
                                            ExternalFieldAccess(
                                                "input",
                                            ),
                                        ],
                                    ),
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        },
    ),
}