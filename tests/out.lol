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
                        YololNumber(
                            100,
                        ),
                    ),
                ),
                Assignment(
                    Identifier {
                        name: "d",
                        external: false,
                    },
                    ConstantNumber(
                        YololNumber(
                            10,
                        ),
                    ),
                ),
                Assignment(
                    Identifier {
                        name: "x",
                        external: false,
                    },
                    ConstantNumber(
                        YololNumber(
                            1000,
                        ),
                    ),
                ),
                Assignment(
                    Identifier {
                        name: "y",
                        external: false,
                    },
                    ConstantNumber(
                        YololNumber(
                            0,
                        ),
                    ),
                ),
            ],
        ),
    ],
    types: {
        "x": Bool,
        "y": Bool,
        "d": Num,
        "i": Num,
        "p": Num,
    },
    consts: {
        "p": ConstantNumber(
            YololNumber(
                1000,
            ),
        ),
    },
}