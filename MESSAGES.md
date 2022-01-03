## FractionAfterNumeric

    Emitted for code
    ```text
    1.2.3
    ```

Fields:



## NoDigitsAfterDot

    Emitted for code like
    ```text
    foo.2
    ```

Fields:



## UnknownTypeOfPercentString

    Emitted for code like
    ```text
    %k[foo]
    ```

Fields:



## NumericLiteralWithoutDigits

    Emitted for code like
    ```text
    0b
    ```

Fields:



## UnterminatedList

    Emitted for code like
    ```text
    %w[foo bar
    ```

Fields:



## UnterminatedRegexp

    Emitted for code like
    ```text
    /foo
    ```

Fields:



## UnterminatedString

    Emitted for code like
    ```text
    "foo
    ```

Fields:



## UnterminatedQuotedString

    Emitted for code like
    ```text
    %s
    //    ^ EOF, not "
   
    ```

Fields:



## InvalidUnicodeEscape

    Emitted for code like
    ```text
    "\ufoo"
    ```

Fields:



## TooLargeUnicodeCodepoint

    Emitted for code like
    ```text
    "\u{999999}"
    ```

Fields:



## InvalidUnicodeCodepoint

    Emitted for code like
    ```text
    "\u{d800}"
    ```

Fields:



## MultipleCodepointAtSingleChar

    Emitted for code like
    ```text
    ?\u{41 42}
    ```

Fields:



## InvalidEscapeCharacter

    Emitted for code like
    ```text
    "\M-"
    ```

Fields:



## InvalidHexEscape

    Emitted for code like
    ```text
    "\xZZ"
    ```

Fields:



## UnterminatedHeredoc

    Emitted for code like
    ```text
    <<-HERE
    ```

Fields:

1. **heredoc_id** (`Str`)

    Heredoc identifier


## UnterminatedHeredocId

    Emitted for code like
    ```text
    <<-"HERE
    ```

Fields:



## SlashRAtMiddleOfLine

    Emitted for code like
    ```text
    eval("foo \r = 42")
    ```

Fields:



## DStarInterpretedAsArgPrefix

    Emitted for code like (only in $VERBOSE mode)
    ```text
    foo **arg
    ```

Fields:



## StarInterpretedAsArgPrefix

    Emitted for code like (only in $VERBOSE mode)
    ```text
    foo *arg
    ```

Fields:



## AmpersandInterpretedAsArgPrefix

    Emitted for code like (only in $VERBOSE mode)
    ```text
    foo &arg
    ```

Fields:



## TripleDotAtEol

    Emitted for code like
    ```text
    range = 1...
    ```

Fields:



## ParenthesesIterpretedAsArglist

    Emitted for code like (only in $VERBOSE mode)
    ```text
    def m (a, b, c); end
    ```

Fields:



## AmbiguousFirstArgument

    Emitted for code like (only in $VERBOSE mode)
    ```text
    m +foo
    ```

Fields:

1. **operator** (`Byte`)

    Operator that is ambiguous


## AmbiguousOperator

    Emitted for code like (only in $VERBOSE mode)
    ```text
    1 *2
    ```

Fields:

1. **operator** (`Str`)

    Operator that is ambiguous

2. **interpreted_as** (`Str`)

    Interpretation of this operator


## InvalidCharacterSyntax

    Emitted for code like
    ```text
    "\M- "
    ```

Fields:

1. **suggestion** (`Str`)

    Valid syntax sugestions


## InvalidOctalDigit

    Emitted for code like
    ```text
    09
    ```

Fields:



## TrailingCharInNumber

    Emitted for code like
    ```text
    0_a
    ```

Fields:

1. **c** (`Byte`)

    Invalid trailing char


## EmbeddedDocumentMeetsEof

    Emitted for code like
    ```text
    =begin
    ```

Fields:



## InvalidChar

    Emitted for code like
    ```text
    eval("\x01foo")
    ```

Fields:

1. **c** (`Byte`)

    char


## IncompleteCharacterSyntax

    It is unknown how to trigger this error.
    Code that triggers it in MRI can be dead.

Fields:



## GvarWithoutId

    Emitted for code like
    ```text
    $
    ```

Fields:



## InvalidGvarName

    Emitted for code like
    ```text
    $@
    ```

Fields:

1. **c** (`Byte`)

    char after `$`


## IvarWithoutId

    Emitted for code like
    ```text
    @
    ```

Fields:



## InvalidIvarName

    Emitted for code like
    ```text
    @1
    ```

Fields:

1. **c** (`Byte`)

    char after `@`


## CvarWithoutId

    Emitted for code like
    ```text
    @@
    ```

Fields:



## InvalidCvarName

    Emitted for code like
    ```text
    @@1
    ```

Fields:

1. **c** (`Byte`)

    char after `@@`


## UnknownRegexOptions

    Emitted for code like
    ```text
    /re/foo
    ```

Fields:

1. **options** (`Str`)

    Concatenated unknown options


## UnterminatedUnicodeEscape

    Emitted for code like
    ```text
    "\u{1234"
    ```

Fields:



## EncodingError

    Emitted for code like
    ```text
    # encoding: foo
    ```

Fields:

1. **error** (`Str`)

    Error from decoder


## InvalidMultibyteChar

    Emitter for code like
    ```text
    eval("\xFF = 42")
    ```

Fields:



## AmbiguousTernaryOperator

    Emitted for code like
    ```text
    a ?AA : 2
    ```

Fields:

1. **condition** (`Str`)

    Source of the condition expression


## AmbiguousRegexp

    Emitted for code like
    ```text
    m /foo/
    ```

Fields:



## ElseWithoutRescue

    Emitted for code like
    ```text
    begin; else; end
    ```

Fields:



## BeginNotAtTopLevel

    Emitted for code like
    ```text
    def f; BEGIN{}; end
    ```

Fields:



## AliasNthRef

    Emitted for code like
    ```text
    alias $a $1
    ```

Fields:



## CsendInsideMasgn

    Emitted for code like
    ```text
    *a&.x = 0
    ```

Fields:



## ClassOrModuleNameMustBeConstant

    Emitted for code like
    ```text
    module foo; end
    ```

Fields:



## EndlessSetterDefinition

    Emitted for code like
    ```text
    def foo=() = 42
    ```

Fields:



## UnexpectedToken

    Emitted for any code that produces invalid sequence of tokens

Fields:

1. **token_name** (`Str`)

    Name of the token


## ClassDefinitionInMethodBody

    Emitted for code like
    ```text
    def a; class Foo; end; end
    ```

Fields:



## ModuleDefinitionInMethodBody

    Emitted for code like
    ```text
    def a; module Foo; end; end
    ```

Fields:



## InvalidReturnInClassOrModuleBody

    Emitted for code like
    ```text
    class A; return; end
    ```

Fields:



## ConstArgument

    Emitted for code like
    ```text
    def foo(Abc); end
    ```

Fields:



## IvarArgument

    Emitted for code like
    ```text
    def foo(@abc); end
    ```

Fields:



## GvarArgument

    Emitted for code like
    ```text
    def foo($abc); end
    ```

Fields:



## CvarArgument

    Emitted for code like
    ```text
    def foo(@@abc); end
    ```

Fields:



## NoSuchLocalVariable

    Emitted for code like
    ```text
    case 0; in ^a; true; end
    ```

Fields:

1. **var_name** (`Str`)

    Variable name


## OrdinaryParamDefined

    Emitted for code like
    ```text
    m { |a| _1 }
    ```

Fields:



## NumparamUsed

    Emitted for code like
    ```text
    foo { _1; bar { _2 }; }
    ```

Fields:



## TokAtEolWithoutExpression

    Emitted for code like (only in $VERBOSE mode)
    ```text
    if
    42
    end
    ```

Fields:

1. **token_name** (`Str`)

    Name of the token


## InvalidIdToGet

    Emitted for code like
    ```text
    { foo?: }
    # or
    { foo!: }
    ```

Fields:

1. **identifier** (`Str`)

    Identifier


## ForwardArgAfterRestarg

    Emitted for code like
    ```text
    def foo *rest, ...
    end
    ```

Fields:



## NoAnonymousBlockarg

    Emitted for code like
    ```text
    def foo(); bar(&); end
    ```

Fields:



## EndInMethod

    Emitted for code like
    ```text
    def m; END {}; end
    ```

Fields:



## ComparisonAfterComparison

    Emitted for code like (only in $VERBOSE mode)
    ```text
    a < b < c
    ```

Fields:

1. **comparison** (`Str`)

    Source of the first comparison


## DuplicateHashKey

    Emitted for code like
    ```text
    { 42 => value, 42 => another_value }
    ```

Fields:



## CircularArgumentReference

    Emitted for code like
    ```text
    def m(foo = foo) end
    ```

Fields:

1. **arg_name** (`Str`)

    Name of the argument


## DynamicConstantAssignment

    Emitted for code like
    ```text
    def m; FOO = 1; end
    ```

Fields:



## CantAssignToSelf

    Emitted for code like
    ```text
    self = foo
    ```

Fields:



## CantAssignToNil

    Emitted for code like
    ```text
    nil = foo
    ```

Fields:



## CantAssignToTrue

    Emitted for code like
    ```text
    true = foo
    ```

Fields:



## CantAssignToFalse

    Emitted for code like
    ```text
    false = foo
    ```

Fields:



## CantAssignToFile

    Emitted for code like
    ```text
    __FILE__ = foo
    ```

Fields:



## CantAssignToLine

    Emitted for code like
    ```text
    __LINE__ = foo
    ```

Fields:



## CantAssignToEncoding

    Emitted for code like
    ```text
    __ENCODING__ = foo
    ```

Fields:



## CantAssignToNumparam

    Emitted for code like
    ```text
    proc {_1; _1 = nil}
    ```

Fields:

1. **numparam** (`Str`)

    Source of the numbered parameter


## CantSetVariable

    Emitted for code like
    ```text
    $1 = foo
    ```

Fields:

1. **var_name** (`Str`)

    Source of the read-only variable that is assigned


## BlockGivenToYield

    Emitted for code like
    ```text
    yield(&foo)
    ```

Fields:



## BlockAndBlockArgGiven

    Emitted for code like
    ```text
    fun(&bar) do end
    ```

Fields:



## SymbolLiteralWithInterpolation

    Emitted for code like
    ```text
    case a; in "#{a}": 1; end
    ```

Fields:



## ReservedForNumparam

    Emitted for code like
    ```text
    _1 = 1
    ```

Fields:

1. **numparam** (`Str`)

    Numbered parameter that is treated as a local variable


## KeyMustBeValidAsLocalVariable

    Emitted for code like
    ```text
    case a; in a?:; end
    ```

Fields:



## DuplicateVariableName

    Emitted for code like
    ```text
    case 0; in a, a; end
    ```

Fields:



## DuplicateKeyName

    Emitted for code like
    ```text
    case 0; in a: 1, a: 2; end
    ```

Fields:



## SingletonLiteral

    Emitted for code like
    ```text
    def (1).foo; end
    ```

Fields:



## NthRefIsTooBig

    Emitted for code like (only in $VERBOSE mode)
    ```text
    $100
    ```

Fields:

1. **nth_ref** (`Str`)

    Source of the nth_ref that is techincally a regular global variable


## DuplicatedArgumentName

    Emitted for code like
    ```text
    def foo(aa, aa); end
    ```

Fields:



## RegexError

    Emitted for code like
    ```text
    /[/
    ```

Fields:

1. **error** (`Str`)

    Error from Onigurama engine


## InvalidSymbol

    Emitted for code like
    ```text
    %I"x .\xc3."
    ```

Fields:

1. **symbol** (`Str`)

    Source of the symbol


## VoidValueExpression

    Emitted for code like
    ```text
    a = return
    ```

Fields:


