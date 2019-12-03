(load "utils.scm")

(display (reduce + 0 (map module-calc input)))
(newline)
