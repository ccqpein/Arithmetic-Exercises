(defun smallest-from-leaf (nodes)
  "nodes are list"
  (let ((all-re (all-results nodes))
        )
    (create-str (find-least all-re))
    ))

(defun all-results (nodes)
  (if (not nodes) (return-from all-results nil))

  (let ((result '())
        (left-re (all-results (cadr nodes)))
        (right-re (all-results (caddr nodes))))

    (if (and (not left-re) (not right-re)) (return-from all-results (list (list (car nodes)))))
    (setf result
          (append result
                  (mapcar (lambda (x) (cons (car nodes) x)) left-re)
                  (mapcar (lambda (x) (cons (car nodes) x)) right-re)))
    result
    ))

(defun find-least (ll)
  (let ((reverse-ll (mapcar #'reverse ll)))
    (labels ((compare-inner (a b)
               (do ((aa a (cdr aa))
                    (bb b (cdr bb)))
                   ((or (not aa) (not bb))
                    (return (if (not aa)
                                a
                                b)))
                 (cond ((< (car aa) (car bb)) (return-from compare-inner a))
                       ((> (car aa) (car bb)) (return-from compare-inner b))))
               ))
      (reduce #'compare-inner reverse-ll))
    ))

(defun create-str (l)
  (coerce (mapcar (lambda (x) (code-char (+ 97 x))) l) 'string))

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
    ;;(create-str (find-least (all-results test0)))
    (smallest-from-leaf test1)
    ))
