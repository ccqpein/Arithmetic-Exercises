(defun compress (l)
  ;(print l)
  (cond ((eql nil l) '())
	((listp (car l))
	 (append (compress (car l))
		 (compress (cdr l))))
	(t
	 (append (list (car l)) (compress (cdr l))))))

(print (compress '(1 (2 3 (4 5) (6 7 (8)) 9) 10)))

