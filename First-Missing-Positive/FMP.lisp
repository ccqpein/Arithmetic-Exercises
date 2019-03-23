(defun first-mission-position (num-l)
  (let ((sorted-num (sort (copy-list num-l) #'<))
		(ind 1))
	(loop
	   for num in sorted-num
	   when (> num 0)
	   if (/= ind num)
	   do (if (/= (- ind 1) num)
			  (return ind))
	   else
	   do (incf ind)
		 )
	ind
	))

