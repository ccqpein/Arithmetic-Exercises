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
