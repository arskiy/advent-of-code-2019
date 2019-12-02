; guile 2.2.6

(use-modules (ice-9 rdelim))
(use-modules (srfi srfi-1))

(define (module-calc n)
  (- (floor (/ n 3)) 2))
  
(define (fuel-calc n)
  (if
    (< n 1)
    0
    (+ n (fuel-calc (module-calc n)))))
    
(define (fuel-calc-wrap n)
  (fuel-calc (module-calc n)))
 
(define (readlines filename)
  (call-with-input-file filename
    (lambda (p)
      (let loop ((line (read-line p))
                 (result '()))
        (if (eof-object? line)
            (reverse result)
            (loop (read-line p) (cons line result)))))))

(define input (map string->number (readlines "input.txt")))

(display "Part 1: ")
(display (reduce + 0 (map module-calc input)))
(newline)
(display "Part 2: ")
(display (reduce + 0 (map fuel-calc-wrap input)))
(newline)
