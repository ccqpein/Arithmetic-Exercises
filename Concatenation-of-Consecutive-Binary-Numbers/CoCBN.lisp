(defun concatenated-binary (n)
  (let ((result 0)
		(space 1)
		(count 0))
	(loop for i from 1 to n
		  if (>= i space)
			do (setf space (ash space 1)
					 count (1+ count))
		  do (setf result (mod (+ i (ash result count)) 1000000007))
		  )
	result))

(assert (= (concatenated-binary 1) 1))
(assert (= (concatenated-binary 3) 27))
(assert (= (concatenated-binary 12) 505379714))

