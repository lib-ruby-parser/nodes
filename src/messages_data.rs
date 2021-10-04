use crate::{Message, MessageField, MessageFieldList, MessageFieldType, MessagesList};

pub const ALL_MESSAGES: MessagesList = MessagesList(&[
    //
    // Lexer errors
    //
    Message {
        camelcase_name: "FractionAfterNumeric",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code", "```text", "1.2.3", "```"],
    },
    Message {
        camelcase_name: "NoDigitsAfterDot",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "foo.2", "```"],
    },
    Message {
        camelcase_name: "UnknownTypeOfPercentString",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "%k[foo]", "```"],
    },
    Message {
        camelcase_name: "NumericLiteralWithoutDigits",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "0b", "```"],
    },
    Message {
        camelcase_name: "UnterminatedList",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "%w[foo bar", "```"],
    },
    Message {
        camelcase_name: "UnterminatedRegexp",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "/foo", "```"],
    },
    Message {
        camelcase_name: "UnterminatedString",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "\"foo", "```"],
    },
    Message {
        camelcase_name: "UnterminatedQuotedString",
        fields: MessageFieldList(&[]),
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
        camelcase_name: "InvalidUnicodeEscape",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "\"\\ufoo\"", "```"],
    },
    Message {
        camelcase_name: "TooLargeUnicodeCodepoint",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "\"\\u{999999}\"", "```"],
    },
    Message {
        camelcase_name: "InvalidUnicodeCodepoint",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "\"\\u{d800}\"", "```"],
    },
    Message {
        camelcase_name: "MultipleCodepointAtSingleChar",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "?\\u{41 42}", "```"],
    },
    Message {
        camelcase_name: "InvalidEscapeCharacter",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "\"\\M-\u{1}\"", "```"],
    },
    Message {
        camelcase_name: "InvalidHexEscape",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "\"\\xZZ\"", "```"],
    },
    Message {
        camelcase_name: "UnterminatedHeredoc",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "heredoc_id",
            field_type: MessageFieldType::Str,
            comment: &["Heredoc identifier"],
        }]),
        comment: &["Emitted for code like", "```text", "<<-HERE", "```"],
    },
    Message {
        camelcase_name: "UnterminatedHeredocId",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "<<-\"HERE", "```"],
    },
    Message {
        camelcase_name: "SlashRAtMiddleOfLine",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "eval(\"foo \\r = 42\")",
            "```",
        ],
    },
    Message {
        camelcase_name: "DStarInterpretedAsArgPrefix",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "foo **arg",
            "```",
        ],
    },
    Message {
        camelcase_name: "StarInterpretedAsArgPrefix",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "foo *arg",
            "```",
        ],
    },
    Message {
        camelcase_name: "AmpersandInterpretedAsArgPrefix",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "foo &arg",
            "```",
        ],
    },
    Message {
        camelcase_name: "TripleDotAtEol",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "range = 1...", "```"],
    },
    Message {
        camelcase_name: "ParenthesesIterpretedAsArglist",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "def m (a, b, c); end",
            "```",
        ],
    },
    Message {
        camelcase_name: "AmbiguousFirstArgument",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "operator",
            field_type: MessageFieldType::Byte,
            comment: &["Operator that is ambiguous"],
        }]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "m +foo",
            "```",
        ],
    },
    Message {
        camelcase_name: "AmbiguousOperator",
        fields: MessageFieldList(&[
            MessageField {
                camelcase_name: "operator",
                field_type: MessageFieldType::Str,
                comment: &["Operator that is ambiguous"],
            },
            MessageField {
                camelcase_name: "interpreted_as",
                field_type: MessageFieldType::Str,
                comment: &["Interpretation of this operator"],
            },
        ]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "1 *2",
            "```",
        ],
    },
    Message {
        camelcase_name: "InvalidCharacterSyntax",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "suggestion",
            field_type: MessageFieldType::Str,
            comment: &["Valid syntax sugestions"],
        }]),
        comment: &["Emitted for code like", "```text", "\"\\M- \"", "```"],
    },
    Message {
        camelcase_name: "InvalidOctalDigit",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "09", "```"],
    },
    Message {
        camelcase_name: "TrailingCharInNumber",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "c",
            field_type: MessageFieldType::Byte,
            comment: &["Invalid trailing char"],
        }]),
        comment: &["Emitted for code like", "```text", "0_a", "```"],
    },
    Message {
        camelcase_name: "EmbeddedDocumentMeetsEof",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "=begin", "```"],
    },
    Message {
        camelcase_name: "InvalidChar",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "c",
            field_type: MessageFieldType::Byte,
            comment: &["char"],
        }]),
        comment: &[
            "Emitted for code like",
            "```text",
            "eval(\"\\x01foo\")",
            "```",
        ],
    },
    Message {
        camelcase_name: "IncompleteCharacterSyntax",
        fields: MessageFieldList(&[]),
        comment: &[
            "It is unknown how to trigger this error.",
            "Code that triggers it in MRI can be dead.",
        ],
    },
    Message {
        camelcase_name: "GvarWithoutId",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "$", "```"],
    },
    Message {
        camelcase_name: "InvalidGvarName",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "c",
            field_type: MessageFieldType::Byte,
            comment: &["char after `$`"],
        }]),
        comment: &["Emitted for code like", "```text", "$@", "```"],
    },
    Message {
        camelcase_name: "IvarWithoutId",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "@", "```"],
    },
    Message {
        camelcase_name: "InvalidIvarName",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "c",
            field_type: MessageFieldType::Byte,
            comment: &["char after `@`"],
        }]),
        comment: &["Emitted for code like", "```text", "@1", "```"],
    },
    Message {
        camelcase_name: "CvarWithoutId",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "@@", "```"],
    },
    Message {
        camelcase_name: "InvalidCvarName",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "c",
            field_type: MessageFieldType::Byte,
            comment: &["char after `@@`"],
        }]),
        comment: &["Emitted for code like", "```text", "@@1", "```"],
    },
    Message {
        camelcase_name: "UnknownRegexOptions",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "options",
            field_type: MessageFieldType::Str,
            comment: &["Concatenated unknown options"],
        }]),
        comment: &["Emitted for code like", "```text", "/re/foo", "```"],
    },
    Message {
        camelcase_name: "UnterminatedUnicodeEscape",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "\"\\u{1234\"", "```"],
    },
    Message {
        camelcase_name: "EncodingError",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "error",
            field_type: MessageFieldType::Str,
            comment: &["Error from decoder"],
        }]),
        comment: &["Emitted for code like", "```text", "# encoding: foo", "```"],
    },
    Message {
        camelcase_name: "InvalidMultibyteChar",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitter for code like",
            "```text",
            "eval(\"\\xFF = 42\")",
            "```",
        ],
    },
    //
    // Lexer warnings
    //
    Message {
        camelcase_name: "AmbiguousTernaryOperator",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "condition",
            field_type: MessageFieldType::Str,
            comment: &["Source of the condition expression"],
        }]),
        comment: &["Emitted for code like", "```text", "a ?AA : 2", "```"],
    },
    Message {
        camelcase_name: "AmbiguousRegexp",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "m /foo/", "```"],
    },
    //
    // Parser errors
    //
    Message {
        camelcase_name: "ElseWithoutRescue",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "begin; else; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "BeginNotAtTopLevel",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def f; BEGIN{}; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "AliasNthRef",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "alias $a $1", "```"],
    },
    Message {
        camelcase_name: "CsendInsideMasgn",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "*a&.x = 0", "```"],
    },
    Message {
        camelcase_name: "ClassOrModuleNameMustBeConstant",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "module foo; end", "```"],
    },
    Message {
        camelcase_name: "EndlessSetterDefinition",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "def foo=() = 42", "```"],
    },
    Message {
        camelcase_name: "UnexpectedToken",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "token_name",
            field_type: MessageFieldType::Str,
            comment: &["Name of the token"],
        }]),
        comment: &["Emitted for any code that produces invalid sequence of tokens"],
    },
    Message {
        camelcase_name: "ClassDefinitionInMethodBody",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def a; class Foo; end; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "ModuleDefinitionInMethodBody",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def a; module Foo; end; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "InvalidReturnInClassOrModuleBody",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "class A; return; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "ConstArgument",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def foo(Abc); end",
            "```",
        ],
    },
    Message {
        camelcase_name: "IvarArgument",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def foo(@abc); end",
            "```",
        ],
    },
    Message {
        camelcase_name: "GvarArgument",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def foo($abc); end",
            "```",
        ],
    },
    Message {
        camelcase_name: "CvarArgument",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def foo(@@abc); end",
            "```",
        ],
    },
    Message {
        camelcase_name: "NoSuchLocalVariable",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "var_name",
            field_type: MessageFieldType::Str,
            comment: &["Variable name"],
        }]),
        comment: &[
            "Emitted for code like",
            "```text",
            "case 0; in ^a; true; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "OrdinaryParamDefined",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "m { |a| _1 }", "```"],
    },
    Message {
        camelcase_name: "NumparamUsed",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "foo { _1; bar { _2 }; }",
            "```",
        ],
    },
    Message {
        camelcase_name: "TokAtEolWithoutExpression",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "token_name",
            field_type: MessageFieldType::Str,
            comment: &["Name of the token"],
        }]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "if",
            "42",
            "end",
            "```",
        ],
    },
    //
    // Parser warnings
    //
    Message {
        camelcase_name: "EndInMethod",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def m; END {}; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "ComparisonAfterComparison",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "comparison",
            field_type: MessageFieldType::Str,
            comment: &["Source of the first comparison"],
        }]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "a < b < c",
            "```",
        ],
    },
    //
    // Builder errors
    //
    Message {
        camelcase_name: "CircularArgumentReference",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "arg_name",
            field_type: MessageFieldType::Str,
            comment: &["Name of the argument"],
        }]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def m(foo = foo) end",
            "```",
        ],
    },
    Message {
        camelcase_name: "DynamicConstantAssignment",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def m; FOO = 1; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "CantAssignToSelf",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "self = foo", "```"],
    },
    Message {
        camelcase_name: "CantAssignToNil",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "nil = foo", "```"],
    },
    Message {
        camelcase_name: "CantAssignToTrue",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "true = foo", "```"],
    },
    Message {
        camelcase_name: "CantAssignToFalse",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "false = foo", "```"],
    },
    Message {
        camelcase_name: "CantAssignToFile",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "__FILE__ = foo", "```"],
    },
    Message {
        camelcase_name: "CantAssignToLine",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "__LINE__ = foo", "```"],
    },
    Message {
        camelcase_name: "CantAssignToEncoding",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "__ENCODING__ = foo",
            "```",
        ],
    },
    Message {
        camelcase_name: "CantAssignToNumparam",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "numparam",
            field_type: MessageFieldType::Str,
            comment: &["Source of the numbered parameter"],
        }]),
        comment: &[
            "Emitted for code like",
            "```text",
            "proc {_1; _1 = nil}",
            "```",
        ],
    },
    Message {
        camelcase_name: "CantSetVariable",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "var_name",
            field_type: MessageFieldType::Str,
            comment: &["Source of the read-only variable that is assigned"],
        }]),
        comment: &["Emitted for code like", "```text", "$1 = foo", "```"],
    },
    Message {
        camelcase_name: "BlockGivenToYield",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "yield(&foo)", "```"],
    },
    Message {
        camelcase_name: "BlockAndBlockArgGiven",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "fun(&bar) do end",
            "```",
        ],
    },
    Message {
        camelcase_name: "SymbolLiteralWithInterpolation",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "case a; in \"#{a}\": 1; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "ReservedForNumparam",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "numparam",
            field_type: MessageFieldType::Str,
            comment: &["Numbered parameter that is treated as a local variable"],
        }]),
        comment: &["Emitted for code like", "```text", "_1 = 1", "```"],
    },
    Message {
        camelcase_name: "KeyMustBeValidAsLocalVariable",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "case a; in a?:; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "DuplicateVariableName",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "case 0; in a, a; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "DuplicateKeyName",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "case 0; in a: 1, a: 2; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "SingletonLiteral",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def (1).foo; end",
            "```",
        ],
    },
    Message {
        camelcase_name: "NthRefIsTooBig",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "nth_ref",
            field_type: MessageFieldType::Str,
            comment: &["Source of the nth_ref that is techincally a regular global variable"],
        }]),
        comment: &[
            "Emitted for code like (only in $VERBOSE mode)",
            "```text",
            "$100",
            "```",
        ],
    },
    Message {
        camelcase_name: "DuplicatedArgumentName",
        fields: MessageFieldList(&[]),
        comment: &[
            "Emitted for code like",
            "```text",
            "def foo(aa, aa); end",
            "```",
        ],
    },
    Message {
        camelcase_name: "RegexError",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "error",
            field_type: MessageFieldType::Str,
            comment: &["Error from Onigurama engine"],
        }]),
        comment: &["Emitted for code like", "```text", "/[/", "```"],
    },
    Message {
        camelcase_name: "InvalidSymbol",
        fields: MessageFieldList(&[MessageField {
            camelcase_name: "symbol",
            field_type: MessageFieldType::Str,
            comment: &["Source of the symbol"],
        }]),
        comment: &["Emitted for code like", "```text", "%I\"x .\\xc3.\"", "```"],
    },
    Message {
        camelcase_name: "VoidValueExpression",
        fields: MessageFieldList(&[]),
        comment: &["Emitted for code like", "```text", "a = return", "```"],
    },
]);
