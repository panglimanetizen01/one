# One v0.1 AST Contract

The AST is language-independent and contains no Indonesian keyword tokens after parsing.

## Program

`Program { statements }`

## Statements

- `Declaration { name, value }`
- `Assignment { name, value }`
- `Action { name, args }`
- `Conditional { condition, then_block, else_block }`
- `Loop { kind, body }`
- `Function { name, params, result, body }`
- `Event { trigger, body }`

## Expressions

- Text
- Number
- Boolean
- Nothing
- Identifier
- Binary operation
- Function call

The AST is a parser contract. Semantic rules belong to the semantic-analysis layer and execution behavior belongs to the IR/runtime contract.
