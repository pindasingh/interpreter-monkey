package lexer

import (
	"interpreter-monkey/token"
	"testing"
)

func TestNextToken(t *testing.T) {
	input := `let five = 5;
let ten = 10;

let add = fn(x, y) {
	x + y;
};

let result = add(five, ten);
`

	tests := []struct {
		expectedType    token.Type
		expectedLiteral string
	}{
		{token.LET, "let"},
	}

	l := New(input)

	for _, test := range tests {
		tok := l.nextToken()

		if tok.Type != test.expectedType {
			t.Fatal()
		}
	}
}
