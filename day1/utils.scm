; Needed on Guile
;(use-modules (ice-9 rdelim))
;(use-modules (srfi srfi-1))

(define (readlines filename)
  (call-with-input-file filename
    (lambda (p)
      (let loop ((line (read-line p))
                 (result '()))
        (if (eof-object? line)
            (reverse result)
            (loop (read-line p) (cons line result)))))))

(define input (map string->number (readlines "input.txt")))

(define (module-calc n)
  (- (floor (/ n 3)) 2))
