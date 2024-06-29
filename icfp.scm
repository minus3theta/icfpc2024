

(define order-icfp (string->list "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"))
(define order-human (string->list "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"))

(define icfp->human (map cons order-icfp order-human))
(define human->icfp (map cons order-human order-icfp))
;(print human->icfp)

(define (replace-chars str table)
  (let* ([chars (string->list str)]
         [converted (map (lambda (ch) (or (cdr (assq ch table)) ch)) chars)])
    (list->string converted) ))

(define (from-human s) (replace-chars s human->icfp))
(define (to-human s) (replace-chars s icfp->human))

;(print (from-human "test"))

(define (unary-operator ch)
    (cond
        ((eq? ch #\-) '-) ; '(lambda (n) (- n)))
        ((eq? ch #\!) 'not) ; '(lambda (b) (not b)))
        ((eq? ch #\#) 'icfp-string->number) ; '(lambda (s) (string->number s)))
        ((eq? ch #\$) 'number->icfp-string) ; '(lambda (n) (number->string n)))
        (#t (lambda (x) 'nil)) ))



(define (apply* a b) ((force a) (force b)))

(define (binary-operator ch)
    (let1 proc (cond
        ((eq? ch #\+) '+); '(lambda (a b) (+ a b)))
        ((eq? ch #\-) '-); '(lambda (a b) (- a b)))
        ((eq? ch #\*) '*); '(lambda (a b) (* a b)))
        ((eq? ch #\/) 'quotient); '(lambda (a b) (quotient a b)))
        ((eq? ch #\%) 'remainder); '(lambda (a b) (remainder a b)))
        ((eq? ch #\<) '<); '(lambda (a b) (< a b)))
        ((eq? ch #\>) '>); '(lambda (a b) (> a b)))
        ((eq? ch #\=) '=); '(lambda (a b) (= a b)))
        ((eq? ch #\|) '(lambda (a b) (or a b)))
        ((eq? ch #\&) '(lambda (a b) (and a b)))
        ((eq? ch #\.) 'string-append); '(lambda (a b) (string-append a b)))
        ((eq? ch #\T) '(lambda (a b) (substring b 0 a)))
        ((eq? ch #\D) '(lambda (a b) (substring b a (string-length b))))
        ;((eq? ch #\$) 'apply);'(lambda (a b) (apply a b)))
        ;((eq? ch #\$) '(lambda (a b) (a b)))
        ((eq? ch #\$) 'apply*)
        (else '(lambda (a b) nil)) )
;      (print "(binary-operator " ch ") -> " proc)
      proc))


(define (icfp-string->number s)
    (let loop ((x 0) (r (string->list s)))
        (if (null? r) x
            (loop (+ (* x 94) (- (char->integer (car r)) 33)) (cdr r)))))


(define (number->icfp-string n)
    (let loop ((s ()) (x n))
        (if (= x 0) (list->string s)
            (receive (q r) (quotient&remainder x 94)
                (loop (cons (integer->char (+ 33 r)) s) q) ))))
        ;     (loop (cons (integer->char (+ 33 (remainder x 94))) s) (quotient x 94)))))
        ; (if (null? r) x
        ;     (loop (+ (* x 94) (- (char->integer (car r)) 33)) (cdr r)))))


(define (read-token token)
    (let* ([indicator (string-ref token 0)]
           [body (substring token 1 (string-length token))])
        ;    (print "read-token " token " -> " indicator " " body)
        (cond
            ((eq? indicator #\T) (list 'literal #t))
            ((eq? indicator #\F) (list 'literal #f))
            ((eq? indicator #\I) (list 'literal (icfp-string->number body)))
            ((eq? indicator #\S) (list 'literal body))
;            ((eq? indicator #\S) (list 'literal (to-human body)))
            ((eq? indicator #\U) (list 'unary (unary-operator (string-ref body 0))))
            ((eq? indicator #\B)
                (if (eq? (string-ref token 1) #\$)
                    (list 'apply)
                    (list 'binary (binary-operator (string-ref body 0)))))
            ((eq? indicator #\?) (list 'if))
            ((eq? indicator #\L) (list 'lambda (icfp-string->number body)))
            ((eq? indicator #\v) (list 'symbol (icfp-string->number body)))
            (else 'nil)
        ) ))


(define (parse s)
    (map read-token (string-split s " ")))

(define (make-symbol num)
;    (string->symbol (string-append "sym" (number->string num))))
    (string->symbol (string-append "v" (number->string num))))

(define (forced-op2 op)
    (lambda (x y) (op (force x) (force y))))

(define (forced-op1 op)
    (lambda (x) (op (force x))))


(define (make-ast elems)
    (define (take-one seq)  ;; -> (obj rest)
        (let* ([elem (car seq)]
               [rest (cdr seq)]
               [type (car elem)])
            (cond
                [(eq? type 'literal)
                    (values (cadr elem) rest)]
                [(eq? type 'unary)
                    (let1 op (cadr elem)
                        (receive (arg rest) (take-one rest)
                            (values (list (list 'forced-op1 op) arg) rest) ))]
                [(eq? type 'binary)
                    (let1 op (cadr elem)
                        (receive (arg1 rest) (take-one rest)
                          (receive (arg2 rest) (take-one rest)
                              (values (list (list 'forced-op2 op) arg1 arg2) rest) )))]
                [(eq? type 'apply)
                    (receive (arg1 rest) (take-one rest)
                      (receive (arg2 rest) (take-one rest)
                          (values (list apply* arg1 arg2) rest)))]
                [(eq? type 'if)
                    (receive (cl-cond rest) (take-one rest)
                      (receive (cl-then rest) (take-one rest)
                        (receive (cl-else rest) (take-one rest)
                           (values (list 'if cl-cond cl-then cl-else) rest) )))]
                [(eq? type 'lambda)
                    (let1 sym (make-symbol (cadr elem))
                        ; (print ":lambda: " sym)
                        (receive (body rest) (take-one rest)
                            ; (print ":1: " body)
                            ; (print ":2: " (list 'lambda (list sym) body)  )
                            (values (list 'lambda (list sym) body) rest) ))]
                [(eq? type 'symbol)
                    (let1 sym (make-symbol (cadr elem))
                            (values (list 'delay sym) rest))]
                [else (values 'nil rest)]
            );cond
        ));define
    (receive (obj rest) (take-one elems)
        obj))


(define (read-icfp s)
    (print s)
    (let1 tokens (parse s)
        (print "  -> " tokens)
        (let1 ast (make-ast tokens)
            (print "  --> " ast)
;            ast)))
        ; (let1 ast-wrapped
        ;     (list 'let1 'v23 '(lambda (x) x)
        ;     ast)
        ;     (print "  --> " ast-wrapped)
           (let1 value  (force (eval ast ()))
               (print "  ---> " value)
               value))))

(use gauche.test)
(test-start "parse")

;(test-section "0")
;(my-test "? B> I# I$ S9%3 S./")
(define-macro (parse-test input expected)
  `(test* "parse" ,expected (parse ,input)))

(define-macro (my-test input expected)
  `(let1 ast (read-icfp ,input)
    (test* "parse" ,expected ast)))


(my-test "S'%4}).$%8" (from-human "get index"))

(test-section "Booleans")
(my-test "T" #t)
(my-test "F" #f)

(test-section "Integers")
(my-test "I/6" 1337)

(test-section "Strings")
(my-test "SB%,,/}Q/2,$_" (from-human "Hello World!"))

(test-section "Unary operators")
(my-test "U- I$" -3)
(my-test "U! T" #f)
(my-test "U# S4%34" 15818151)
(my-test "U$ I4%34" (from-human "test"))

(test-section "Binary operators")
(my-test "B+ I# I$" 5)
(my-test "B- I$ I#" 1)
(my-test "B* I$ I#" 6)
(my-test "B/ U- I( I#" -3)
(my-test "B% U- I( I#" -1)
(my-test "B< I$ I#" #f)
(my-test "B> I$ I#" #t)
(my-test "B= I$ I#" #f)
(my-test "B| T F" #t)
(my-test "B& T F" #f)
(my-test "B. S4% S34" (from-human "test"))
(my-test "BT I$ S4%34" (from-human "tes"))
(my-test "BD I$ S4%34" (from-human "t"))

(test-section "If")
(my-test "? B> I# I$ S9%3 S./" (from-human "no"))

(test-section "Lambda")
(my-test "B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK" (from-human "Hello World!"))
;(my-test "B$ L# B$ L\" B+ v\" v\" B* I$ I# v8" 12)
(my-test "B$ L\" B+ v\" v\" B* I$ I#" 12)
(my-test "B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L\" L# ? B= v# I! I\" B$ L$ B+ B$ v\" v$ B$ v\" v$ B- v# I\" I%" 16)

(test-end :exit-on-failure #f)
