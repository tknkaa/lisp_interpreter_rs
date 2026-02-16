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

; User-defined functions (lambda)
; Simple function: double a number
(define double (lambda (x) (* x 2)))
(double 5)
(double 10)

; Multi-parameter function
(define add (lambda (x y) (+ x y)))
(add 3 7)
(add 100 200)

; Function with complex body
(define square (lambda (x) (* x x)))
(square 4)
(square 9)

; Function using other functions
(define sum-of-squares (lambda (a b) (+ (square a) (square b))))
(sum-of-squares 3 4)

; Recursive function: factorial
(define factorial (lambda (n) (if (<= n 1) 1 (* n (factorial (- n 1))))))
(factorial 5)
(factorial 6)

; Recursive function: fibonacci
(define fib (lambda (n) (if (<= n 2) 1 (+ (fib (- n 1)) (fib (- n 2))))))
(fib 1)
(fib 5)
(fib 8)

; Closure example: function that creates functions
(define make-adder (lambda (x) (lambda (y) (+ x y))))
(define add5 (make-adder 5))
(define add10 (make-adder 10))
(add5 3)
(add10 7)

; Function returning functions based on condition
(define make-multiplier (lambda (factor) (lambda (x) (* x factor))))
(define times3 (make-multiplier 3))
(times3 10)
