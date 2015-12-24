(defun isAnagram (s tt)
    (do* ((ss (coerce s 'list) (cdr ss))
          (ttt (reverse (coerce tt 'list)) (cdr ttt)))
         ((not (eql (car ss) (car ttt))) (format t "succes"))))
