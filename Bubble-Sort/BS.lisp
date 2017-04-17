;;; writing in Common Lisp, you can run this algorithm in https://www.tutorialspoint.com/execute_lisp_online.php 

(defvar *test '(14 33 27 35 10))

(defun bubble-sort (nl func)
  (do ((numL nl)
       (flag 1)) ;to mark whether the algorithm is done
      ((= flag 0) numL) ;flag is 0 mean the algorithm is finish
    (print numL)
    (setf flag 0)
    (let ((head '()) ;head is result list
          (len (length numL)))
      (loop for ind from 1 to (1- len) ;traverse all elements (with index) except the first one
         with tempEle = (nth 0 numL) ;make temp element to compare with, first element
         do (if (funcall func tempEle (nth ind numL))
                ;if func return 'yes', update tempEle, and append this element to head
                (progn (setf head (append head (list tempEle)))
                       (setf tempEle (nth ind numL)))
                ;if func return 'no', append this element to head, do not update tempEle
                (progn (setf head (append head (list (nth ind numL))))
                       (setf flag 1))) ;set new flag
         finally (setf head (append head (list tempEle)))) ;do not forget the last element
      (setf numL head) ;update the numL
      )))

;; test algorithm
(bubble-sort *test #'<)
(bubble-sort *test #'>)
