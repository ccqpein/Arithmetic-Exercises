(defun fib (n)
  (case n
	(0 0)
	(1 1)
	(otherwise
	 (do ((re 0)
		  (llast-n 0)
		  (last-n 1)
		  (time 0 (1+ time)))
		 ((>= time (1- n)) re)
	   (setf re (+ llast-n last-n)
			 llast-n last-n
			 last-n re)
	   ))))
