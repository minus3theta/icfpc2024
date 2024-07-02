

(define order-icfp (string->list "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"))
(define order-human (string->list "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"))

(define icfp->human (map cons order-icfp order-human))
(define human->icfp (map cons order-human order-icfp))

(define (replace-chars str table)
  (let* ([chars (string->list str)]
         [converted (map (lambda (ch) (or (cdr (assq ch table)) ch)) chars)])
    (list->string converted) ))

(define (from-human s) (replace-chars s human->icfp))
(define (to-human s) (replace-chars s icfp->human))

;(print (from-human "test"))

(define (unary-operator ch)
    (cond
        ((eq? ch #\-) '-)
        ((eq? ch #\!) 'not)
        ((eq? ch #\#) 'icfp-string->number)
        ((eq? ch #\$) 'number->icfp-string)
        (#t (lambda (x) 'nil)) ))

(define (binary-operator ch)
    (let1 proc (cond
        ((eq? ch #\+) '+)
        ((eq? ch #\-) '-)
        ((eq? ch #\*) '*)
        ((eq? ch #\/) 'quotient)
        ((eq? ch #\%) 'remainder)
        ((eq? ch #\<) '<)
        ((eq? ch #\>) '>)
        ((eq? ch #\=) '=)
        ((eq? ch #\|) '(lambda (a b) (or a b)))
        ((eq? ch #\&) '(lambda (a b) (and a b)))
        ((eq? ch #\.) 'string-append)
        ((eq? ch #\T) '(lambda (a b) (substring b 0 a)))
        ((eq? ch #\D) '(lambda (a b) (substring b a (string-length b))))
        ((eq? ch #\$) '(lambda (a b) (a b)))
        (else '(lambda (a b) nil)) )
;      (print "(binary-operator " ch ") -> " proc)
      proc))

(define (icfp-char->number ch)
    (- (char->integer ch) 33))

(define (icfp-string->number s)
    (let loop ((x 0) (r (string->list s)))
        (if (null? r) x
            (let1 cp (icfp-char->number (car r))
               (if (<= 0 cp 93)
                   (loop (+ (* x 94) (icfp-char->number (car r))) (cdr r))
                   x)))))

(define (number->icfp-string n)
    (let loop ((s ()) (x n))
        (if (= x 0) (list->string s)
            (receive (q r) (quotient&remainder x 94)
                (loop (cons (integer->char (+ 33 r)) s) q) ))))

(define (read-token token)
    (let* ([indicator (string-ref token 0)]
           [body (substring token 1 (string-length token))])
        (cond
            ((eq? indicator #\T) (list 'literal #t))
            ((eq? indicator #\F) (list 'literal #f))
            ((eq? indicator #\I) (list 'literal (icfp-string->number body)))
            ((eq? indicator #\S) (list 'literal body))
            ; ((eq? indicator #\S) (list 'literal (to-human body)))
            ((eq? indicator #\U) (list 'unary (unary-operator (string-ref body 0))))
            ((eq? indicator #\B)
                (if (eq? (string-ref token 1) #\$)
                    (list 'apply)
                    (list 'binary (binary-operator (string-ref body 0)))))
            ((eq? indicator #\?) (list 'if))
            ((eq? indicator #\L) (list 'lambda (icfp-string->number body)))
            ((eq? indicator #\v) (list 'symbol (icfp-string->number body)))
            ((eq? indicator #\Y) (list 'y-combinator (icfp-char->number (string-ref body 0)) (icfp-char->number (string-ref body 1)) ))
            (else
                (print "Unknown token: " token)
                'nil)
        ) ))


;(define (Y f)
;    ((lambda (x) (f (lambda (y) (x x y))) (lambda (x) (f (lambda (y) (x x y))))))
;(define (Z f)
    ;((lambda (x) (f (lambda (y) (x x y))) (lambda (x) (f (lambda (y) (x x y)))))))

(define (z-combinator f x y)
  #"B$ L~f B$ L~x B$ v~f L~y B$ B$ v~x v~x v~y L~x B$ v~f L~y B$ B$ v~x v~x v~y")

(define (replace-y-combinator s)
    (regexp-replace-all
        #/B\$ L(.) B\$ L(.) B\$ v(.) B\$ v(.) v(.) L(.) B\$ v(.) B\$ v(.) v(.)/
        s
        (lambda (m)
            (let ([f (rxmatch-substring m 1)]
                  [x (rxmatch-substring m 2)])
               (if (and (string=? f (rxmatch-substring m 3) (rxmatch-substring m 7))
                        (string=? x (rxmatch-substring m 4) (rxmatch-substring m 5) (rxmatch-substring m 6) (rxmatch-substring m 8) (rxmatch-substring m 9)) )
                   (z-combinator f x "%")
                   (rxmatch-substring m 0)
                   )))))

(define (parse s)
    (let1 pass1 (replace-y-combinator s)
        (map read-token (string-split pass1 " "))))

(define (make-symbol num)
    (string->symbol (string-append "v" (number->string num))))

(define (make-ast elems)
  (define (take-one seq)  ;; -> (obj rest)
    (if (null? seq) (values #f ())
        (let* ([elem (car seq)]
               [rest (cdr seq)]
               [type (if (null? elem) #f (car elem))])
            (cond
                [(eq? type 'literal)
                    (values (cadr elem) rest)]
                [(eq? type 'unary)
                    (let1 op (cadr elem)
                        (receive (arg rest) (take-one rest)
                            (values (list op arg) rest) ))]
                [(eq? type 'binary)
                    (let1 op (cadr elem)
                        (receive (arg1 rest) (take-one rest)
                          (receive (arg2 rest) (take-one rest)
                              (values (list op arg1 arg2) rest) )))]
                [(eq? type 'apply)
                    (receive (arg1 rest) (take-one rest)
                      (receive (arg2 rest) (take-one rest)
                          (values (list arg1 arg2) rest)))]
                [(eq? type 'if)
                    (receive (cl-cond rest) (take-one rest)
                      (receive (cl-then rest) (take-one rest)
                        (receive (cl-else rest) (take-one rest)
                           (values (list 'if cl-cond cl-then cl-else) rest) )))]
                [(eq? type 'lambda)
                    (let1 sym (make-symbol (cadr elem))
                        (receive (body rest) (take-one rest)
                            (values (list 'lambda (list sym) body) rest) ))]
                [(eq? type 'symbol)
                    (let1 sym (make-symbol (cadr elem))
                            (values sym rest))]
                [(eq? type 'y-combinator)
                    (let ([sym1 (make-symbol (cadr elem))]
                          [sym2 (make-symbol (caddr elem))])
                        (receive (fun rest) (take-one rest)
                          (receive (arg rest) (take-one rest)
                            (values (list 'y-combinator fun arg) rest)
                          )))]
                [else (values 'nil rest)]
            ))))
  (receive (obj rest) (take-one elems)
    obj))

(define (read-icfp s)
    (print "\n" s)
    (let1 tokens (parse s)
        (print "* tokens: " tokens)
        (let1 ast (make-ast tokens)
            (print "* ast: " ast)
            (let1 value (eval ast ())
                (print "* eval: " value)
                value))))

(define (eval-icfp s)
  (let* ([tokens (parse s)]
         ;[_ (print tokens)]
         [ast (make-ast tokens)]
         ;[_ (print ast)]
         [value (eval ast ())])
         value))

#|
(use gauche.test)
(test-start "parse")

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
|#

; 1
;(print (read-icfp "B$ L! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! B$ v! I\" L! B+ B+ v! v! B+ v! v!"))

; 2
;(print(read-icfp "B+ I7c B* B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L$ L% ? B= v% I! I\" B+ I\" B$ v$ B- v% I\" I\":c1+0 I!"))

;12
;(print (read-icfp "B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L$ L% B$ B$ L\" L# ? B< v\" v# v\" v# v% B+ I\" ? B> v% I# B$ B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L& L' L( ? B= v' v% v( B$ B$ v& B+ v' I\" ? B> B$ v$ v' B- v' I\" ? B= B% v% v' I! B* B/ v( B$ v$ v' B- B$ v$ v' I\" v( v( I# v% v% I\"Ndb"))

; lambdaman21
(use file.util)
;(let1 raw (file->string "data/lambdaman/lambdaman20.raw")
;  (print (to-human (eval-icfp raw))))

(define (main args)
  (if (null? (cdr args))
      #f
      (let* ([input-file (cadr args)]
             [mode (if (null? (cddr args)) #f (caddr args))]
             [code (file->string input-file)]
             [decoded (eval-icfp code)])
        (if mode
            (print (to-human decoded))
            (print decoded)) ))
  0)
