(ql:quickload "str")

(defun compare-version (v1 v2)
  (let ((vv1 (str:split #\. v1))
		(vv2 (str:split #\. v2)))
	(do ((x vv1 (cdr x))
		 (y vv2 (cdr y)))
		((and (not x) (not y))
		 0)
	  
	  (if (= 0 (length x)) (push "0" x))
	  (if (= 0 (length y)) (push "0" y))
	  
	  (let ((xx (parse-integer (car x)))
			(yy (parse-integer (car y))))
		
		(cond ((< xx yy)
			   (return-from compare-version -1))
			  ((> xx yy)
			   (return-from compare-version 1))
			  )
		))))

(defun main ()
  (assert (= 0 (compare-version "1.01" "1.001")))
  (assert (= 0 (compare-version "1.0" "1.0.0")))
  (assert (= -1 (compare-version "0.1" "1.1")))
  (assert (= -1 (compare-version "1.1" "1.10")))
  )
