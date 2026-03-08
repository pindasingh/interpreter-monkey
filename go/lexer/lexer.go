package lexer

import (
	"fmt"
	"interpreter-monkey/token"
)

type Lexer struct {
	input        string
	position     int
	readPosition int
	ch           byte
}

func New(input string) Lexer {
	l := Lexer{input: input}
	l.readChar()
	return l
}

func (l Lexer) readChar() {
	if l.readPosition >= len(l.input) {
		l.ch = 0
	} else {
		l.ch = l.input[l.readPosition]
	}
	fmt.Println(l.ch, l.readPosition, l.position)
	l.readPosition++
}

func (l Lexer) nextToken() token.Token {
	var tok token.Token

	switch l.ch {
	case '=':
		tok = newToken(token.ASSIGN, l.ch)
	}
	l.readChar()
	return tok
}

func newToken(tokenType token.Type, ch byte) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}
