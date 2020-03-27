YololStatementBlocks {
    blocks: [
        Statements(
            None,
            [
                Assignment(
                    Identifier {
                        name: "i",
                        external: false,
                    },
                    ConstantNumber(
                        "0.1",
                    ),
                ),
                Assignment(
                    Identifier {
                        name: "d",
                        external: false,
                    },
                    ConstantNumber(
                        "0.01",
                    ),
                ),
            ],
        ),
        Line(
            Some(
                "loop_start",
            ),
            [
                Goto(
                    VariableAccess(
                        Identifier {
                            name: "goto_layout_label_loop_start",
                            external: false,
                        },
                    ),
                ),
            ],
        ),
        Statements(
            None,
            [],
        ),
    ],
    types: {
        "p": TypeName {
            typename: "number",
        },
        "i": TypeName {
            typename: "number",
        },
        "d": TypeName {
            typename: "number",
        },
    },
    consts: {
        "p": ConstantNumber(
            "1",
        ),
    },
}