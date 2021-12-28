use crate::{Message, MessageField, MessageFieldType, MessagesList};

//
// Lexer errors
//
static FractionAfterNumeric: Message = Message {
    camelcase_name: "FractionAfterNumeric",
    fields: &[],
    comment: &["Emitted for code", "```text", "1.2.3", "```"],
};
static NoDigitsAfterDot: Message = Message {
    camelcase_name: "NoDigitsAfterDot",
    fields: &[],
    comment: &["Emitted for code like", "```text", "foo.2", "```"],
};
static UnknownTypeOfPercentString: Message = Message {
    camelcase_name: "UnknownTypeOfPercentString",
    fields: &[],
    comment: &["Emitted for code like", "```text", "%k[foo]", "```"],
};
static NumericLiteralWithoutDigits: Message = Message {
    camelcase_name: "NumericLiteralWithoutDigits",
    fields: &[],
    comment: &["Emitted for code like", "```text", "0b", "```"],
};
static UnterminatedList: Message = Message {
    camelcase_name: "UnterminatedList",
    fields: &[],
    comment: &["Emitted for code like", "```text", "%w[foo bar", "```"],
};
static UnterminatedRegexp: Message = Message {
    camelcase_name: "UnterminatedRegexp",
    fields: &[],
    comment: &["Emitted for code like", "```text", "/foo", "```"],
};
static UnterminatedString: Message = Message {
    camelcase_name: "UnterminatedString",
    fields: &[],
    comment: &["Emitted for code like", "```text", "\"foo", "```"],
};
static UnterminatedQuotedString: Message = Message {
    camelcase_name: "UnterminatedQuotedString",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "%s",
        "//    ^ EOF, not \"",
        "",
        "```",
    ],
};
static InvalidUnicodeEscape: Message = Message {
    camelcase_name: "InvalidUnicodeEscape",
    fields: &[],
    comment: &["Emitted for code like", "```text", "\"\\ufoo\"", "```"],
};
static TooLargeUnicodeCodepoint: Message = Message {
    camelcase_name: "TooLargeUnicodeCodepoint",
    fields: &[],
    comment: &["Emitted for code like", "```text", "\"\\u{999999}\"", "```"],
};
static InvalidUnicodeCodepoint: Message = Message {
    camelcase_name: "InvalidUnicodeCodepoint",
    fields: &[],
    comment: &["Emitted for code like", "```text", "\"\\u{d800}\"", "```"],
};
static MultipleCodepointAtSingleChar: Message = Message {
    camelcase_name: "MultipleCodepointAtSingleChar",
    fields: &[],
    comment: &["Emitted for code like", "```text", "?\\u{41 42}", "```"],
};
static InvalidEscapeCharacter: Message = Message {
    camelcase_name: "InvalidEscapeCharacter",
    fields: &[],
    comment: &["Emitted for code like", "```text", "\"\\M-\u{1}\"", "```"],
};
static InvalidHexEscape: Message = Message {
    camelcase_name: "InvalidHexEscape",
    fields: &[],
    comment: &["Emitted for code like", "```text", "\"\\xZZ\"", "```"],
};
static UnterminatedHeredoc: Message = Message {
    camelcase_name: "UnterminatedHeredoc",
    fields: &[&MessageField {
        message: &UnterminatedHeredoc,
        snakecase_name: "heredoc_id",
        field_type: MessageFieldType::Str,
        comment: &["Heredoc identifier"],
    }],
    comment: &["Emitted for code like", "```text", "<<-HERE", "```"],
};
static UnterminatedHeredocId: Message = Message {
    camelcase_name: "UnterminatedHeredocId",
    fields: &[],
    comment: &["Emitted for code like", "```text", "<<-\"HERE", "```"],
};
static SlashRAtMiddleOfLine: Message = Message {
    camelcase_name: "SlashRAtMiddleOfLine",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "eval(\"foo \\r = 42\")",
        "```",
    ],
};
static DStarInterpretedAsArgPrefix: Message = Message {
    camelcase_name: "DStarInterpretedAsArgPrefix",
    fields: &[],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "foo **arg",
        "```",
    ],
};
static StarInterpretedAsArgPrefix: Message = Message {
    camelcase_name: "StarInterpretedAsArgPrefix",
    fields: &[],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "foo *arg",
        "```",
    ],
};
static AmpersandInterpretedAsArgPrefix: Message = Message {
    camelcase_name: "AmpersandInterpretedAsArgPrefix",
    fields: &[],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "foo &arg",
        "```",
    ],
};
static TripleDotAtEol: Message = Message {
    camelcase_name: "TripleDotAtEol",
    fields: &[],
    comment: &["Emitted for code like", "```text", "range = 1...", "```"],
};
static ParenthesesIterpretedAsArglist: Message = Message {
    camelcase_name: "ParenthesesIterpretedAsArglist",
    fields: &[],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "def m (a, b, c); end",
        "```",
    ],
};
static AmbiguousFirstArgument: Message = Message {
    camelcase_name: "AmbiguousFirstArgument",
    fields: &[&MessageField {
        message: &AmbiguousFirstArgument,
        snakecase_name: "operator",
        field_type: MessageFieldType::Byte,
        comment: &["Operator that is ambiguous"],
    }],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "m +foo",
        "```",
    ],
};
static AmbiguousOperator: Message = Message {
    camelcase_name: "AmbiguousOperator",
    fields: &[
        &MessageField {
            message: &AmbiguousOperator,
            snakecase_name: "operator",
            field_type: MessageFieldType::Str,
            comment: &["Operator that is ambiguous"],
        },
        &MessageField {
            message: &AmbiguousOperator,
            snakecase_name: "interpreted_as",
            field_type: MessageFieldType::Str,
            comment: &["Interpretation of this operator"],
        },
    ],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "1 *2",
        "```",
    ],
};
static InvalidCharacterSyntax: Message = Message {
    camelcase_name: "InvalidCharacterSyntax",
    fields: &[&MessageField {
        message: &InvalidCharacterSyntax,
        snakecase_name: "suggestion",
        field_type: MessageFieldType::Str,
        comment: &["Valid syntax sugestions"],
    }],
    comment: &["Emitted for code like", "```text", "\"\\M- \"", "```"],
};
static InvalidOctalDigit: Message = Message {
    camelcase_name: "InvalidOctalDigit",
    fields: &[],
    comment: &["Emitted for code like", "```text", "09", "```"],
};
static TrailingCharInNumber: Message = Message {
    camelcase_name: "TrailingCharInNumber",
    fields: &[&MessageField {
        message: &TrailingCharInNumber,
        snakecase_name: "c",
        field_type: MessageFieldType::Byte,
        comment: &["Invalid trailing char"],
    }],
    comment: &["Emitted for code like", "```text", "0_a", "```"],
};
static EmbeddedDocumentMeetsEof: Message = Message {
    camelcase_name: "EmbeddedDocumentMeetsEof",
    fields: &[],
    comment: &["Emitted for code like", "```text", "=begin", "```"],
};
static InvalidChar: Message = Message {
    camelcase_name: "InvalidChar",
    fields: &[&MessageField {
        message: &InvalidChar,
        snakecase_name: "c",
        field_type: MessageFieldType::Byte,
        comment: &["char"],
    }],
    comment: &[
        "Emitted for code like",
        "```text",
        "eval(\"\\x01foo\")",
        "```",
    ],
};
static IncompleteCharacterSyntax: Message = Message {
    camelcase_name: "IncompleteCharacterSyntax",
    fields: &[],
    comment: &[
        "It is unknown how to trigger this error.",
        "Code that triggers it in MRI can be dead.",
    ],
};
static GvarWithoutId: Message = Message {
    camelcase_name: "GvarWithoutId",
    fields: &[],
    comment: &["Emitted for code like", "```text", "$", "```"],
};
static InvalidGvarName: Message = Message {
    camelcase_name: "InvalidGvarName",
    fields: &[&MessageField {
        message: &InvalidGvarName,
        snakecase_name: "c",
        field_type: MessageFieldType::Byte,
        comment: &["char after `$`"],
    }],
    comment: &["Emitted for code like", "```text", "$@", "```"],
};
static IvarWithoutId: Message = Message {
    camelcase_name: "IvarWithoutId",
    fields: &[],
    comment: &["Emitted for code like", "```text", "@", "```"],
};
static InvalidIvarName: Message = Message {
    camelcase_name: "InvalidIvarName",
    fields: &[&MessageField {
        message: &InvalidIvarName,
        snakecase_name: "c",
        field_type: MessageFieldType::Byte,
        comment: &["char after `@`"],
    }],
    comment: &["Emitted for code like", "```text", "@1", "```"],
};
static CvarWithoutId: Message = Message {
    camelcase_name: "CvarWithoutId",
    fields: &[],
    comment: &["Emitted for code like", "```text", "@@", "```"],
};
static InvalidCvarName: Message = Message {
    camelcase_name: "InvalidCvarName",
    fields: &[&MessageField {
        message: &InvalidCvarName,
        snakecase_name: "c",
        field_type: MessageFieldType::Byte,
        comment: &["char after `@@`"],
    }],
    comment: &["Emitted for code like", "```text", "@@1", "```"],
};
static UnknownRegexOptions: Message = Message {
    camelcase_name: "UnknownRegexOptions",
    fields: &[&MessageField {
        message: &UnknownRegexOptions,
        snakecase_name: "options",
        field_type: MessageFieldType::Str,
        comment: &["Concatenated unknown options"],
    }],
    comment: &["Emitted for code like", "```text", "/re/foo", "```"],
};
static UnterminatedUnicodeEscape: Message = Message {
    camelcase_name: "UnterminatedUnicodeEscape",
    fields: &[],
    comment: &["Emitted for code like", "```text", "\"\\u{1234\"", "```"],
};
static EncodingError: Message = Message {
    camelcase_name: "EncodingError",
    fields: &[&MessageField {
        message: &EncodingError,
        snakecase_name: "error",
        field_type: MessageFieldType::Str,
        comment: &["Error from decoder"],
    }],
    comment: &["Emitted for code like", "```text", "# encoding: foo", "```"],
};
static InvalidMultibyteChar: Message = Message {
    camelcase_name: "InvalidMultibyteChar",
    fields: &[],
    comment: &[
        "Emitter for code like",
        "```text",
        "eval(\"\\xFF = 42\")",
        "```",
    ],
};

//
// Lexer warnings
//
static AmbiguousTernaryOperator: Message = Message {
    camelcase_name: "AmbiguousTernaryOperator",
    fields: &[&MessageField {
        message: &AmbiguousTernaryOperator,
        snakecase_name: "condition",
        field_type: MessageFieldType::Str,
        comment: &["Source of the condition expression"],
    }],
    comment: &["Emitted for code like", "```text", "a ?AA : 2", "```"],
};
static AmbiguousRegexp: Message = Message {
    camelcase_name: "AmbiguousRegexp",
    fields: &[],
    comment: &["Emitted for code like", "```text", "m /foo/", "```"],
};
//
// Parser errors
//
static ElseWithoutRescue: Message = Message {
    camelcase_name: "ElseWithoutRescue",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "begin; else; end",
        "```",
    ],
};
static BeginNotAtTopLevel: Message = Message {
    camelcase_name: "BeginNotAtTopLevel",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def f; BEGIN{}; end",
        "```",
    ],
};
static AliasNthRef: Message = Message {
    camelcase_name: "AliasNthRef",
    fields: &[],
    comment: &["Emitted for code like", "```text", "alias $a $1", "```"],
};
static CsendInsideMasgn: Message = Message {
    camelcase_name: "CsendInsideMasgn",
    fields: &[],
    comment: &["Emitted for code like", "```text", "*a&.x = 0", "```"],
};
static ClassOrModuleNameMustBeConstant: Message = Message {
    camelcase_name: "ClassOrModuleNameMustBeConstant",
    fields: &[],
    comment: &["Emitted for code like", "```text", "module foo; end", "```"],
};
static EndlessSetterDefinition: Message = Message {
    camelcase_name: "EndlessSetterDefinition",
    fields: &[],
    comment: &["Emitted for code like", "```text", "def foo=() = 42", "```"],
};
static UnexpectedToken: Message = Message {
    camelcase_name: "UnexpectedToken",
    fields: &[&MessageField {
        message: &UnexpectedToken,
        snakecase_name: "token_name",
        field_type: MessageFieldType::Str,
        comment: &["Name of the token"],
    }],
    comment: &["Emitted for any code that produces invalid sequence of tokens"],
};
static ClassDefinitionInMethodBody: Message = Message {
    camelcase_name: "ClassDefinitionInMethodBody",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def a; class Foo; end; end",
        "```",
    ],
};
static ModuleDefinitionInMethodBody: Message = Message {
    camelcase_name: "ModuleDefinitionInMethodBody",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def a; module Foo; end; end",
        "```",
    ],
};
static InvalidReturnInClassOrModuleBody: Message = Message {
    camelcase_name: "InvalidReturnInClassOrModuleBody",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "class A; return; end",
        "```",
    ],
};
static ConstArgument: Message = Message {
    camelcase_name: "ConstArgument",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def foo(Abc); end",
        "```",
    ],
};
static IvarArgument: Message = Message {
    camelcase_name: "IvarArgument",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def foo(@abc); end",
        "```",
    ],
};
static GvarArgument: Message = Message {
    camelcase_name: "GvarArgument",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def foo($abc); end",
        "```",
    ],
};
static CvarArgument: Message = Message {
    camelcase_name: "CvarArgument",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def foo(@@abc); end",
        "```",
    ],
};
static NoSuchLocalVariable: Message = Message {
    camelcase_name: "NoSuchLocalVariable",
    fields: &[&MessageField {
        message: &NoSuchLocalVariable,
        snakecase_name: "var_name",
        field_type: MessageFieldType::Str,
        comment: &["Variable name"],
    }],
    comment: &[
        "Emitted for code like",
        "```text",
        "case 0; in ^a; true; end",
        "```",
    ],
};
static OrdinaryParamDefined: Message = Message {
    camelcase_name: "OrdinaryParamDefined",
    fields: &[],
    comment: &["Emitted for code like", "```text", "m { |a| _1 }", "```"],
};
static NumparamUsed: Message = Message {
    camelcase_name: "NumparamUsed",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "foo { _1; bar { _2 }; }",
        "```",
    ],
};
static TokAtEolWithoutExpression: Message = Message {
    camelcase_name: "TokAtEolWithoutExpression",
    fields: &[&MessageField {
        message: &TokAtEolWithoutExpression,
        snakecase_name: "token_name",
        field_type: MessageFieldType::Str,
        comment: &["Name of the token"],
    }],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "if",
        "42",
        "end",
        "```",
    ],
};
static InvalidIdToGet: Message = Message {
    camelcase_name: "InvalidIdToGet",
    fields: &[&MessageField {
        message: &InvalidIdToGet,
        snakecase_name: "identifier",
        field_type: MessageFieldType::Str,
        comment: &["Identifier"],
    }],
    comment: &[
        "Emitted for code like",
        "```text",
        "{ foo?: }",
        "# or",
        "{ foo!: }",
        "```",
    ],
};

//
// Parser warnings
//
static EndInMethod: Message = Message {
    camelcase_name: "EndInMethod",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def m; END {}; end",
        "```",
    ],
};
static ComparisonAfterComparison: Message = Message {
    camelcase_name: "ComparisonAfterComparison",
    fields: &[&MessageField {
        message: &ComparisonAfterComparison,
        snakecase_name: "comparison",
        field_type: MessageFieldType::Str,
        comment: &["Source of the first comparison"],
    }],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "a < b < c",
        "```",
    ],
};
static DuplicateHashKey: Message = Message {
    camelcase_name: "DuplicateHashKey",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "{ 42 => value, 42 => another_value }",
        "```",
    ],
};

//
// Builder errors
//
static CircularArgumentReference: Message = Message {
    camelcase_name: "CircularArgumentReference",
    fields: &[&MessageField {
        message: &CircularArgumentReference,
        snakecase_name: "arg_name",
        field_type: MessageFieldType::Str,
        comment: &["Name of the argument"],
    }],
    comment: &[
        "Emitted for code like",
        "```text",
        "def m(foo = foo) end",
        "```",
    ],
};
static DynamicConstantAssignment: Message = Message {
    camelcase_name: "DynamicConstantAssignment",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def m; FOO = 1; end",
        "```",
    ],
};
static CantAssignToSelf: Message = Message {
    camelcase_name: "CantAssignToSelf",
    fields: &[],
    comment: &["Emitted for code like", "```text", "self = foo", "```"],
};
static CantAssignToNil: Message = Message {
    camelcase_name: "CantAssignToNil",
    fields: &[],
    comment: &["Emitted for code like", "```text", "nil = foo", "```"],
};
static CantAssignToTrue: Message = Message {
    camelcase_name: "CantAssignToTrue",
    fields: &[],
    comment: &["Emitted for code like", "```text", "true = foo", "```"],
};
static CantAssignToFalse: Message = Message {
    camelcase_name: "CantAssignToFalse",
    fields: &[],
    comment: &["Emitted for code like", "```text", "false = foo", "```"],
};
static CantAssignToFile: Message = Message {
    camelcase_name: "CantAssignToFile",
    fields: &[],
    comment: &["Emitted for code like", "```text", "__FILE__ = foo", "```"],
};
static CantAssignToLine: Message = Message {
    camelcase_name: "CantAssignToLine",
    fields: &[],
    comment: &["Emitted for code like", "```text", "__LINE__ = foo", "```"],
};
static CantAssignToEncoding: Message = Message {
    camelcase_name: "CantAssignToEncoding",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "__ENCODING__ = foo",
        "```",
    ],
};
static CantAssignToNumparam: Message = Message {
    camelcase_name: "CantAssignToNumparam",
    fields: &[&MessageField {
        message: &CantAssignToNumparam,
        snakecase_name: "numparam",
        field_type: MessageFieldType::Str,
        comment: &["Source of the numbered parameter"],
    }],
    comment: &[
        "Emitted for code like",
        "```text",
        "proc {_1; _1 = nil}",
        "```",
    ],
};
static CantSetVariable: Message = Message {
    camelcase_name: "CantSetVariable",
    fields: &[&MessageField {
        message: &CantSetVariable,
        snakecase_name: "var_name",
        field_type: MessageFieldType::Str,
        comment: &["Source of the read-only variable that is assigned"],
    }],
    comment: &["Emitted for code like", "```text", "$1 = foo", "```"],
};
static BlockGivenToYield: Message = Message {
    camelcase_name: "BlockGivenToYield",
    fields: &[],
    comment: &["Emitted for code like", "```text", "yield(&foo)", "```"],
};
static BlockAndBlockArgGiven: Message = Message {
    camelcase_name: "BlockAndBlockArgGiven",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "fun(&bar) do end",
        "```",
    ],
};
static SymbolLiteralWithInterpolation: Message = Message {
    camelcase_name: "SymbolLiteralWithInterpolation",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "case a; in \"#{a}\": 1; end",
        "```",
    ],
};
static ReservedForNumparam: Message = Message {
    camelcase_name: "ReservedForNumparam",
    fields: &[&MessageField {
        message: &ReservedForNumparam,
        snakecase_name: "numparam",
        field_type: MessageFieldType::Str,
        comment: &["Numbered parameter that is treated as a local variable"],
    }],
    comment: &["Emitted for code like", "```text", "_1 = 1", "```"],
};
static KeyMustBeValidAsLocalVariable: Message = Message {
    camelcase_name: "KeyMustBeValidAsLocalVariable",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "case a; in a?:; end",
        "```",
    ],
};
static DuplicateVariableName: Message = Message {
    camelcase_name: "DuplicateVariableName",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "case 0; in a, a; end",
        "```",
    ],
};
static DuplicateKeyName: Message = Message {
    camelcase_name: "DuplicateKeyName",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "case 0; in a: 1, a: 2; end",
        "```",
    ],
};
static SingletonLiteral: Message = Message {
    camelcase_name: "SingletonLiteral",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def (1).foo; end",
        "```",
    ],
};
static NthRefIsTooBig: Message = Message {
    camelcase_name: "NthRefIsTooBig",
    fields: &[&MessageField {
        message: &NthRefIsTooBig,
        snakecase_name: "nth_ref",
        field_type: MessageFieldType::Str,
        comment: &["Source of the nth_ref that is techincally a regular global variable"],
    }],
    comment: &[
        "Emitted for code like (only in $VERBOSE mode)",
        "```text",
        "$100",
        "```",
    ],
};
static DuplicatedArgumentName: Message = Message {
    camelcase_name: "DuplicatedArgumentName",
    fields: &[],
    comment: &[
        "Emitted for code like",
        "```text",
        "def foo(aa, aa); end",
        "```",
    ],
};
static RegexError: Message = Message {
    camelcase_name: "RegexError",
    fields: &[&MessageField {
        message: &RegexError,
        snakecase_name: "error",
        field_type: MessageFieldType::Str,
        comment: &["Error from Onigurama engine"],
    }],
    comment: &["Emitted for code like", "```text", "/[/", "```"],
};
static InvalidSymbol: Message = Message {
    camelcase_name: "InvalidSymbol",
    fields: &[&MessageField {
        message: &InvalidSymbol,
        snakecase_name: "symbol",
        field_type: MessageFieldType::Str,
        comment: &["Source of the symbol"],
    }],
    comment: &["Emitted for code like", "```text", "%I\"x .\\xc3.\"", "```"],
};
static VoidValueExpression: Message = Message {
    camelcase_name: "VoidValueExpression",
    fields: &[],
    comment: &["Emitted for code like", "```text", "a = return", "```"],
};

pub static ALL_MESSAGES: MessagesList = &[
    &FractionAfterNumeric,
    &NoDigitsAfterDot,
    &UnknownTypeOfPercentString,
    &NumericLiteralWithoutDigits,
    &UnterminatedList,
    &UnterminatedRegexp,
    &UnterminatedString,
    &UnterminatedQuotedString,
    &InvalidUnicodeEscape,
    &TooLargeUnicodeCodepoint,
    &InvalidUnicodeCodepoint,
    &MultipleCodepointAtSingleChar,
    &InvalidEscapeCharacter,
    &InvalidHexEscape,
    &UnterminatedHeredoc,
    &UnterminatedHeredocId,
    &SlashRAtMiddleOfLine,
    &DStarInterpretedAsArgPrefix,
    &StarInterpretedAsArgPrefix,
    &AmpersandInterpretedAsArgPrefix,
    &TripleDotAtEol,
    &ParenthesesIterpretedAsArglist,
    &AmbiguousFirstArgument,
    &AmbiguousOperator,
    &InvalidCharacterSyntax,
    &InvalidOctalDigit,
    &TrailingCharInNumber,
    &EmbeddedDocumentMeetsEof,
    &InvalidChar,
    &IncompleteCharacterSyntax,
    &GvarWithoutId,
    &InvalidGvarName,
    &IvarWithoutId,
    &InvalidIvarName,
    &CvarWithoutId,
    &InvalidCvarName,
    &UnknownRegexOptions,
    &UnterminatedUnicodeEscape,
    &EncodingError,
    &InvalidMultibyteChar,
    &AmbiguousTernaryOperator,
    &AmbiguousRegexp,
    &ElseWithoutRescue,
    &BeginNotAtTopLevel,
    &AliasNthRef,
    &CsendInsideMasgn,
    &ClassOrModuleNameMustBeConstant,
    &EndlessSetterDefinition,
    &UnexpectedToken,
    &ClassDefinitionInMethodBody,
    &ModuleDefinitionInMethodBody,
    &InvalidReturnInClassOrModuleBody,
    &ConstArgument,
    &IvarArgument,
    &GvarArgument,
    &CvarArgument,
    &NoSuchLocalVariable,
    &OrdinaryParamDefined,
    &NumparamUsed,
    &TokAtEolWithoutExpression,
    &InvalidIdToGet,
    &EndInMethod,
    &ComparisonAfterComparison,
    &DuplicateHashKey,
    &CircularArgumentReference,
    &DynamicConstantAssignment,
    &CantAssignToSelf,
    &CantAssignToNil,
    &CantAssignToTrue,
    &CantAssignToFalse,
    &CantAssignToFile,
    &CantAssignToLine,
    &CantAssignToEncoding,
    &CantAssignToNumparam,
    &CantSetVariable,
    &BlockGivenToYield,
    &BlockAndBlockArgGiven,
    &SymbolLiteralWithInterpolation,
    &ReservedForNumparam,
    &KeyMustBeValidAsLocalVariable,
    &DuplicateVariableName,
    &DuplicateKeyName,
    &SingletonLiteral,
    &NthRefIsTooBig,
    &DuplicatedArgumentName,
    &RegexError,
    &InvalidSymbol,
    &VoidValueExpression,
];
