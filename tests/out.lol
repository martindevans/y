YololStatementBlocks {
    blocks: [
        Statements(
            None,
            [
                Assignment(
                    Identifier {
                        name: "a",
                        external: false,
                    },
                    ConstantNumber(
                        "7",
                    ),
                ),
                Assignment(
                    Identifier {
                        name: "b",
                        external: false,
                    },
                    ConstantNumber(
                        "10",
                    ),
                ),
                Assignment(
                    Identifier {
                        name: "out",
                        external: true,
                    },
                    Add(
                        VariableAccess(
                            Identifier {
                                name: "a",
                                external: false,
                            },
                        ),
                        VariableAccess(
                            Identifier {
                                name: "b",
                                external: false,
                            },
                        ),
                    ),
                ),
            ],
        ),
    ],
    types: {
        "b": Num,
        "a": Num,
    },
    consts: {},
}