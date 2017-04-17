;;; writing in Common Lisp, you can run this algorithm in https://www.tutorialspoint.com/execute_lisp_online.php

(defvar *test '(14 33 27 10 35 19 42 44)) ;test data

(defun selection-sort (nl)
  (do ((head '()) ;head is result
       (tail nl))
      ((= 0 (length tail)) head)
    (print head)
    (let ((minVal (loop for i in tail minimize i))) ;minVal is the smallest element in tail
      (setf head (append head (list minVal))) ;add minVal to head
      (setf tail (remove minVal tail :count 1))))) ;remove minVal from tail

;; test algorithm
(selection-sort *test)
