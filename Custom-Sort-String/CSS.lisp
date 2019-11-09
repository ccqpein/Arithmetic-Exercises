(defun make-sort-func (s)
  (let ((sb (concatenate 'list s)))
    (labels ((inner-sort (a b)
               (let ((a-position (position a sb))
                     (b-position (position b sb)))
                 (cond ((not (and a-position b-position))
                        (cond ((not a-position) nil)
                              (t t)))
                       ((>= a-position b-position)
                          nil)
                       (t t)))))
      (lambda (s)
        (sort s #'inner-sort))
      ))
  )

(defun custom-sort-string (s ta)
  (funcall (make-sort-func s) (concatenate 'list ta))
  )
