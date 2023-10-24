(defun apply-operations (nums)
  (let ((result nil))
	(let (last)
	  (setf last
			(reduce (lambda (a b)
					  (if (/= a b)
						  (progn
							(if (/= a 0)
								(setf result (append result (list a))))
							b)
						  (progn
							(if (/= a 0)
								(setf result (append result (list (* 2 a)))))
							0))
					  )
					nums
					:initial-value -1))
	  (setf result (append result
						   (list last)
						   (make-list (- (length nums)
										 (length result))
									  :initial-element 0)))
	  (cdr result))))

(defun main ()
  (princ (apply-operations '(1 2 2 1 1 0)))
  (princ (apply-operations '(0 1)))
  nil
  )
