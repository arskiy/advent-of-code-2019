(load "utils.scm")
  
(define (fuel-calc n)
  (if
    (< n 1)
    0
    (+ n (fuel-calc (module-calc n)))))
    
(define (fuel-calc-wrap n)
  (fuel-calc (module-calc n)))
 
(display (reduce + 0 (map fuel-calc-wrap input)))
(newline)
