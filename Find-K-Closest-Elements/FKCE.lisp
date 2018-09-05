(defun find-k-closest-elements (arr k x)
  (let* ((i_lager (loop with i = 0
		     for v in arr
		     while (< v x)
		     do (incf i)
		     finally (return i)))
	 (i (if (> (- i_lager k) 0)
		(- i_lager k)
		0))
	 (j (if (< (+ i_lager k) (length arr))
		(+ i_lager k)
		(length arr)))
	 (inner_arr (subseq arr i j)))
    (sort
     (subseq
      (sort inner_arr
	    #'< :key
	    #'(lambda (a) (abs (- a x))))
      0 k)
     #'<)))

(find-k-closest-elements '(1 2 3 4 5) 4 3)
(find-k-closest-elements '(0 0 0 1 3 5 6 7 8 8) 2 2)
