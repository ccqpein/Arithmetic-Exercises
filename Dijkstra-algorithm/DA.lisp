;;; give a matrix which included all distances between each neighbour point
;;; example [url]<http://www.cnblogs.com/biyeymyhjob/archive/2012/07/31/2615833.html>

#|
Example:

  |A B C D E F
 A|0 6 3 n n n
 B|6 0 2 5 n n
 C|3 2 0 3 4 n
 D|n 5 3 0 2 3
 E|n n 4 2 0 5
 F|n n n 3 5 0

|#

(defvar *Example (make-array '(6 6) :initial-contents
                             '((0 6 3 nil nil nil)
                               (6 0 2 5 nil nil)
                               (3 2 0 3 4 nil)
                               (nil 5 3 0 2 3)
                               (nil nil 4 2 0 5)
                               (nil nil nil 3 5 0))))

;;; The marcos below come from the machine learning package toolsbox
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defmacro with-gensyms ((&rest names) &body body)
  `(let ,(loop for n in names collect `(,n (gensym)))
     ,@body))

(defmacro array-slice (m i)
  "get array from matrix, m is matrix i is index"
  (with-gensyms (mm ii)
    `(let* ((,mm ,m)
            (,ii ,i)
            (dim (array-dimensions ,mm))
            (colNum (cadr dim)))
       (make-array colNum :initial-contents
                   (loop for id from 0 to (1- colNum) collect
                        (aref ,mm ,ii id))))))

(defmacro aappend (l &rest eles)
  "l must be symbol not expression. For example, (aappend a 2 3 4) is fine, (aappend '(1 2) 2 3) and (aappend (list 2 3) 2 2) will issue error"
  (with-gensyms (elel)
    `(let ((,elel (list ,@eles)))
       (loop for i in ,elel do
            (setf ,l (append ,l (list i))))
       ,l)))

;;;;;;; Borrow tools finished ;;;;;;;;;;;;;;;

;; This function can be more flexible than now, I might change it later
(defmacro second-last-position (l)
  "return second last element index in list."
  (let* ((l0 (eval l))
         (l1 (remove 'nil l0))
         (l2 (gensym)))
    `(let* ((,l2 (list ,@l1)))
       (position (cadr (sort ,l2 #'<)) '(,@l0)))))

(defun get-val (l &optional (m *Example))
  "l is index-list for store all index numbers, this function is get the sum val follow l index points"
  (let ((sum 0)
        (len (length l)))
    (do ((first 0 (1+ first))
         (second 1 (1+ second)))
        ((= second len))
      (setf sum (+ sum (aref m (nth first l)
                             (nth second l)))))
    sum))

(defun dijkstra (start end &optional (m *Example))
  "start and end are the index in matrix"
  (let ((s '())
        (u (loop for i from 0 to (1- (car (array-dimensions m))) collect i))
        (storeList '()))

  ))
