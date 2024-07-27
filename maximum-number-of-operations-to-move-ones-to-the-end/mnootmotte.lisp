(ql:quickload "cl-ppcre")

(defun max-operations (s)
  (let ((ss (cl-ppcre::all-matches-as-strings "(1+0+)" s)))
    (loop 
      for n in (mapcar (lambda (s) (count #\1 (concatenate 'list s))) ss)
      sum (+ result n) into result
      finally (return result))
    ))

(defun main ()
  (assert (= (max-operations "1001101") 4))
  (assert (= (max-operations "00111") 0))
  (assert (= (max-operations "110") 2))
  )
