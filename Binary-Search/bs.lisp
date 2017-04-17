;;; writing in Common Lisp, you can run this algorithm in https://www.tutorialspoint.com/execute_lisp_online.php 

(defvar *test '(10 14 19 26 27 31 33 35 42 44)) ;test data

(defun binary-search (nl num &optional (tempInd 0))
  "if num larger than mid value, tempInd store indexs which have calcilated before."
  (let* ((len (length nl))
         (midInd (multiple-value-bind (a) (ceiling (/ (1- len) 2)) a)) ;middle index
         (midNum (nth midInd nl))) ;middle value
    ;(print (list midInd midNum tempInd))
    (cond ((= num midNum)
           (print (+ tempInd midInd)))
          ((> num midNum) ;search right side
           (binary-search (subseq nl midInd len) num (+ tempInd midInd))) ;recursive with updated tempind
          ((< num midNum) ;search left side
           (binary-search (subseq nl 0 midInd) num)))));recursive

;; test algorithm
(binary-search *test 10)
(binary-search *test 14)
(binary-search *test 31)
(binary-search *test 44)
