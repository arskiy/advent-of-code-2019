; Needed on Guile
;(use-modules (ice-9 rdelim))
;(use-modules (srfi srfi-1))

(define (tokenize l s)
  (let loop ((t '())
             (l l))
    (if (pair? l)
        (let ((c (car l)))
          (if (char=? c s)
              (cons (reverse t) (loop '() (cdr l)))
              (loop (cons (car l) t) (cdr l))))
        (if (null? t)
            '()
            (list (reverse t))))))

(define (string-split s x)
  (map list->string (tokenize (string->list s x))))

(define (readlines filename)
  (call-with-input-file filename
    (lambda (p)
      (let loop ((line (read-line p))
                 (result '()))
        (if (eof-object? line)
            (reverse result)
            (loop (read-line p) (cons line result)))))))

(define (list-set list k val)
    (if (zero? k)
        (set-car! list val)
        (list-set (cdr list) (- k 1) val)))

(define input (map string->number (string-split (readlines "input.txt") ',')))
