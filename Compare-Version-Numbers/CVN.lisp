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

(defun compare-version2 (v1 v2)
  "v1 & v2 format: (1 3 4)"
  (let ((max-len (max (length v1) (length v2))))
	(loop for i from 0 below max-len
		  for x = (if (nth i v1) (nth i v1) 0)
		  and y = (if (nth i v2) (nth i v2) 0)
		  do (cond ((> x y) (return-from compare-version2 1))
				   ((< x y) (return-from compare-version2 -1))))
	0
	))

(defun main ()
  (assert (= 0 (compare-version "1.01" "1.001")))
  (assert (= 0 (compare-version "1.0" "1.0.0")))
  (assert (= -1 (compare-version "0.1" "1.1")))
  (assert (= -1 (compare-version "1.1" "1.10")))

  (assert (= 0 (compare-version2 '(1 1) '(1 1))))
  (assert (= 0 (compare-version2 '(1 0) '(1 0 0))))
  (assert (= -1 (compare-version2 '(0 1) '(1 1))))
  (assert (= -1 (compare-version2 '(1 1) '(1 10))))
  )
