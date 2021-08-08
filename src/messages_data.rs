use crate::{
    BuiltinMessageList, Message, MessageField, MessageFieldList, MessageFieldType, Section,
    SectionList,
};

pub const ALL_SECTIONS: SectionList = SectionList(
    &[
        Section {
            name: "Lexer errors",
            messages: BuiltinMessageList(
                &[
                    Message {
                        name: "FractionAfterNumeric",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code",
                            "```text",
                            "1.2.3",
                            "```",
                        ],
                    },
                    Message {
                        name: "NoDigitsAfterDot",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "foo.2",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnknownTypeOfPercentString",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "%k[foo]",
                            "```",
                        ],
                    },
                    Message {
                        name: "NumericLiteralWithoutDigits",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "0b",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnterminatedList",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "%w[foo bar",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnterminatedRegexp",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "/foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnterminatedString",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "\"foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnterminatedQuotedString",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "%s",
                            "//    ^ EOF, not \"",
                            "",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidUnicodeEscape",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "\"\\ufoo\"",
                            "```",
                        ],
                    },
                    Message {
                        name: "TooLargeUnicodeCodepoint",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "\"\\u{999999}\"",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidUnicodeCodepoint",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "\"\\u{d800}\"",
                            "```",
                        ],
                    },
                    Message {
                        name: "MultipleCodepointAtSingleChar",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "?\\u{41 42}",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidEscapeCharacter",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "\"\\M-\u{1}\"",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidHexEscape",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "\"\\xZZ\"",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnterminatedHeredoc",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "heredoc_id",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Heredoc identifier"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "<<-HERE",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnterminatedHeredocId",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "<<-\"HERE",
                            "```",
                        ],
                    },
                    Message {
                        name: "SlashRAtMiddleOfLine",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "eval(\"foo \\r = 42\")",
                            "```",
                        ],
                    },
                    Message {
                        name: "DStarInterpretedAsArgPrefix",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "foo **arg",
                            "```",
                        ],
                    },
                    Message {
                        name: "StarInterpretedAsArgPrefix",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "foo *arg",
                            "```",
                        ],
                    },
                    Message {
                        name: "AmpersandInterpretedAsArgPrefix",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "foo &arg",
                            "```",
                        ],
                    },
                    Message {
                        name: "TripleDotAtEol",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "range = 1...",
                            "```",
                        ],
                    },
                    Message {
                        name: "ParenthesesIterpretedAsArglist",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "def m (a, b, c); end",
                            "```",
                        ],
                    },
                    Message {
                        name: "AmbiguousFirstArgument",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "operator",
                                    field_type: MessageFieldType::Byte,
                                    comment: &[
                                        "Operator that is ambiguous"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "m +foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "AmbiguousOperator",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "operator",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Operator that is ambiguous"
                                    ],
                                },
                                MessageField {
                                    name: "interpreted_as",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Interpretation of this operator"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "1 *2",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidCharacterSyntax",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "suggestion",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Valid syntax sugestions"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "\"\\M- \"",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidOctalDigit",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "09",
                            "```",
                        ],
                    },
                    Message {
                        name: "TrailingCharInNumber",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "c",
                                    field_type: MessageFieldType::Byte,
                                    comment: &[
                                        "Invalid trailing char"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "0_a",
                            "```",
                        ],
                    },
                    Message {
                        name: "EmbeddedDocumentMeetsEof",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "=begin",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidChar",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "c",
                                    field_type: MessageFieldType::Byte,
                                    comment: &[
                                        "char"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "eval(\"\\x01foo\")",
                            "```",
                        ],
                    },
                    Message {
                        name: "IncompleteCharacterSyntax",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "It is unknown how to trigger this error.",
                            "Code that triggers it in MRI can be dead.",
                        ],
                    },
                    Message {
                        name: "GvarWithoutId",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "$",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidGvarName",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "c",
                                    field_type: MessageFieldType::Byte,
                                    comment: &[
                                        "char after `$`"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "$@",
                            "```",
                        ],
                    },
                    Message {
                        name: "IvarWithoutId",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "@",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidIvarName",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "c",
                                    field_type: MessageFieldType::Byte,
                                    comment: &[
                                        "char after `@`"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "@1",
                            "```",
                        ],
                    },
                    Message {
                        name: "CvarWithoutId",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "@@",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidCvarName",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "c",
                                    field_type: MessageFieldType::Byte,
                                    comment: &[
                                        "char after `@@`"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "@@1",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnknownRegexOptions",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "options",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Concatenated unknown options"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "/re/foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnterminatedUnicodeEscape",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "\"\\u{1234\"",
                            "```",
                        ],
                    },
                    Message {
                        name: "EncodingError",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "error",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Error from decoder"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "# encoding: foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidMultibyteChar",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitter for code like",
                            "```text",
                            "eval(\"\\xFF = 42\")",
                            "```",
                        ],
                    },
                ],
            ),
        },
        Section {
            name: "Lexer warnings",
            messages: BuiltinMessageList(
                &[
                    Message {
                        name: "AmbiguousTernaryOperator",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "condition",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Source of the condition expression"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "a ?AA : 2",
                            "```",
                        ],
                    },
                    Message {
                        name: "AmbiguousRegexp",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "m /foo/",
                            "```",
                        ],
                    },
                ],
            ),
        },
        Section {
            name: "Parser errors",
            messages: BuiltinMessageList(
                &[
                    Message {
                        name: "ElseWithoutRescue",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "begin; else; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "BeginNotAtTopLevel",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def f; BEGIN{}; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "AliasNthRef",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "alias $a $1",
                            "```",
                        ],
                    },
                    Message {
                        name: "CsendInsideMasgn",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "*a&.x = 0",
                            "```",
                        ],
                    },
                    Message {
                        name: "ClassOrModuleNameMustBeConstant",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "module foo; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "EndlessSetterDefinition",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def foo=() = 42",
                            "```",
                        ],
                    },
                    Message {
                        name: "UnexpectedToken",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "token_name",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Name of the token"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for any code that produces invalid sequence of tokens"
                        ],
                    },
                    Message {
                        name: "ClassDefinitionInMethodBody",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def a; class Foo; end; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "ModuleDefinitionInMethodBody",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def a; module Foo; end; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidReturnInClassOrModuleBody",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "class A; return; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "ConstArgument",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def foo(Abc); end",
                            "```",
                        ],
                    },
                    Message {
                        name: "IvarArgument",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def foo(@abc); end",
                            "```",
                        ],
                    },
                    Message {
                        name: "GvarArgument",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def foo($abc); end",
                            "```",
                        ],
                    },
                    Message {
                        name: "CvarArgument",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def foo(@@abc); end",
                            "```",
                        ],
                    },
                    Message {
                        name: "NoSuchLocalVariable",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "var_name",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Variable name"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "case 0; in ^a; true; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "OrdinaryParamDefined",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "m { |a| _1 }",
                            "```",
                        ],
                    },
                    Message {
                        name: "NumparamUsed",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "foo { _1; bar { _2 }; }",
                            "```",
                        ],
                    },
                    Message {
                        name: "TokAtEolWithoutExpression",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "token_name",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Name of the token"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "if",
                            "42",
                            "end",
                            "```",
                        ],
                    },
                ],
            ),
        },
        Section {
            name: "Parser warnings",
            messages: BuiltinMessageList(
                &[
                    Message {
                        name: "EndInMethod",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def m; END {}; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "ComparisonAfterComparison",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "comparison",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Source of the first comparison"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "a < b < c",
                            "```",
                        ],
                    },
                ],
            ),
        },
        Section {
            name: "Builder errors",
            messages: BuiltinMessageList(
                &[
                    Message {
                        name: "CircularArgumentReference",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "arg_name",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Name of the argument"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def m(foo = foo) end",
                            "```",
                        ],
                    },
                    Message {
                        name: "DynamicConstantAssignment",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def m; FOO = 1; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantAssignToSelf",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "self = foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantAssignToNil",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "nil = foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantAssignToTrue",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "true = foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantAssignToFalse",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "false = foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantAssignToFile",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "__FILE__ = foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantAssignToLine",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "__LINE__ = foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantAssignToEncoding",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "__ENCODING__ = foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantAssignToNumparam",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "numparam",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Source of the numbered parameter"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "proc {_1; _1 = nil}",
                            "```",
                        ],
                    },
                    Message {
                        name: "CantSetVariable",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "var_name",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Source of the read-only variable that is assigned"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "$1 = foo",
                            "```",
                        ],
                    },
                    Message {
                        name: "BlockGivenToYield",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "yield(&foo)",
                            "```",
                        ],
                    },
                    Message {
                        name: "BlockAndBlockArgGiven",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "fun(&bar) do end",
                            "```",
                        ],
                    },
                    Message {
                        name: "SymbolLiteralWithInterpolation",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "case a; in \"#{a}\": 1; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "ReservedForNumparam",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "numparam",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Numbered parameter that is treated as a local variable"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "_1 = 1",
                            "```",
                        ],
                    },
                    Message {
                        name: "KeyMustBeValidAsLocalVariable",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "case a; in a?:; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "DuplicateVariableName",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "case 0; in a, a; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "DuplicateKeyName",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "case 0; in a: 1, a: 2; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "SingletonLiteral",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def (1).foo; end",
                            "```",
                        ],
                    },
                    Message {
                        name: "NthRefIsTooBig",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "nth_ref",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Source of the nth_ref that is techincally a regular global variable"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like (only in $VERBOSE mode)",
                            "```text",
                            "$100",
                            "```",
                        ],
                    },
                    Message {
                        name: "DuplicatedArgumentName",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "def foo(aa, aa); end",
                            "```",
                        ],
                    },
                    Message {
                        name: "RegexError",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "error",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Error from Onigurama engine"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "/[/",
                            "```",
                        ],
                    },
                    Message {
                        name: "InvalidSymbol",
                        fields: MessageFieldList(
                            &[
                                MessageField {
                                    name: "symbol",
                                    field_type: MessageFieldType::Str,
                                    comment: &[
                                        "Source of the symbol"
                                    ],
                                },
                            ],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "%I\"x .\\xc3.\"",
                            "```",
                        ],
                    },
                    Message {
                        name: "VoidValueExpression",
                        fields: MessageFieldList(
                            &[],
                        ),
                        comment: &[
                            "Emitted for code like",
                            "```text",
                            "a = return",
                            "```",
                        ],
                    },
                ],
            ),
        },
    ],
);
