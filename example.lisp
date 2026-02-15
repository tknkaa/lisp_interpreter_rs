; Sample Lisp program for the interpreter
; Lines starting with semicolon are comments

; Basic arithmetic
(+ 1 2)
(- 10 3)
(* 4 5)
(/ 20 4)

; Nested expressions
(+ (* 2 3) 4)
(* (+ 1 2) (- 10 5))

; Define variables
(define x 10)
(define y 5)

; Use variables
(+ x y)
(- x y)
(* x y)
(/ x y)

; Define with expressions
(define result (+ (* x 2) y))

; Use the computed variable
(* result 3)

; Complex nested expression
(+ (- 100 10) (* 5 (+ 2 3)))

; Multiple operations
(define a 7)
(define b 3)
(define sum (+ a b))
(define product (* a b))
(+ sum product)
