;;; writing in Common Lisp, you can run this algorithm in https://www.tutorialspoint.com/execute_lisp_online.php 

(defvar *test '(3 7 4 9 5 2 6 1)) ;test data

(defun insertion-sort (nl func)
  (let ((head (list (car nl)))) ;head is result list
    (loop for i in (cdr nl) ;traverse all elements except the first one
       do 
         (let ((lagerInd (position-if #'(lambda (x) (funcall func i x)) head)))
           ;lagerInd is the index that elements start larger/smaller than this element
           (if (eql nil lagerInd)
               (setf head (append head (list i))) ;if nil, means this element is largest/smallest
               (setf head (append (subseq head 0 lagerInd) ;if not nil, insert this element in head
                                  (list i)
                                  (subseq head lagerInd))))
           (print head)));print head for debug
    head));return

;; test algorithm
(insertion-sort *test #'<)
(insertion-sort *test #'>)
