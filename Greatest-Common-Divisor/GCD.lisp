(defun remove-zero (l)
  (remove-if #'(lambda (ele) (= 0 ele)) l))

(defun GCD-l (l)
  (let* ((inner-l (remove-zero l))
		(smallest (apply #'min inner-l)))
	(if (= 1 (length l))
		(car l)
		(GCD-l (cons smallest
					 (remove-zero (mapcar #'(lambda (ele)
											  (- ele smallest))
										  inner-l)))))))

(GCD-L '(4 18 22 16)) ;;=> 2
(GCD-L '(120 168 328 624 320)) ;;=> 8

(labels ((euclidA (a b)
		   (if (= 0 b)
			   a
			   (euclidA b (mod a b)))))
  (defun GCD2 (l)
	(if (= 1 (length l))
		(car l)
		(euclidA (car (last l))
				 (GCD2 (butlast l)))))
  
  (defun GCD-recursive (l &optional last)
	(cond ((eql nil last)
		   (GCD-recursive (cdr l) (car l)))
		  ((eql nil l)
		   last)
		  (t
		   (GCD-recursive (cdr l) (euclidA last (car l)))))))

