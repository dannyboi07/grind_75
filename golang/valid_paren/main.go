package main

import (
	"fmt"
)

func main() {
	fmt.Println(isValid("(])"))
}

func isValid(s string) bool {
	var stack []string = make([]string, 0, len(s))

	for i := range s {
		var bracket string = string(s[i])

		if bracket == "(" || bracket == "{" || bracket == "[" {
			stack = append(stack, bracket)
		} else {
			var relatedBracket string = ""
			if bracket == ")" {
				relatedBracket = "("
			} else if bracket == "}" {
				relatedBracket = "{"
			} else if bracket == "]" {
				relatedBracket = "["
			}

			var stackLen = len(stack)
			if stackLen > 0 && stack[stackLen-1] == relatedBracket {
				stack = stack[:stackLen-1]
			} else {
				stack = append(stack, bracket)
			}
		}
	}

	return len(stack) == 0
}
