(defvar dict (make-hash-table))
(defvar chars '(#\1 #\2 #\3 #\4 #\5 #\6 #\7 #\8 #\9 #\0))
(defvar ints '(1 2 3 4 5 6 7 8 9 0))

(loop for int in ints
   for char in chars
   do (setf (gethash char dict) int)
   ;do (setf (gethash int dict) char)
     )

(declaim (inline sum-these))

(defun sum-these (num ind)
  (* (gethash (elt num ind) dict)
     (expt 10 (- (length num) ind 1))))

(defun add-strings (num1 num2)
  (declare (string num1 num2))
  (let ((sum 0))
    (do ((ind1 0 (1+ ind1))
	 (ind2 0 (1+ ind2))
	 )
	((and (>= ind1 (length num1))
	      (>= ind2 (length num2)))
	 (write-to-string sum))
      (if (< ind1 (length num1))
	  (setf sum (+ sum (sum-these num1 ind1))))
      (if (< ind2 (length num2))
	  (setf sum (+ sum (sum-these num2 ind2)))))))
