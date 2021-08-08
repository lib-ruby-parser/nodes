use crate::{Node, NodeField, NodeFieldList, NodeFieldType, NodeList};

pub(crate) const ALL_NODES: NodeList = NodeList(
    &[
        Node {
            struct_name: "Alias",
            str_type: "alias",
            filename: "alias",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "to",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Target of the `alias`.",
                            "",
                            "`Sym(\"foo\")` node for `alias :foo :bar`"
                        ],
                    },

                    NodeField {
                        field_name: "from",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Source of the `alias`.",
                            "",
                            "`Sym(\"bar\")` node for `alias :foo :bar`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `alias` keyword",
                            "",
                            "```text",
                            "alias foo bar",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "alias foo bar",
                            "~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `alias to from` statement."
            ],
        },

        Node {
            struct_name: "AndAsgn",
            str_type: "and_asgn",
            filename: "and_asgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "recv",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Receiver of the `&&=` operation.",
                            "",
                            "`Lvasgn(\"a\")` node for `a &&= 1`"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Right hand statement of assignment",
                            "",
                            "`Int(\"1\")` node for `a &&= 1`"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `&&=` operator",
                            "",
                            "```text",
                            "a &&= 1",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "a &&= 1",
                            "~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `a &&= 1` statement."
            ],
        },

        Node {
            struct_name: "And",
            str_type: "and",
            filename: "and",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "lhs",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Left hand statament of the `&&` operation.",
                            "",
                            "`Lvar(\"foo\")` node for `foo && bar`"
                        ],
                    },

                    NodeField {
                        field_name: "rhs",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Right hand statement of the `&&` operation.",
                            "",
                            "`Lvar(\"bar\")` node for `foo && bar`"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `&&` (or `and`) operator",
                            "",
                            "```text",
                            "a && b",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "a && b",
                            "~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `foo && bar` (or `foo and bar`) statement."
            ],
        },

        Node {
            struct_name: "Arg",
            str_type: "arg",
            filename: "arg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the argument"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def m(argument); end",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a positional required block/method argument.",
                "",
                "`a` in `def m(a); end` or `proc { |a| }`"
            ],
        },

        Node {
            struct_name: "Args",
            str_type: "args",
            filename: "args",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "List of arguments"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def m(a, b = 1, c:, &blk); end",
                            "~~~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "def m(a, b = 1, c:, &blk); end",
                            "~",
                            "```",
                            "",
                            "`None` for code like `def m; end` or `def m arg; end`"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "def m(a, b = 1, c:, &blk); end",
                            "~",
                            "```",
                            "",
                            "`None` for code like `def m; end` or `def m arg; end`"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an arguments list",
                "",
                "`Args(vec![Arg(\"a\"), Optarg(\"b\", Int(\"1\"))])` in `def m(a, b = 1); end`"
            ],
        },

        Node {
            struct_name: "Array",
            str_type: "array",
            filename: "array",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "elements",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of elements"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open bracket",
                            "",
                            "```text",
                            "[1, 2, 3]",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing bracket",
                            "",
                            "```text",
                            "[1, 2, 3]",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "[1, 2, 3]",
                            "~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an array literal"
            ],
        },

        Node {
            struct_name: "ArrayPattern",
            str_type: "array_pattern",
            filename: "array_pattern",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "elements",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of elements"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open bracket",
                            "",
                            "```text",
                            "[1, ^a, 3 => foo]",
                            "~",
                            "```",
                            "",
                            "`None` for pattern like `1, 2` without brackets"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing bracket",
                            "",
                            "```text",
                            "[1, ^a, 3 => foo]",
                            "~",
                            "```",
                            "",
                            "`None` for pattern like `1, 2` without brackets"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "[1, ^a, 3 => foo]",
                            "~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an array pattern used in pattern matching"
            ],
        },

        Node {
            struct_name: "ArrayPatternWithTail",
            str_type: "array_pattern_with_tail",
            filename: "array_pattern_with_tail",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "elements",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of elements"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open bracket",
                            "",
                            "```text",
                            "[1, ^a, 3 => foo,]",
                            "~",
                            "```",
                            "",
                            "`None` for pattern like `1, 2,` without brackets"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing bracket",
                            "",
                            "```text",
                            "[1, ^a, 3 => foo,]",
                            "~",
                            "```",
                            "",
                            "`None` for pattern like `1, 2,` without brackets"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "[1, ^a, 3 => foo,]",
                            "~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an array pattern *with trailing comma* used in pattern matching",
                "",
                "It's slightly different from `ArrayPattern`, trailing comma at the end works as `, *`"
            ],
        },

        Node {
            struct_name: "BackRef",
            str_type: "back_ref",
            filename: "back_ref",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the variable (`\"$+\"` for `$+`)"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "$+",
                            "~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents special global variables:",
                "1. `` $` ``",
                "2. `$&`",
                "3. `$'`",
                "4. `$+`"
            ],
        },

        Node {
            struct_name: "Begin",
            str_type: "begin",
            filename: "begin",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "statements",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of statements"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Begin of the block",
                            "",
                            "```text",
                            "(1; 2)",
                            "~",
                            "```",
                            "",
                            "`None` if the block of code is \"implicit\", like",
                            "",
                            "```text",
                            "if true; 1; 2; end",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "End of the block",
                            "",
                            "```text",
                            "(1; 2)",
                            "~",
                            "```",
                            "",
                            "`None` if the block of code is \"implicit\", like",
                            "",
                            "```text",
                            "if true; 1; 2; end",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "(1; 2)",
                            "~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents compound statement (i.e. a multi-statement)",
                "",
                "Basically all blocks of code are wrapped into `Begin` node (e.g. method/block body, rescue/ensure handler etc)"
            ],
        },

        Node {
            struct_name: "Block",
            str_type: "block",
            filename: "block",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "call",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Method call that takes a block",
                            "",
                            "`Send(\"foo\")` in `foo {}`"
                        ],
                    },

                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "A list of argument that block takes",
                            "",
                            "`vec![ Arg(\"a\"), Optarg(\"b\", Int(\"1\")) ]` for `proc { |a, b = 1| }`",
                            "",
                            "`None` if the block takes no arguments"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Block body, `None` if block has no body."
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the open brace",
                            "",
                            "```text",
                            "proc { }",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the closing brace",
                            "",
                            "```text",
                            "proc { }",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "proc { }",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a Ruby block that is passed to a method (`proc { |foo| bar }`)"
            ],
        },

        Node {
            struct_name: "BlockPass",
            str_type: "block_pass",
            filename: "block_pass",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Value that is converted to a block",
                            "",
                            "`Int(\"1\")` in `foo(&1)` (yes, it's possible)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `&` operator",
                            "",
                            "```text",
                            "foo(&blk)",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo(&bar)",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `&blk` argument of the method call (but not of the method definition, see `BlockArg`)"
            ],
        },

        Node {
            struct_name: "Blockarg",
            str_type: "blockarg",
            filename: "blockarg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the argument, `String(\"foo\")` for `def m(&foo)`"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `&` operator",
                            "",
                            "```text",
                            "def m(&foo); end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the name",
                            "",
                            "```text",
                            "def m(&foo); end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def m(&foo); end",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `&blk` argument in the method definition (but not in the method call, see `BlockPass`)"
            ],
        },

        Node {
            struct_name: "Break",
            str_type: "break",
            filename: "break_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of arguments"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `break` keyword",
                            "",
                            "```text",
                            "break :foo",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "break(:foo)",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `break` keyword (with optional argument)"
            ],
        },

        Node {
            struct_name: "Case",
            str_type: "case",
            filename: "case",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expr",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Expression given to `case`, `Int(\"1\")` for `case 1; end`",
                            "`None` for code like",
                            "",
                            "```text",
                            "case",
                            "when pattern",
                            "end",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "when_bodies",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of `When` nodes (each has `patterns` and `body`)"
                        ],
                    },

                    NodeField {
                        field_name: "else_body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the `else` branch, `None` if there's no `else` branch"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `case` keyword",
                            "",
                            "```text",
                            "case 1; end",
                            "~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "else_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `else` keyword",
                            "",
                            "```text",
                            "case 1; else; end",
                            "~~~~",
                            "```",
                            "",
                            "`None` if there's no `else` branch"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "case 1; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case 1; end",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `case` statement (for pattern matching see `CaseMatch` node)"
            ],
        },

        Node {
            struct_name: "CaseMatch",
            str_type: "case_match",
            filename: "case_match",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expr",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Expression given to `case`, `Int(\"1\")` for `case 1; in 1; end`",
                            "`None` for code like",
                            "",
                            "```text",
                            "case",
                            "in pattern",
                            "end",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "in_bodies",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of `InPattern` nodes (each has `pattern`, `guard` and `body`)"
                        ],
                    },

                    NodeField {
                        field_name: "else_body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the `else` branch, `None` if there's no `else` branch"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `case` keyword",
                            "",
                            "```text",
                            "case 1; in 2; end",
                            "~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "else_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `else` keyword",
                            "",
                            "```text",
                            "case 1; in 2; else; end",
                            "~~~~",
                            "```",
                            "",
                            "`None` if there's no `else` branch"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "case 1; in 2; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case 1; in 2; end",
                            "~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `case` statement used for pattern matching (for regular `case` see `Case` node)"
            ],
        },

        Node {
            struct_name: "Casgn",
            str_type: "casgn",
            filename: "casgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "scope",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Scope where the constant is defined:",
                            "1. `Some(Const(\"A\"))` for `A::B = 1`",
                            "2. `None` if it's defined in the current scope (i.e. `A = 1`)",
                            "3. `Some(Cbase)` if it's defined in the global scope (i.e. `::A = 1`)"
                        ],
                    },

                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the constant, `String(\"A\")` for `A = 1`"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Value that is assigned to a constant, `Int(\"1\")` for `A = 1`.",
                            "",
                            "**Note**: `None` if constant assignment is a part of the multi-assignment.",
                            "In such case `value` belongs to `Masgn` node of the multi-assignment."
                        ],
                    },

                    NodeField {
                        field_name: "double_colon_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `::` operator",
                            "",
                            "```text",
                            "A::B = 1",
                            "~~",
                            "",
                            "::A = 1",
                            "~~",
                            "```",
                            "",
                            "`None` if the constant is defined in the current scope"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the constant name",
                            "",
                            "```text",
                            "A::CONST = 1",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator",
                            "",
                            "```text",
                            "A = 1",
                            "~",
                            "```",
                            "",
                            "`None` if constant assignment is a part of the multi-assignment.",
                            "In such case `=` belongs to a `Masgn` node"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "A = 1",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a constant assignment (i.e. `A = 1`)"
            ],
        },

        Node {
            struct_name: "Cbase",
            str_type: "cbase",
            filename: "cbase",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "::A",
                            "~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents leading `::` part of the constant access/assignment that is used to get/set on a global namespace."
            ],
        },

        Node {
            struct_name: "Class",
            str_type: "class",
            filename: "class",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Name of the class, `String(\"Foo\")` for `class Foo; end`"
                        ],
                    },

                    NodeField {
                        field_name: "superclass",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Superclass. Can be an expression in cases like `class A < (obj.foo + 1); end`",
                            "",
                            "`None` if no explicit superclass given (i.e. `class Foo; end`)"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the method, `None` if there's no body."
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `class` keyword.",
                            "",
                            "```text",
                            "class Foo; end",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `<` operator",
                            "",
                            "```text",
                            "class A < B; end",
                            "~",
                            "```",
                            "",
                            "`None` if there's no explicit superclass given."
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword.",
                            "",
                            "```text",
                            "class Foo; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "class Foo; end",
                            "~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a class definition (using a `class` keyword, `Class.new` is just a method call)"
            ],
        },

        Node {
            struct_name: "Complex",
            str_type: "complex",
            filename: "complex",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Value of the complex literal, returned as a `String`, `String(\"1i\")` for `1i`"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `-` (but not `+`) operator. `+` is a part of the literal:",
                            "1. `+1i` is `String(\"+1i\")` with `operator = None`",
                            "2. `-1i` is `String(\"1i\")` with `operator = String(\"-\")`",
                            "",
                            "```text",
                            "-1i",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "-1i",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `Complex` literal (that returns an `Complex` number)"
            ],
        },

        Node {
            struct_name: "Const",
            str_type: "const",
            filename: "const_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "scope",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Scope where the constant is taken from:",
                            "1. `Some(Const(\"A\"))` for `A::B`",
                            "2. `None` if it's taken from the current scope (i.e. `A`)",
                            "3. `Some(Cbase)` if it's taken from the global scope (i.e. `::A`)"
                        ],
                    },

                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the constant, `String(\"Foo\")` for `Foo`"
                        ],
                    },

                    NodeField {
                        field_name: "double_colon_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `::` operator. `None` if constant is taken from the current scope.",
                            "",
                            "```text",
                            "A::B",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the constant name",
                            "",
                            "```text",
                            "Foo::Bar",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "Foo::Bar",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents constant access (i.e. `Foo::Bar`)"
            ],
        },

        Node {
            struct_name: "ConstPattern",
            str_type: "const_pattern",
            filename: "const_pattern",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "const_",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Constant that is used, `Const(\"Foo\")` for `in For(42)`"
                        ],
                    },

                    NodeField {
                        field_name: "pattern",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Inner part of the constant pattern",
                            "",
                            "`ArrayPattern(vec![ Int(\"1\") ])` for `Foo(1)`"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "case 1; in Foo(42); end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "case 1; in Foo(42); end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case 1; in Foo(42); end",
                            "~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Const pattern used in pattern matching (e.g. `in A(1, 2)`)"
            ],
        },

        Node {
            struct_name: "CSend",
            str_type: "csend",
            filename: "csend",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "recv",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Receiver of the method call, `Int(\"1\")` for `1&.foo`"
                        ],
                    },

                    NodeField {
                        field_name: "method_name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the method, `String(\"foo\")` for `1&.foo`"
                        ],
                    },

                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "List of arguments",
                            "",
                            "```text",
                            "foo&.bar(42)",
                            "# and also setters like",
                            "foo&.bar = 42",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "dot_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `&.` operator",
                            "",
                            "```text",
                            "foo&.bar",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "selector_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the method name",
                            "",
                            "```text",
                            "foo&.bar(42)",
                            "~~~",
                            "```",
                            "",
                            "`None` in a very special case when method call is implicit (i.e. `foo&.()`)"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "foo&.bar(42)",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "foo&.bar(42)",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the operator if `CSend` is a part of assignment like",
                            "",
                            "```text",
                            "foo&.bar = 1",
                            "~",
                            "```",
                            "",
                            "`None` for a regular call."
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo&.bar(42)",
                            "~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents conditional method call using `&.` operator"
            ],
        },

        Node {
            struct_name: "Cvar",
            str_type: "cvar",
            filename: "cvar",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the class variable, `String(\"@@foo\")` for `@@foo`"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "@@foo",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents access to class variable (i.e. `@@var`)"
            ],
        },

        Node {
            struct_name: "Cvasgn",
            str_type: "cvasgn",
            filename: "cvasgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the class variable, `String(\"@@foo\")` for `@@foo = 1`"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Value that is assigned to class variable, `Int(\"1\")` for `@@foo = 1`"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the class variable name",
                            "",
                            "```text",
                            "@@foo = 1",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator",
                            "",
                            "```text",
                            "@@foo = 1",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "@@foo = 1",
                            "~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents class variable assignment (i.e. `@@var = 42`)"
            ],
        },

        Node {
            struct_name: "Def",
            str_type: "def",
            filename: "def",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the method, `String(\"foo\")` for `def foo; end`"
                        ],
                    },

                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Arguments of a method, `None` if there's no arguments.",
                            "",
                            "All information about parentheses around arguments is stored in this node."
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of a method, `None` if there's no body."
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `def` keyword.",
                            "",
                            "```text",
                            "def foo; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the method name.",
                            "",
                            "```text",
                            "def foo; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword.",
                            "",
                            "```text",
                            "def foo; end",
                            "~~~",
                            "```",
                            "",
                            "`None` for endless method definition"
                        ],
                    },

                    NodeField {
                        field_name: "assignment_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator for endless method definition",
                            "",
                            "```text",
                            "def m() = 1",
                            "~",
                            "```",
                            "",
                            "`None` for regular method definition"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def m(a); foo; end",
                            "~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents method definition using `def` keyword (not on a singleton, see `Defs` node)."
            ],
        },

        Node {
            struct_name: "Defined",
            str_type: "defined?",
            filename: "defined",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Value given to `defined?`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `defined?` keyword",
                            "",
                            "```text",
                            "defined?(foo)",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "defined?(foo)",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "defined?(foo)",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "defined?(foo)",
                            "~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `defined?(foo)` expression"
            ],
        },

        Node {
            struct_name: "Defs",
            str_type: "defs",
            filename: "defs",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "definee",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Definee of a method definition, `Lvar(\"x\")` for `def x.foo; end"
                        ],
                    },

                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the method, `String(\"foo\")` for `def x.foo; end`"
                        ],
                    },

                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Arguments of a method, `None` if there's no arguments.",
                            "",
                            "All information about parentheses around arguments is stored in this node."
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the method, `None` if there's no body."
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `def` keyword",
                            "",
                            "```text",
                            "def self.foo; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `.`",
                            "",
                            "```text",
                            "def self.foo; end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the method name",
                            "",
                            "```text",
                            "def self.foo; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "assignment_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator for endless method definition",
                            "",
                            "```text",
                            "def self.foo() = 42",
                            "~",
                            "```",
                            "",
                            "`None` for regular method definition"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "def self.foo; end",
                            "~~~",
                            "```",
                            "",
                            "`None` for endless method definition"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def self.foo; end",
                            "~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a singleton method definition (i.e. `def self.foo; end`)"
            ],
        },

        Node {
            struct_name: "Dstr",
            str_type: "dstr",
            filename: "dstr",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "parts",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of string parts (static literals and interpolated expressions)"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the string begin",
                            "",
                            "```text",
                            "\"#{foo}\"",
                            "~",
                            "",
                            "%Q{#{foo}}",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the string end",
                            "",
                            "```text",
                            "\"#{foo}\"",
                            "~",
                            "",
                            "%Q{#{foo}}",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "\"#{foo}\"",
                            "~~~~~~~~",
                            "",
                            "%Q{#{foo}}",
                            "~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a string with interpolation (i.e. `\"#{foo}\"`)"
            ],
        },

        Node {
            struct_name: "Dsym",
            str_type: "dsym",
            filename: "dsym",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "parts",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of symbol parts (static literals and interpolated expressions)"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the symbol begin",
                            "",
                            "```text",
                            ":\"#{foo}\"",
                            "~~",
                            "```",
                            "",
                            "`None` if `Dsym` is a part of the interpolated symbol array:",
                            "",
                            "```text",
                            "%I[#{bar}]",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the symbol begin",
                            "",
                            "```text",
                            ":\"#{foo}\"",
                            "~",
                            "```",
                            "",
                            "`None` if `Dsym` is a part of the interpolated symbol array:",
                            "",
                            "```text",
                            "%I[#{bar}]",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            ":\"#{foo}\"",
                            "~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a symbol with interpolation (i.e. `:\"#{foo}\"`)"
            ],
        },

        Node {
            struct_name: "EFlipFlop",
            str_type: "eflipflop",
            filename: "eflipflop",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "left",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Left part of the flip-flop. `None` if based on a range without begin (`...bar`)"
                        ],
                    },

                    NodeField {
                        field_name: "right",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Right part of the flip-flop. `None` if based on a range without end (`foo...`)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `...` operator",
                            "",
                            "```text",
                            "if foo...bar; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "if foo...bar; end",
                            "~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents exclusive flip-flop (i.e. in `if foo...bar; end`)"
            ],
        },

        Node {
            struct_name: "EmptyElse",
            str_type: "empty_else",
            filename: "empty_else",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `else` keyword",
                            "",
                            "```text",
                            "case foo; in 1; else; end",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a special empty else that is a part of the pattern matching.",
                "",
                "Usually empty else (e.g. part of the `if` statement) doesn't mean anything,",
                "however in pattern matching it prevents raising a `NoPatternError`.",
                "",
                "Throwing away this `else` may affect your code."
            ],
        },

        Node {
            struct_name: "Encoding",
            str_type: "__ENCODING__",
            filename: "encoding_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `__ENCODING__` keyword",
                            "",
                            "```text",
                            "__ENCODING__",
                            "~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a special `__ENCODING__` keyword"
            ],
        },

        Node {
            struct_name: "Ensure",
            str_type: "ensure",
            filename: "ensure",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Block of code that is wrapped into `ensure`",
                            "**Note**: that's the body of the `ensure` block",
                            "",
                            "`Int(\"1\")` for `begin; 1; ensure; 2; end`"
                        ],
                    },

                    NodeField {
                        field_name: "ensure",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the `ensure` block",
                            "",
                            "`Int(\"2\")` for `begin; 1; ensure; 2; end`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `ensure` keyword",
                            "",
                            "```text",
                            "begin; ensure; end",
                            "~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "begin; 1; rescue; 2; else; 3; ensure; 4; end",
                            "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
                            "```",
                            "",
                            "**Note**: begin/end belong to `KwBegin` node."
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a block of code with `ensure` (i.e. `begin; ensure; end`)"
            ],
        },

        Node {
            struct_name: "Erange",
            str_type: "erange",
            filename: "erange",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "left",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Begin of the range, `None` if range has no begin (i.e `...42`)"
                        ],
                    },

                    NodeField {
                        field_name: "right",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "End of the range, `None` if range has no end (i.e `42...`)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `...` operator",
                            "",
                            "```text",
                            "1...3",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "1...3",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents range literal with excluded `end` (i.e. `1...3`)"
            ],
        },

        Node {
            struct_name: "False",
            str_type: "false",
            filename: "false_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `false` literal",
                            "",
                            "```text",
                            "false",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `false` literal"
            ],
        },

        Node {
            struct_name: "File",
            str_type: "__FILE__",
            filename: "file",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `__FILE__` literal",
                            "",
                            "```text",
                            "__FILE__",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a special `__FILE__` literal"
            ],
        },

        Node {
            struct_name: "FindPattern",
            str_type: "find_pattern",
            filename: "find_pattern",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "elements",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "Inner part of the find pattern"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the begin",
                            "",
                            "```text",
                            "case foo; in [*x, 1 => a, *y]; end",
                            "~",
                            "```",
                            "",
                            "`None` if there are no brackets/parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the end",
                            "",
                            "```text",
                            "case foo; in [*x, 1 => a, *y]; end",
                            "~",
                            "```",
                            "",
                            "`None` if there are no brackets/parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case foo; in [*x, 1 => a, *y]; end",
                            "~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a find pattern using in pattern matching (i.e. `in [*x, 1 => a, *y]`)",
                "",
                "It's different from `ArrayPattern`/`ConstPattern` because it supports multiple wildcard pattern"
            ],
        },

        Node {
            struct_name: "Float",
            str_type: "float",
            filename: "float",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "String value of the literal, `String(\"42.5\")` for `42.5`"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of unary `-` (but not `+`)",
                            "",
                            "```text",
                            "-42.5",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "-42.5",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a float literal (i.e. `42.5`)"
            ],
        },

        Node {
            struct_name: "For",
            str_type: "for",
            filename: "for_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "iterator",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Variable that is used in loop, `Lvasgn(\"a\")` in `for a in b; end`"
                        ],
                    },

                    NodeField {
                        field_name: "iteratee",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Collection that is for iteration. `Lvar(\"b\")` in `for a in b; end`"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the loop. `None` if there's no body"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `for` keyword",
                            "",
                            "```text",
                            "for a in b; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `in` keyword",
                            "",
                            "```text",
                            "for a in b; end",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `do` keyword",
                            "",
                            "```text",
                            "for a in b do; end",
                            "~~",
                            "```",
                            "",
                            "**Note**: this `do` is optional, and so `begin_l` can be `None`."
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "for a in b; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "for a in b; end",
                            "~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `for` loop"
            ],
        },

        Node {
            struct_name: "ForwardArg",
            str_type: "forward_arg",
            filename: "forward_arg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `...`",
                            "",
                            "```text",
                            "def m(...); end",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a special `...` argument that forwards positional/keyword/block arguments."
            ],
        },

        Node {
            struct_name: "ForwardedArgs",
            str_type: "forwarded_args",
            filename: "forwarded_args",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `...`",
                            "",
                            "```text",
                            "def m(...); foo(...); end",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `...` operator that contains forwarded argument (see `ForwardArg`)"
            ],
        },

        Node {
            struct_name: "Gvar",
            str_type: "gvar",
            filename: "gvar",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the global variable, `String(\"$foo\")` for `$foo`"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "$foo",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents access to global variable (i.e. `$foo`)"
            ],
        },

        Node {
            struct_name: "Gvasgn",
            str_type: "gvasgn",
            filename: "gvasgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the global variable, `String(\"$foo\")` for `$foo`"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Value that is assigned to global variable, `Int(\"42\")` for `$foo = 42`",
                            "",
                            "`None` if global variable assignment is a part of the multi-assignment.",
                            "In such case `value` is a part of the `Masgn` node."
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the global variable name",
                            "",
                            "```text",
                            "$foo = 42",
                            "~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator",
                            "",
                            "```text",
                            "$foo = 42",
                            "~",
                            "```",
                            "",
                            "`None` if global variable assignment is a part of the multi-assignment.",
                            "In such case `=` operator belongs to the `Masgn` node."
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "$foo = 42",
                            "~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents global variable assignment (i.e. `$foo = 42`)"
            ],
        },

        Node {
            struct_name: "Hash",
            str_type: "hash",
            filename: "hash",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "pairs",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of key-value pairs"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "{ a: 1 }",
                            "~",
                            "```",
                            "",
                            "`None` if hash literal is implicit, e.g. `foo(key: \"value\")`"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "{ a: 1 }",
                            "~",
                            "```",
                            "",
                            "`None` if hash literal is implicit, e.g. `foo(key: \"value\")`"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "{ a: 1 }",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a hash literal (i.e. `{ foo: 42 }`)"
            ],
        },

        Node {
            struct_name: "Kwargs",
            str_type: "kwargs",
            filename: "kwargs",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "pairs",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of key-value pairs"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo(bar: 1)",
                            "~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents kwargs that are given to a method call, super or yield (i.e. `foo(bar: 1)`)"
            ],
        },

        Node {
            struct_name: "HashPattern",
            str_type: "hash_pattern",
            filename: "hash_pattern",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "elements",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of inner patterns"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "case foo; in { a: 1 }; end",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "case foo; in { a: 1 }; end",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case foo; in { a: 1 }; end",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a hash pattern used in pattern matching (i.e. `in { a: 1 }`)"
            ],
        },

        Node {
            struct_name: "Heredoc",
            str_type: "dstr",
            filename: "heredoc",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "parts",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of string parts (static literals and interpolated expressions)"
                        ],
                    },

                    NodeField {
                        field_name: "heredoc_body_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the here-document body",
                            "",
                            "```text",
                            "<<-HERE\\n  a\\n   #{42}\\nHERE",
                            "~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "heredoc_end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the here-document end",
                            "",
                            "```text",
                            "<<-HERE\\n  a\\n   #{42}\\nHERE",
                            "~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the here-document identifier",
                            "",
                            "```text",
                            "<<-HERE\\n  a\\n   #{42}\\nHERE",
                            "~~~~~~~",
                            "```",
                            "",
                            "**Note**: This is the only node (with `XHeredoc`) that has `expression_l` smaller that all other sub-locations merged.",
                            "The reason for that is that it's possible to add more code after here-document ID:",
                            "",
                            "```text",
                            "<<-HERE + \"rest\"",
                            "content",
                            "HERE",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a here-document literal (both with and without interpolation)",
                "",
                "It's similar to `Dstr` in terms of abstract syntax tree, but has different source maps."
            ],
        },

        Node {
            struct_name: "If",
            str_type: "if",
            filename: "if_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition given to the `if` statement, `Lvar(\"a\")` for `if a; b; else; c; end`"
                        ],
                    },

                    NodeField {
                        field_name: "if_true",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "True-branch of the `if` statement, `Lvar(\"b\")` for `if a; b; else; c; end`"
                        ],
                    },

                    NodeField {
                        field_name: "if_false",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "False-branch of the `if` statement, `Lvar(\"c\")` for `if a; b; else; c; end`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `if` keyword",
                            "",
                            "```text",
                            "if foo; end",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `then` keyword",
                            "",
                            "```text",
                            "if foo then; end",
                            "~~~~",
                            "```",
                            "",
                            "`None` if `then` keyword is omitted"
                        ],
                    },

                    NodeField {
                        field_name: "else_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `else` keyword",
                            "",
                            "```text",
                            "if foo; else; end",
                            "~~~~",
                            "```",
                            "",
                            "`None` if there's no `else` branch"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "if foo; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "if a then; b; else; c end",
                            "~~~~~~~~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an `if` statement (i.e. `if foo; bar; else; baz; end`)"
            ],
        },

        Node {
            struct_name: "IfGuard",
            str_type: "if_guard",
            filename: "if_guard",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition of the guard, `Lvar(\"foo\")` in `in pattern if guard`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `if` keyword",
                            "",
                            "```text",
                            "case foo; in pattern if cond; end",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case foo; in pattern if cond; end",
                            "~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an `if` guard used in pattern matching (i.e. `case foo; in pattern if guard; end`)"
            ],
        },

        Node {
            struct_name: "IfMod",
            str_type: "if",
            filename: "if_mod",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition of the modifier"
                        ],
                    },

                    NodeField {
                        field_name: "if_true",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "True-branch of the modifier.",
                            "",
                            "Always set for `if` modifier.",
                            "Always `None` for `unless` modifier."
                        ],
                    },

                    NodeField {
                        field_name: "if_false",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "False-branch of the modifier.",
                            "",
                            "Always set for `unless` modifier.",
                            "Always `None` for `if` modifier."
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `if`/`unless` keyword",
                            "",
                            "```text",
                            "stmt if cond",
                            "~~",
                            "",
                            "stmt unless cond",
                            "~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "stmt if cond",
                            "~~~~~~~~~~~~",
                            "",
                            "stmt unless cond",
                            "~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an `if`/`unless` modifier (i.e. `stmt if cond`)"
            ],
        },

        Node {
            struct_name: "IfTernary",
            str_type: "if",
            filename: "if_ternary",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition of the `if` statement"
                        ],
                    },

                    NodeField {
                        field_name: "if_true",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "True-branch"
                        ],
                    },

                    NodeField {
                        field_name: "if_false",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "True-branch"
                        ],
                    },

                    NodeField {
                        field_name: "question_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `?` operator",
                            "",
                            "```text",
                            "cond ? if_true : if_false",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "colon_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `:` operator",
                            "",
                            "```text",
                            "cond ? if_true : if_false",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "cond ? if_true : if_false",
                            "~~~~~~~~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents ternary `if` statement (i.e. `cond ? if_true : if_false`)"
            ],
        },

        Node {
            struct_name: "IFlipFlop",
            str_type: "iflipflop",
            filename: "iflipflop",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "left",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Left part of the flip-flop. `None` if based on a range without begin (`..bar`)"
                        ],
                    },

                    NodeField {
                        field_name: "right",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Right part of the flip-flop. `None` if based on a range without end (`foo..`)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `..` operator",
                            "",
                            "```text",
                            "if foo..bar; end",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "if foo..bar; end",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents inclusive flip-flop (i.e. in `if foo..bar; end`)"
            ],
        },

        Node {
            struct_name: "MatchPattern",
            str_type: "match_pattern",
            filename: "match_pattern",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Value that is used for matching"
                        ],
                    },

                    NodeField {
                        field_name: "pattern",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Pattern that is used for matching"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `=>` operator",
                            "",
                            "```text",
                            "foo => pattern",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo => pattern",
                            "~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a one-line pattern matching that can throw an error (i.e. `foo => pattern`)"
            ],
        },

        Node {
            struct_name: "MatchPatternP",
            str_type: "match_pattern_p",
            filename: "match_pattern_p",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Value that is used for matching"
                        ],
                    },

                    NodeField {
                        field_name: "pattern",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Pattern that is used for matching"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `in` operator",
                            "",
                            "```text",
                            "foo in pattern",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo in pattern",
                            "~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a one-line pattern matching that never throws but returns true/false (i.e. `foo in pattern`)"
            ],
        },

        Node {
            struct_name: "InPattern",
            str_type: "in_pattern",
            filename: "in_pattern",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "pattern",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Value that is used for matching"
                        ],
                    },

                    NodeField {
                        field_name: "guard",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Guard that is used for matching",
                            "",
                            "Optional, so can be `None`"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the branch that is invoked if value matches pattern"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `in` keyword",
                            "",
                            "```text",
                            "case value; in pattern; end",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `then` keyword",
                            "",
                            "```text",
                            "case value; in pattern then; end",
                            "~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case value; in pattern then; 42; end",
                            "~~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an `in pattern` branch of the pattern matching"
            ],
        },

        Node {
            struct_name: "Index",
            str_type: "index",
            filename: "index",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "recv",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Receiver of indexing"
                        ],
                    },

                    NodeField {
                        field_name: "indexes",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of indexes"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of open bracket",
                            "",
                            "```text",
                            "foo[1, 2, 3]",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of closing bracket",
                            "",
                            "```text",
                            "foo[1, 2, 3]",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo[1, 2, 3]",
                            "~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents indexing operation (i.e. `foo[1,2,3]`)"
            ],
        },

        Node {
            struct_name: "IndexAsgn",
            str_type: "indexasgn",
            filename: "index_asgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "recv",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Receiver of the indexing"
                        ],
                    },

                    NodeField {
                        field_name: "indexes",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of indexes"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Value that is assigned",
                            "",
                            "`None` if assignment is a part of the multi-assignment.",
                            "In such case `value` belongs to `Masgn` node."
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of open bracket",
                            "",
                            "```text",
                            "foo[1, 2, 3] = bar",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of closing bracket",
                            "",
                            "```text",
                            "foo[1, 2, 3] = bar",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator",
                            "",
                            "```text",
                            "foo[1, 2, 3] = bar",
                            "~",
                            "```",
                            "",
                            "`None` if assignment is a part of the multi-assignment.",
                            "In such case operator `=` belongs to `Masgn` node."
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo[1, 2, 3] = bar",
                            "~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents assignment using indexing operation (i.e. `foo[1, 2, 3] = bar`)"
            ],
        },

        Node {
            struct_name: "Int",
            str_type: "int",
            filename: "int",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "String value of the literal, `String(\"42\")` for `42`"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of unary `-` (but not `+`)",
                            "",
                            "```text",
                            "-42",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "-42",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an integer literal (i.e. `42`)"
            ],
        },

        Node {
            struct_name: "Irange",
            str_type: "irange",
            filename: "irange",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "left",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Begin of the range, `None` if range has no `begin` (i.e. `..4`)"
                        ],
                    },

                    NodeField {
                        field_name: "right",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "End of the range, `None` if range has no `end` (i.e. `2..`)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `..` operator",
                            "",
                            "```text",
                            "2..4",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "2..4",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents inclusive range (i.e. `2..4`)"
            ],
        },

        Node {
            struct_name: "Ivar",
            str_type: "ivar",
            filename: "ivar",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the instance variable, `String(\"@foo\")` in `@foo`"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "@foo",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents access to instance variable (i.e. `@foo`)"
            ],
        },

        Node {
            struct_name: "Ivasgn",
            str_type: "ivasgn",
            filename: "ivasgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the instance variable, `String(\"@foo\")` in `@foo = 42`"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Value that is assigned to instance variable.",
                            "",
                            "`None` if instance variable assignment is a part of the multi-assignment.",
                            "In such case `value` is a part of the `Masgn` node."
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the instance variable name.",
                            "",
                            "```text",
                            "@foo = 1",
                            "~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator.",
                            "",
                            "```text",
                            "@foo = 1",
                            "~",
                            "```",
                            "",
                            "`None` if instance variable assignment is a part of the multi-assignment.",
                            "In such case `value` is a part of the `Masgn` node."
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "@foo = 42",
                            "~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents instance variable assignment (i.e `@foo = 42`)"
            ],
        },

        Node {
            struct_name: "Kwarg",
            str_type: "kwarg",
            filename: "kwarg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the keyword argument"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the name",
                            "",
                            "```text",
                            "def foo(bar:); end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def foo(bar:); end",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents required keyword argument (i.e. `foo` in `def m(foo:); end`)"
            ],
        },

        Node {
            struct_name: "KwBegin",
            str_type: "kwbegin",
            filename: "kwbegin",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "statements",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of statements"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `begin` keyword",
                            "",
                            "```text",
                            "begin; foo; end",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `begin` keyword",
                            "",
                            "```text",
                            "begin; foo; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "begin; foo; bar",
                            "~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an explicit `begin; end` block.",
                "",
                "The reason why it's different is that only",
                "```text",
                "begin; foo; end while cond",
                "```",
                "is a post-while loop (same with post-until loop)"
            ],
        },

        Node {
            struct_name: "Kwnilarg",
            str_type: "kwnilarg",
            filename: "kwnilarg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `nil`",
                            "",
                            "```text",
                            "def m(**nil); end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `nil`",
                            "",
                            "```text",
                            "def m(**nil); end",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an special argument that rejects all keyword arguments (i.e. `def m(**nil); end`)"
            ],
        },

        Node {
            struct_name: "Kwoptarg",
            str_type: "kwoptarg",
            filename: "kwoptarg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the optional keyword argument"
                        ],
                    },

                    NodeField {
                        field_name: "default",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Default value of the optional keyword argument"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the argument name",
                            "",
                            "```text",
                            "def m(foo: 1); end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the argument name",
                            "",
                            "```text",
                            "def m(foo: 1); end",
                            "~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an optional keyword argument (i.e. `foo` in `def m(foo: 42); end`)"
            ],
        },

        Node {
            struct_name: "Kwrestarg",
            str_type: "kwrestarg",
            filename: "kwrestarg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::MaybeStr,
                        always_print: false,
                        comment: &[
                            "Name of the keyword rest argument, `String(\"foo\")` in `def m(**foo); end`.",
                            "",
                            "`None` if argument has no name (`def m(**); end`)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `**` operator",
                            "",
                            "```text",
                            "def m(**foo); end",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the argument name",
                            "",
                            "```text",
                            "def m(**foo); end",
                            "~~~",
                            "```",
                            "",
                            "`None` if argument has no name (`def m(**); end`)"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def m(**foo); end",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a keyword rest argument (i.e. `foo` in `def m(**foo); end`)"
            ],
        },

        Node {
            struct_name: "Kwsplat",
            str_type: "kwsplat",
            filename: "kwsplat",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Value that is converted into a `Hash` using `**`"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `**` operator",
                            "",
                            "```text",
                            "foo(**bar)",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo(**bar)",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a keyword arguments splat (i.e. `**bar` in a call like `foo(**bar)`)"
            ],
        },

        Node {
            struct_name: "Lambda",
            str_type: "lambda",
            filename: "lambda",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `->`",
                            "",
                            "```text",
                            "-> {}",
                            "~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a lambda call using `->` (i.e. `-> {}`)",
                "",
                "Note that `Lambda` is a part of the `Block`, not other way around."
            ],
        },

        Node {
            struct_name: "Line",
            str_type: "__LINE__",
            filename: "line",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `__LINE__` literal",
                            "",
                            "```text",
                            "__LINE__",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a special `__LINE__` literal"
            ],
        },

        Node {
            struct_name: "Lvar",
            str_type: "lvar",
            filename: "lvar",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the local variable"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the local variable",
                            "",
                            "```text",
                            "foo",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents access to a local variable (i.e. `foo`)",
                "",
                "Parser knows that it's a local variable because:",
                "1. there was an assignment to this variable **before** accessing it",
                "2. it's an argument of the current method / block",
                "3. it's been implicitly declared by `MatchWithLvasgn` node",
                "",
                "Otherwise it's a method call (see `Send`)"
            ],
        },

        Node {
            struct_name: "Lvasgn",
            str_type: "lvasgn",
            filename: "lvasgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the local variable"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Value that is assigned to a local variable"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the local variable name",
                            "",
                            "```text",
                            "foo = 42",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator",
                            "",
                            "```text",
                            "foo = 42",
                            "~",
                            "```",
                            "",
                            "`None` if local variable assignment is a part of the multi-assignment.",
                            "In such case `value` is a part of the `Masgn` node."
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo = 42",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents local variable assignment (i.e. `foo = 42`)"
            ],
        },

        Node {
            struct_name: "Masgn",
            str_type: "masgn",
            filename: "masgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "lhs",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Left hand statement of the assignment"
                        ],
                    },

                    NodeField {
                        field_name: "rhs",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Left hand statement of the assignment"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator",
                            "",
                            "```text",
                            "foo, bar = 1, 2",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo, bar = 1, 2",
                            "~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents mass-assignment (i.e. `foo, bar = 1, 2`)"
            ],
        },

        Node {
            struct_name: "MatchAlt",
            str_type: "match_alt",
            filename: "match_alt",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "lhs",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Left pattern"
                        ],
                    },

                    NodeField {
                        field_name: "rhs",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Right pattern"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `|` operator",
                            "",
                            "```text",
                            "foo in 1 | 2",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo in 1 | 2",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents pattern matching using one of the given patterns (i.e. `foo in 1 | 2`)"
            ],
        },

        Node {
            struct_name: "MatchAs",
            str_type: "match_as",
            filename: "match_as",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Pattern that is used for matching"
                        ],
                    },

                    NodeField {
                        field_name: "as_",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Variable that is assigned if matched (see `MatchVar` node)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `=>` operator",
                            "",
                            "```text",
                            "case 1; in Integer => a; end",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `=>` operator",
                            "",
                            "```text",
                            "case 1; in Integer => a; end",
                            "~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents matching with renaming into specified local variable (i.e. `case 1; in Integer => a; end`)"
            ],
        },

        Node {
            struct_name: "MatchCurrentLine",
            str_type: "match_current_line",
            filename: "match_current_line",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "re",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Given regex"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the regex",
                            "",
                            "```text",
                            "if /re/; end",
                            "~~~~",
                            "```",
                            "",
                            "Technically this location is redundant, but keeping it is the only way to",
                            "have the same interface for all nodes."
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents implicit matching using `if /regex/`",
                "",
                "```text",
                "if /.*/",
                "puts 'true'",
                "else",
                "puts 'false'",
                "end",
                "```",
                "Prints \"false\".",
                "",
                "Under the hood this construction matches regex against `$_`, so the following works:",
                "```text",
                "$_ = 'match_me'",
                "if /match_me/",
                "puts 'true'",
                "else",
                "puts 'false'",
                "end",
                "```",
                "this code prints \"true\"."
            ],
        },

        Node {
            struct_name: "MatchNilPattern",
            str_type: "match_nil_pattern",
            filename: "match_nil_pattern",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `**` operator",
                            "",
                            "```text",
                            "in **nil",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the name",
                            "",
                            "```text",
                            "in **nil",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "in **nil",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents empty hash pattern that is used in pattern matching (i.e. `in **nil`)"
            ],
        },

        Node {
            struct_name: "MatchRest",
            str_type: "match_rest",
            filename: "match_rest",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Name of the variable name",
                            "",
                            "`None` if there's no name (i.e. `in *`)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `*` operator",
                            "",
                            "```text",
                            "case foo; in *bar; end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `*` operator",
                            "",
                            "```text",
                            "case foo; in *bar; end",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a wildcard pattern used in pattern matching (i.e. `in *foo`)"
            ],
        },

        Node {
            struct_name: "MatchVar",
            str_type: "match_var",
            filename: "match_var",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the variable that is assigned if matching succeeds"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the name",
                            "",
                            "```text",
                            "case foo; in pattern => bar; end",
                            "~~~",
                            "```",
                            "",
                            "**Note** it can also be produced by a hash pattern",
                            "",
                            "```text",
                            "case foo; in { a: }; end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case foo; in pattern => bar; end",
                            "~~~",
                            "```",
                            "",
                            "**Note** it can also be produced by a hash pattern",
                            "",
                            "```text",
                            "case foo; in { a: }; end",
                            "~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents matching with assignment into a local variable (i.e. `pattern => var`)"
            ],
        },

        Node {
            struct_name: "MatchWithLvasgn",
            str_type: "match_with_lvasgn",
            filename: "match_with_lvasgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "re",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Regex that is used for matching"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Value that is used for matching"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `=~` operatir",
                            "",
                            "```text",
                            "/(?<match>bar)/ =~ 'bar'",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "/(?<match>bar)/ =~ 'bar'",
                            "~~~~~~~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents matching a regex that produces local variables (i.e. `/(?<match>bar)/ =~ 'bar'`)",
                "",
                "Each named group in regex declares a local variable."
            ],
        },

        Node {
            struct_name: "Mlhs",
            str_type: "mlhs",
            filename: "mlhs",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "items",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of items that are assigned"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "(a, b) = 1, 2",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "(a, b) = 1, 2",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "(a, b) = 1, 2",
                            "~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents left hand statement of the mass-assignment (i.e. `foo, bar` in `foo, bar = 1, 2`)"
            ],
        },

        Node {
            struct_name: "Module",
            str_type: "module",
            filename: "module",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Name of the module"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the module",
                            "",
                            "`None` if module has no body"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `module` keyword",
                            "",
                            "```text",
                            "module M; end",
                            "~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "module M; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "module M; end",
                            "~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents module declaration using `module` keyword"
            ],
        },

        Node {
            struct_name: "Next",
            str_type: "next",
            filename: "next",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "Arguments given to `next`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `next` keyword",
                            "",
                            "```text",
                            "next 42",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "next(42)",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `next` keyword"
            ],
        },

        Node {
            struct_name: "Nil",
            str_type: "nil",
            filename: "nil",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `nil` keyword",
                            "",
                            "```text",
                            "nil",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `nil` literal"
            ],
        },

        Node {
            struct_name: "NthRef",
            str_type: "nth_ref",
            filename: "nth_ref",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::RawString,
                        always_print: false,
                        comment: &[
                            "Name of the variable, `String(\"1\")` for `$1`"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "$1",
                            "~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents numeric global variable (e.g. `$1`)"
            ],
        },

        Node {
            struct_name: "Numblock",
            str_type: "numblock",
            filename: "numblock",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "call",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Method call that takes a block"
                        ],
                    },

                    NodeField {
                        field_name: "numargs",
                        field_type: NodeFieldType::U8,
                        always_print: false,
                        comment: &[
                            "Number of parameters that block takes"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Block body"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the open brace",
                            "",
                            "```text",
                            "proc { _1 }",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the closing brace",
                            "",
                            "```text",
                            "proc { _1 }",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the open brace",
                            "",
                            "```text",
                            "proc { _1 }",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a block that takes numbered parameters (i.e. `proc { _1 }`)"
            ],
        },

        Node {
            struct_name: "OpAsgn",
            str_type: "op_asgn",
            filename: "op_asgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "recv",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Left hand statement of the assignment"
                        ],
                    },

                    NodeField {
                        field_name: "operator",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Operator, can be one of:",
                            "1. `+=`",
                            "2. `-=`",
                            "3. `*=`",
                            "4. `/=`",
                            "5. `|=`",
                            "6. `&=`",
                            "7. `>>=`",
                            "8. `<<=`",
                            "9. `%=`",
                            "10. `^=`",
                            "11. `**=`"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Right hand statement of the assignment"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the operator",
                            "",
                            "```text",
                            "a.b <<= c",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the operator",
                            "",
                            "```text",
                            "a.b <<= c",
                            "~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an operation with assignment (e.g. `a += 1`)"
            ],
        },

        Node {
            struct_name: "Optarg",
            str_type: "optarg",
            filename: "optarg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the argument"
                        ],
                    },

                    NodeField {
                        field_name: "default",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Default value of the argument"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the argument name",
                            "",
                            "```text",
                            "def m(foo = 1); end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `=` operator",
                            "",
                            "```text",
                            "def m(foo = 1); end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def m(foo = 1); end",
                            "~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents optional positional argument (i.e. `foo` in `m(foo = 1)`)"
            ],
        },

        Node {
            struct_name: "Or",
            str_type: "or",
            filename: "or",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "lhs",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Left hand statement"
                        ],
                    },

                    NodeField {
                        field_name: "rhs",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Right hand statement"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `||`/`or` operator",
                            "",
                            "```text",
                            "foo || bar",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo || bar",
                            "~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `foo || bar` (or `foo or bar`) statement."
            ],
        },

        Node {
            struct_name: "OrAsgn",
            str_type: "or_asgn",
            filename: "or_asgn",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "recv",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Left hand statement"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Right hand statement"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `||=` operator",
                            "",
                            "```text",
                            "foo ||= bar",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo ||= bar",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `lhs ||= rhs` assignment"
            ],
        },

        Node {
            struct_name: "Pair",
            str_type: "pair",
            filename: "pair",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "key",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Key of the pair"
                        ],
                    },

                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Value of the pair"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `:` or `=>` operator",
                            "",
                            "```text",
                            "{ foo: bar }",
                            "~",
                            "",
                            "{ :foo => bar }",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "{ foo: bar }",
                            "~~~~~~~~",
                            "",
                            "{ :foo => bar }",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a key/value pair (e.g. a part of the `Hash` node)"
            ],
        },

        Node {
            struct_name: "Pin",
            str_type: "pin",
            filename: "pin",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "var",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Variable that is pinned"
                        ],
                    },

                    NodeField {
                        field_name: "selector_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `^` operator",
                            "",
                            "```text",
                            "case foo; in ^bar; end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case foo; in ^bar; end",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a pattern based on a \"pinned\" variable (e.g. `^foo`)"
            ],
        },

        Node {
            struct_name: "Postexe",
            str_type: "postexe",
            filename: "postexe",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Body of the block"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `END` keyword",
                            "",
                            "```text",
                            "END { 42 }",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "END { 42 }",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "END { 42 }",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "END { 42 }",
                            "~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `END { .. }` statement"
            ],
        },

        Node {
            struct_name: "Preexe",
            str_type: "preexe",
            filename: "preexe",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Body of the block"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `BEGIN` keyword",
                            "",
                            "```text",
                            "BEGIN { 42 }",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "BEGIN { 42 }",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "BEGIN { 42 }",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "BEGIN { 42 }",
                            "~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `BEGIN { ... }` statement"
            ],
        },

        Node {
            struct_name: "Procarg0",
            str_type: "procarg0",
            filename: "procarg0",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "Parts of the sole block argument.",
                            "",
                            "`proc { |(a, b)| }` also counts as a sole argument, so this list may contain:",
                            "1. A single `Arg` node (for `proc { |a| }` case)",
                            "2. Multiple `Arg` nodes  (for `proc { |(a, b, c)| }` case)"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "proc { |(foo, bar)| }",
                            "~",
                            "```",
                            "",
                            "`None` if there's only one argument"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "proc { |(foo, bar)| }",
                            "~",
                            "```",
                            "",
                            "`None` if there's only one argument"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "proc { |(foo, bar)| }",
                            "~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a sole block argument (e.g. `|foo|`)",
                "",
                "Block that takes a single array argument automatically expands it.",
                "Adding trailing comma after block argument disables this behavior (and then the only argument is emitted as `Arg`)."
            ],
        },

        Node {
            struct_name: "Rational",
            str_type: "rational",
            filename: "rational",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "String value of the literal, `String(\"1r\")` for `1r`"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the unary `-` (but not `+`)",
                            "",
                            "```text",
                            "-1r",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "-1r",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents rational literal (e.g. `1r`)"
            ],
        },

        Node {
            struct_name: "Redo",
            str_type: "redo",
            filename: "redo",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "redo",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `redo` keyword"
            ],
        },

        Node {
            struct_name: "RegOpt",
            str_type: "regopt",
            filename: "reg_opt",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "options",
                        field_type: NodeFieldType::Chars,
                        always_print: false,
                        comment: &[
                            "A list of flags"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "/foo/mix",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents flags of the regex literal (i.e. `mix` for `/foo/mix`)"
            ],
        },

        Node {
            struct_name: "Regexp",
            str_type: "regexp",
            filename: "regexp",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "parts",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of static and dynamic regex parts"
                        ],
                    },

                    NodeField {
                        field_name: "options",
                        field_type: NodeFieldType::RegexOptions,
                        always_print: false,
                        comment: &[
                            "Regex options.",
                            "",
                            "`None` if regex has no explicit flags"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the regex begin",
                            "",
                            "```text",
                            "/foo/",
                            "~",
                            "",
                            "%r{foo}",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the regex end",
                            "",
                            "```text",
                            "/foo/",
                            "~",
                            "",
                            "%r{foo}",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "/foo/mix",
                            "~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents regex literal (e.g. `/foo/`)"
            ],
        },

        Node {
            struct_name: "Rescue",
            str_type: "rescue",
            filename: "rescue",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the block that is wrapped into `rescue` (i.e. the part that may throw an error)"
                        ],
                    },

                    NodeField {
                        field_name: "rescue_bodies",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of `rescue` handlers (see `RescueBody` node)"
                        ],
                    },

                    NodeField {
                        field_name: "else_",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Else branch.",
                            "",
                            "`None` if there's no `else` branch"
                        ],
                    },

                    NodeField {
                        field_name: "else_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `else` keyword",
                            "",
                            "```text",
                            "begin; 1; rescue StandardError => e; 2; else; 3; end",
                            "~~~~",
                            "```",
                            "",
                            "`None` if there's no `else` branch"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "begin; 1; rescue StandardError => e; 2; else; 3; end",
                            "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
                            "```",
                            "",
                            "**Note**: `begin/end` keywords belong to `KwBegin` node"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `rescue` block"
            ],
        },

        Node {
            struct_name: "RescueBody",
            str_type: "resbody",
            filename: "rescue_body",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "exc_list",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "A list of exception classes",
                            "",
                            "`None` if no classes specified (i.e. `rescue => e; ...` or just `rescue; ...`)"
                        ],
                    },

                    NodeField {
                        field_name: "exc_var",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Variable that captures exception",
                            "",
                            "`None` if no variable specified (i.e. `rescue E; ...` or just `rescue; ... `)"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the handler"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `rescue` keyword",
                            "",
                            "```text",
                            "begin; 1; rescue E => e; 2; end",
                            "~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "assoc_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `=>` operator",
                            "",
                            "```text",
                            "begin; 1; rescue E => e; 2; end",
                            "~~",
                            "```",
                            "",
                            "`None` if exception is not captured."
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `then` keyword",
                            "",
                            "```text",
                            "begin; 1; rescue E => e then; 2; end",
                            "~~~~",
                            "```",
                            "",
                            "`then` is optional, so `begin_l` can be `None`"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "begin; 1; rescue E => e then; 2; end",
                            "~~~~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a single `rescue` handler (i.e. `rescue E => e ...`)"
            ],
        },

        Node {
            struct_name: "Restarg",
            str_type: "restarg",
            filename: "restarg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::MaybeStr,
                        always_print: false,
                        comment: &[
                            "Name of the argument.",
                            "",
                            "`None` if argument has no name (i.e. `def m(*); end`)"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `*` operator",
                            "",
                            "```text",
                            "def m(*foo); end",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "name_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the argument name",
                            "",
                            "```text",
                            "def m(*foo); end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "def m(*foo); end",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents positional rest argument (i.e. `*foo` in `def m(*foo); end`)"
            ],
        },

        Node {
            struct_name: "Retry",
            str_type: "retry",
            filename: "retry",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `retry` keyword",
                            "",
                            "```text",
                            "retry",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `retry` keyword"
            ],
        },

        Node {
            struct_name: "Return",
            str_type: "return",
            filename: "return_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of values that is returned"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `return` keyword",
                            "",
                            "```text",
                            "return 1, 2",
                            "~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "return 1, 2",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `return` keyword"
            ],
        },

        Node {
            struct_name: "SClass",
            str_type: "sclass",
            filename: "sclass",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expr",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Expression that is used to get a singleton class",
                            "",
                            "`Lvar(\"foo\")` for `class << foo; end`"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the block"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `class` keyword",
                            "",
                            "```text",
                            "class << foo; end",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `<<` operator",
                            "",
                            "```text",
                            "class << foo; end",
                            "~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "class << foo; end",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "class << foo; end",
                            "~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents opening a singleton class (i.e. `class << foo; ... end;`)"
            ],
        },

        Node {
            struct_name: "Self_",
            str_type: "self",
            filename: "self_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `self` keyword",
                            "",
                            "```text",
                            "self",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `self` keyword"
            ],
        },

        Node {
            struct_name: "Send",
            str_type: "send",
            filename: "send",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "recv",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Receiver of the method call",
                            "",
                            "`None` for implicit method call (e.g. `foo(42)`)"
                        ],
                    },

                    NodeField {
                        field_name: "method_name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the method that is called"
                        ],
                    },

                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of arguments"
                        ],
                    },

                    NodeField {
                        field_name: "dot_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `.` operator",
                            "",
                            "```text",
                            "foo.bar(42)",
                            "~",
                            "```",
                            "",
                            "`None` for implicit method call (e.g. `foo(42)`)"
                        ],
                    },

                    NodeField {
                        field_name: "selector_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the method name",
                            "",
                            "```text",
                            "foo.bar(42)",
                            "~~~",
                            "```",
                            "",
                            "`None` in a very special case when method call is implicit (i.e. `foo.(42)`)"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of open parenthesis",
                            "",
                            "```text",
                            "foo(42)",
                            "~",
                            "```",
                            "",
                            "`None` if there are parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of closing parenthesis",
                            "",
                            "```text",
                            "foo(42)",
                            "~",
                            "```",
                            "",
                            "`None` if there are parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the operator if method is a setter",
                            "",
                            "```text",
                            "foo.bar = 42",
                            "~",
                            "```",
                            "",
                            "`None` otherwise"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo.bar(42)",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a method call (e.g. `foo.bar(42)`)"
            ],
        },

        Node {
            struct_name: "Shadowarg",
            str_type: "shadowarg",
            filename: "shadowarg",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::Str,
                        always_print: false,
                        comment: &[
                            "Name of the argument"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the argument",
                            "",
                            "```text",
                            "proc { |;foo|}",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a special block argument that \"shadows\" outer variable (i.e. `|;foo|`)"
            ],
        },

        Node {
            struct_name: "Splat",
            str_type: "splat",
            filename: "splat",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: false,
                        comment: &[
                            "Value that is converted to array"
                        ],
                    },

                    NodeField {
                        field_name: "operator_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `*` operator",
                            "",
                            "```text",
                            "foo(*bar)",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "foo(*bar)",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an arguments splat (i.e. `*bar` in a call like `foo(*bar)`)"
            ],
        },

        Node {
            struct_name: "Str",
            str_type: "str",
            filename: "str_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "value",
                        field_type: NodeFieldType::StringValue,
                        always_print: false,
                        comment: &[
                            "Value of the string literal",
                            "",
                            "Note that it's a `StringValue`, not a `String`.",
                            "The reason is that you can get UTF-8 incompatible strings",
                            "from a valid UTF-8 source using escape sequences like `\"\\xFF\"`",
                            "",
                            "These \"\\\", \"x\", \"F\", \"F\" chars are valid separately, but together",
                            "they construct a char with code = 255 that is invalid for UTF-8.",
                            "",
                            "You can use `to_string_lossy` or `to_string` methods to get a raw string value."
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the string begin",
                            "",
                            "```text",
                            "\"foo\"",
                            "~",
                            "```",
                            "",
                            "`None` if string literal is a part of the words array (like `%w[foo bar baz]`)"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the string begin",
                            "",
                            "```text",
                            "\"foo\"",
                            "~",
                            "```",
                            "",
                            "`None` if string literal is a part of the words array (like `%w[foo bar baz]`)"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "\"foo\"",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a plain non-interpolated string literal (e.g. `\"foo\"`)"
            ],
        },

        Node {
            struct_name: "Super",
            str_type: "super",
            filename: "super_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of arguments given to `super`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `super` keyword",
                            "",
                            "```text",
                            "super(1, 2)",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "super(1, 2)",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "super(1, 2)",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "super(1, 2)",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `super` keyword"
            ],
        },

        Node {
            struct_name: "Sym",
            str_type: "sym",
            filename: "sym",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "name",
                        field_type: NodeFieldType::StringValue,
                        always_print: false,
                        comment: &[
                            "Value of the symbol literal",
                            "",
                            "Note that it's a `StringValue`, not a `String`.",
                            "The reason is that you can get UTF-8 incompatible strings",
                            "from a valid UTF-8 source using escape sequences like `\"\\xFF\"`",
                            "",
                            "These \"\\\", \"x\", \"F\", \"F\" chars are valid separately, but together",
                            "they construct a char with code = 255 that is invalid for UTF-8.",
                            "",
                            "You can use `to_string_lossy` or `to_string` methods to get a raw symbol value."
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the symbol begin",
                            "",
                            "```text",
                            ":foo",
                            "~",
                            "```",
                            "",
                            "`None` if symbol is a label (`{ foo: 1 }`) or a part of the symbols array (`%i[foo bar baz]`)"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the symbol end",
                            "",
                            "```text",
                            "{ 'foo': 1 }",
                            "~",
                            "```",
                            "",
                            "`None` if symbol is **not** a string label (`:foo`) or a part of the symbols array (`%i[foo bar baz]`)"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            ":foo",
                            "~~~~",
                            "",
                            "{ foo: 1 }",
                            "~~~~",
                            "",
                            "%i[foo]",
                            "~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a plain symbol literal (i.e. `:foo`)",
                "",
                "Note that `:` in `{ foo: bar }` belongs to a `pair` node."
            ],
        },

        Node {
            struct_name: "True",
            str_type: "true",
            filename: "true_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `true` keyword",
                            "",
                            "```text",
                            "true",
                            "~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `true` literal"
            ],
        },

        Node {
            struct_name: "Undef",
            str_type: "undef",
            filename: "undef",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "names",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of names to `undef`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location the `undef` keyword",
                            "",
                            "```text",
                            "undef foo, :bar",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "undef :foo, bar",
                            "~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an `undef` keyword (e.g. `undef foo, :bar`)"
            ],
        },

        Node {
            struct_name: "UnlessGuard",
            str_type: "unless_guard",
            filename: "unless_guard",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition of the guard, `Lvar(\"foo\")` in `in pattern unless guard`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `unless` keyword",
                            "",
                            "```text",
                            "case foo; in pattern unless cond; end",
                            "~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case foo; in pattern unless cond; end",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an `unless` guard used in pattern matching (i.e. `in pattern unless guard`)"
            ],
        },

        Node {
            struct_name: "Until",
            str_type: "until",
            filename: "until",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition of the loop"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the loop.",
                            "",
                            "`None` if body is empty"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `until` keyword",
                            "",
                            "```text",
                            "until cond do; foo; end",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `do` keyword",
                            "",
                            "```text",
                            "until cond do; foo; end",
                            "~~",
                            "```",
                            "",
                            "`do` is optional, and so `begin_l` can be `None`"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "until cond do; foo; end",
                            "~~~",
                            "```",
                            "",
                            "`None` if loop is a modifier (i.e. `foo until bar`)"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "until cond do; foo; end",
                            "~~~~~~~~~~~~~~~~~~~~~~~",
                            "",
                            "foo until bar",
                            "~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `until` loop"
            ],
        },

        Node {
            struct_name: "UntilPost",
            str_type: "until_post",
            filename: "until_post",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition of the loop"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Body of the loop"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `until` keyword",
                            "",
                            "```text",
                            "begin; foo; end until bar",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `until` keyword",
                            "",
                            "```text",
                            "begin; foo; end until bar",
                            "~~~~~~~~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a post-until loop",
                "",
                "```text",
                "begin",
                "foo",
                "end until bar",
                "```"
            ],
        },

        Node {
            struct_name: "When",
            str_type: "when",
            filename: "when",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "patterns",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of values to compare/match against"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the `when` branch"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `when` keyword",
                            "",
                            "```text",
                            "case foo; when bar; end",
                            "~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `then` keyword",
                            "",
                            "```text",
                            "case foo; when bar then baz; end",
                            "~~~~",
                            "```",
                            "",
                            "`then` is optional, and so `begin_l` can be `None`"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "case foo; when bar then baz; end",
                            "~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a branch of the `case` statement (i.e. `when foo`)"
            ],
        },

        Node {
            struct_name: "While",
            str_type: "while",
            filename: "while_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition of the loop"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::MaybeNode,
                        always_print: true,
                        comment: &[
                            "Body of the loop.",
                            "",
                            "`None` if body is empty"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `while` keyword",
                            "",
                            "```text",
                            "while cond do; foo; end",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `do` keyword",
                            "",
                            "```text",
                            "while cond do; foo; end",
                            "~~",
                            "```",
                            "",
                            "`do` is optional, and so `begin_l` can be `None`"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the `end` keyword",
                            "",
                            "```text",
                            "while cond do; foo; end",
                            "~~~",
                            "```",
                            "",
                            "`None` if loop is a modifier (i.e. `foo while bar`)"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "while cond do; foo; end",
                            "~~~~~~~~~~~~~~~~~~~~~~~",
                            "",
                            "foo while bar",
                            "~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents `while` loop"
            ],
        },

        Node {
            struct_name: "WhilePost",
            str_type: "while_post",
            filename: "while_post",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "cond",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Condition of the loop"
                        ],
                    },

                    NodeField {
                        field_name: "body",
                        field_type: NodeFieldType::Node,
                        always_print: false,
                        comment: &[
                            "Body of the loop"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `while` keyword",
                            "",
                            "```text",
                            "begin; foo; end while bar",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `while` keyword",
                            "",
                            "```text",
                            "begin; foo; end while bar",
                            "~~~~~~~~~~~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a post-while loop",
                "",
                "```text",
                "begin",
                "foo",
                "end while bar",
                "```"
            ],
        },

        Node {
            struct_name: "XHeredoc",
            str_type: "xstr",
            filename: "x_heredoc",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "parts",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of string parts (static literals and interpolated expressions)"
                        ],
                    },

                    NodeField {
                        field_name: "heredoc_body_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the executable here-document body",
                            "",
                            "```text",
                            "<<-`HERE`\\n  a\\n   #{42}\\nHERE",
                            "~~~~~~~~~~~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "heredoc_end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the executable here-document end",
                            "",
                            "```text",
                            "<<-`HERE`\\n  a\\n   #{42}\\nHERE",
                            "~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the executable here-document identifier",
                            "",
                            "```text",
                            "<<-`HERE`\\n  a\\n   #{42}\\nHERE",
                            "~~~~~~~",
                            "```",
                            "",
                            "**Note**: This is the only node (with `Heredoc`) that has `expression_l` smaller that all other sub-locations merged.",
                            "The reason for that is that it's possible to add more code after here-document ID:",
                            "",
                            "```text",
                            "<<-`HERE` + \"rest\"",
                            "content",
                            "HERE",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a executable here-document literal (both with and without interpolation)",
                "",
                "It's similar to `Xstr` in terms of abstract syntax tree, but has different source maps."
            ],
        },

        Node {
            struct_name: "Xstr",
            str_type: "xstr",
            filename: "xstr",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "parts",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of string parts (static literals and interpolated expressions)"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the string begin",
                            "",
                            "```text",
                            "`#{foo}`",
                            "~",
                            "",
                            "%X{#{foo}}",
                            "~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the string end",
                            "",
                            "```text",
                            "`#{foo}`",
                            "~",
                            "",
                            "%X{#{foo}}",
                            "~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "`#{foo}`",
                            "~~~~~~~~",
                            "",
                            "%X{#{foo}}",
                            "~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an executable string (i.e. `` `sh #{script_name}` ``)"
            ],
        },

        Node {
            struct_name: "Yield",
            str_type: "yield",
            filename: "yield_",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "args",
                        field_type: NodeFieldType::Nodes,
                        always_print: false,
                        comment: &[
                            "A list of arguments given to `yield`"
                        ],
                    },

                    NodeField {
                        field_name: "keyword_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `yield` keyword",
                            "",
                            "```text",
                            "yield 1, 2",
                            "~~~~~",
                            "```"
                        ],
                    },

                    NodeField {
                        field_name: "begin_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the open parenthesis",
                            "",
                            "```text",
                            "yield(1, 2)",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "end_l",
                        field_type: NodeFieldType::MaybeLoc,
                        always_print: false,
                        comment: &[
                            "Location of the closing parenthesis",
                            "",
                            "```text",
                            "yield(1, 2)",
                            "~",
                            "```",
                            "",
                            "`None` if there are no parentheses"
                        ],
                    },

                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the full expression",
                            "",
                            "```text",
                            "yield(1, 2)",
                            "~~~~~~~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents an `yield` keyword"
            ],
        },

        Node {
            struct_name: "ZSuper",
            str_type: "zsuper",
            filename: "zsuper",
            fields: NodeFieldList(
                &[
                    NodeField {
                        field_name: "expression_l",
                        field_type: NodeFieldType::Loc,
                        always_print: false,
                        comment: &[
                            "Location of the `super` keyword",
                            "",
                            "```text",
                            "super",
                            "~~~~~",
                            "```"
                        ],
                    },

                ]
            ),
            comment: &[
                "Represents a `super` call without arguments and parentheses",
                "",
                "It's different from `super()` as it implicitly forwards current arguments"
            ],
        },

    ]
);
