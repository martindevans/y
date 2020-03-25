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
        Statements(
            Some(
                "label",
            ),
            [],
        ),
        Line(
            Some(
                "loop_start",
            ),
            [],
        ),
        Statements(
            None,
            [],
        ),
    ],
    types: {
        "i": TypeName {
            typename: "number",
        },
        "d": TypeName {
            typename: "number",
        },
        "p": TypeName {
            typename: "number",
        },
    },
    consts: {
        "p": ConstantNumber(
            "1",
        ),
    },
}