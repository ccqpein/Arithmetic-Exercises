(defun smallest-from-leaf (nodes)
  "nodes are list"
  )

(defun all-results (nodes)
  (if (not nodes) (return-from all-results nil))

  (let ((result '())
        (left-re (all-results (cadr nodes)))
        (right-re (all-results (caddr nodes))))
    ;;(format t "~a;~a~%" left-re right-re)
    (if (and (not left-re) (not right-re)) (return-from all-results (list (list (car nodes)))))
    (setf result
          (append result
                  (mapcar (lambda (x) (cons (car nodes) x)) left-re)
                  (mapcar (lambda (x) (cons (car nodes) x)) right-re)))
    result
    ))

(defun )

(defun main ()
  (let ((test0 '(0
                 (1
                  (3) (4))
                 (2
                  (3) (4)))) ;; dba
        (test1 '(2
                 (2
                  nil
                  (1
                   (0)
                   nil))
                 (1
                  (0)
                  nil))) ;; abc
        )
    (all-results test1)
    ))

