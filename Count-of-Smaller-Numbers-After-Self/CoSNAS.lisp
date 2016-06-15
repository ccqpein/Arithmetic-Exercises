(defun countSmaller (nums)
  (do* ((numlist nums b)
	(a (car numlist)
	   (car numlist))
	(b (cdr numlist)
	   (cdr numlist))
	(time (telltime a b)
	      (append time (telltime a b))))
       ((= (length b) 1) (append time (list 0)))))

(defun telltime (a b)
  (let ((time 0)
	(a a)
	(b b))
    (loop for item in b when (< item a) do
         (incf time))
    (return-from telltime (list time))))
	 
