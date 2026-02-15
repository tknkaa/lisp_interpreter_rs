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

; Comparison operators
(< 3 5)
(> 10 2)
(<= 5 5)
(>= 8 8)
(= 4 4)
(= 3 7)

; Comparisons with expressions
(< (+ 1 2) 5)
(> (* 2 3) 5)
(= (+ 2 3) (* 1 5))

; If expressions
(if (< 3 5) 100 200)
(if (> 2 8) 100 200)

; If with variables
(define age 20)
(if (>= age 18) 1 0)

; Nested if
(define score 85)
(if (>= score 90) 10 (if (>= score 80) 8 5))

; If with complex conditions and expressions
(define m 15)
(define n 10)
(if (> m n) (+ m n) (- m n))
