(defun number-of-arithmetic-slices (l)
  (let ((longest '())
		(last-long 0)
		(last-diff nil))

	(loop for i from 1 below (length l)
		  if last-diff
			do (if (/= last-diff (- (nth i l) (nth (1- i) l)))
				   (if (>= last-long 3)
					   (setf longest (cons last-long longest)
							 last-long 2
							 last-diff (- (nth i l) (nth (1- i) l)))
					   (setf last-long 2
							 last-diff (- (nth i l) (nth (1- i) l))))
				   (incf last-long))
		  else
			do (setf last-long 2
					 last-diff (- (nth i l) (nth (1- i) l)))
		  end)

	(if (/= 0 last-long)
		(setf longest (cons last-long longest)))

	(reduce #'+ (mapcar (lambda (x) (loop for i from 1 to (- x 2) sum i)) longest))
	)
  )

(defun main ()
  (assert (= (number-of-arithmetic-slices '(1 3 5 7 9)) 6))
  (assert (= (number-of-arithmetic-slices '(7 7 7 7)) 3))
  (assert (= (number-of-arithmetic-slices '(3 -1 -5 -9)) 3))
  (assert (= (number-of-arithmetic-slices '(1 2 3 4)) 3))
  (assert (= (number-of-arithmetic-slices '(1)) 0))
  (assert (= (number-of-arithmetic-slices '(1 2 3 5 7)) 2))

  
  )
