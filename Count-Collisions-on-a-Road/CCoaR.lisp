(defun count-collisions (directions)
  (let ((chars (concatenate 'list directions))
		(left 0)
		(right (1- (length directions))))

	(if (zerop right)
		(return-from count-collisions 0))

	(loop
	  while (and (< left (length chars))
			  (char= #\L (nth left chars)))
		do (incf left))

	(loop
	  while (and (> right 0)
			  (char= #\R (nth right chars)))
		do (decf right))

	;;(format t "~a:~a" left right)
	(loop
	  for i from left to right
	  if (char= #\S (nth i chars))
		count t
	  )
	))
