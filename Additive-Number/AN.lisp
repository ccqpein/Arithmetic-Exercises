(ql:quickload "str")

(defun helper (x y str ind)
  ;;(format t "~a ~a~%" x y)
  (cond ((= ind (length str))
		 t)
		(t
		 (let ((ss (format nil "~a" (+ x y))))
		   (if (str:starts-with? ss (subseq str ind))
			   (helper y (+ x y) str (+ ind (length ss)))
			   nil)))))

(defun is-additive-number (str)
  (loop for i from 1 below (length str)
		if (and (> i 1) (char= #\0 (elt str 0)))
		  return nil
		do (loop for j from (1+ i) below (length str)
				 unless (and (char= #\0 (elt str i)) (/= 1 (- j i)))
				   if (helper (parse-integer (subseq str 0 i))
							  (parse-integer (subseq str i j))
							  str
							  j)
					 do (return-from is-additive-number t)
				 )))

(defun main ()
  (assert (is-additive-number "112358"))
  (assert (is-additive-number "199100199"))
  (assert (not (is-additive-number "111")))
  (assert (is-additive-number "000"))
  (assert (is-additive-number "101"))
  (assert (not (is-additive-number "1023")))
  (assert (not (is-additive-number "0235813")))
  )
