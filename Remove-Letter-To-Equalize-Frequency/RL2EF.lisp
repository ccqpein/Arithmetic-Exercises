(ql:quickload "str")
(ql:quickload "alexandria")

(defun equal-frequency (s)
  (let ((chars (concatenate 'list s))
		(chars-table (make-hash-table :test 'equal)))
	(dolist (c chars)
	  (incf (gethash c chars-table 0)))

	(let ((freq-table (make-hash-table)))
	  (dolist (f (alexandria:hash-table-values chars-table))
		(incf (gethash f freq-table 0)))

	  ;;(format t "~a" (alexandria:hash-table-plist freq-table))

	  (let ((freq-keys (sort (alexandria:hash-table-keys freq-table)
							 #'<)))
		
		(if (= 1 (length freq-keys))
			(if (or (= 1 (car freq-keys)) (= 1 (gethash (car freq-keys) freq-table)))
				(return-from equal-frequency t)))
		
		(if (/= 2 (length freq-keys))
			(return-from equal-frequency nil))
		
		(if (= 1 (gethash 1 freq-table))
			(return-from equal-frequency t))
		
		(if (and (= 1 (- (second freq-keys) (first freq-keys)))
				 (= 1 (gethash (second freq-keys) freq-table)))
			(return-from equal-frequency t))
		
		nil))))

(defun main ()
  (assert (equal-frequency "abcc"))
  (assert (not (equal-frequency "aazz")))
  (assert (equal-frequency "bac"))
  (assert (equal-frequency "zz"))
  )
