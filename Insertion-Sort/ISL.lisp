;; Depending on lisp's list structure, I do not need to create new data structure
;; This code is translate from haskell version, so it looks have weird recursion

(defvar *test '(2 1 3))

(defun insert-sort-list (l)
  (let ((rel '()))
    (labels ((insert-list (l a)
               (cond ((eql 'nil l)
                      (list a))
                     ((< a (car l))
                      (cons a (cons (car l) (cdr l))))
                     ((>= a (car l))
                      (cons (car l) (insert-list (cdr l) a)))))
             (insert-list-outer (rel l)
               (cond ((eql 'nil l)
                      rel)
                      (t
                       (insert-list-outer (insert-list rel (car l)) (cdr l))))))
      (insert-list-outer rel l))))
