(defun min-impossible-or (nums)
  (let ((s (make-hash-table)))
	(loop for n in nums
		  if (= n (logand n (- n)))
			do (setf (gethash n s) t)
		  )

	(let ((i 1))
	  (loop while (gethash i s)
			do (setf i (ash i 1)))

	  i
	  )))
