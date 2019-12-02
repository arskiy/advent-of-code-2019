(load "utils.scm")

(define (restore l i)
    (cond 
      ((= (list-ref l i) 99) 
       (list-ref l 0))
      ((= (list-ref l i) 1) 
       (list-set l 
                 (list-ref l (list-ref l (+ i 3)))
                 (+ (list-ref l (+ i 1)) (list-ref l (+ i 2)))))
      ((= (list-ref l i) 2) 
       (list-set l 
                 (list-ref l (list-ref l (+ i 3)))
                 (+ (list-ref l 
                              ((list-ref l (+ i 1))
                               (list-ref l (+ i 2))))))))
    (restore l (+ i 4)))

(define (restore-wrap l) (restore l 0))

(define inp input)
(list-set inp 1 12)
(list-set inp 2 2)
(display (restore-wrap inp))
