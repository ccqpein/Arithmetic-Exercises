;; status: up 1, flat 0, down -1
(defun longest-mountain (arr)
  (let ((status 0)
		(last (car arr))
		(leng 0)
		(bucket '(0)))
	(loop for i in arr
		  do (case status
			   (1 (cond ((> i last)
						 (incf leng))
						((< i last)
						 (setf status -1)
						 (incf leng))
						(t (setf status 0
								 leng 0))))
			   (-1 (cond ((> i last)
						  (setf status 1
								bucket (cons leng bucket)
								leng 2))
						 ((< i last)
						  (incf leng))
						 (t (setf status 0
								  bucket (cons leng bucket)
								  leng 0))))
			   (0 (cond ((> i last)
						 (setf status 1
							   leng 2)))))
		  do (setf last i))

	(if (= -1 status)
		(push leng bucket))

	(apply #'max bucket)
	))

(defun main ()
  (assert (= (longest-mountain '(2 1 4 7 3 2 5)) 5))
  (assert (= (longest-mountain '(2 2 2)) 0))
  (assert (= (longest-mountain '(0 1 2 3 4 5 4 3 2 1 0)) 11))
  (assert (= (longest-mountain '(2 3)) 0))
  (assert (= (longest-mountain '(85 884 239 731 723 685)) 4)))
