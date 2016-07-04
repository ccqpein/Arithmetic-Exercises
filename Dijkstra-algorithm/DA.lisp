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
#|(defmacro second-last-position (l) ;this function doesnt need anymore.
  "return second last element index in list."
  (let* ((l0 (eval l))
         (l1 (remove 'nil l0))
         (l2 (gensym)))
    `(let* ((,l2 (list ,@l1)))
       (position (cadr (sort ,l2 #'<)) '(,@l0)))))|#

(defun get-val (l &optional (m *Example))
  "l is index-list for store all index numbers, this function is get the sum val follow l index points"
  (let ((sum 0)
        (len (length l)))
    (do ((first 0 (1+ first))
         (second 1 (1+ second)))
        ((= second len))
      (let ((val (aref m (nth first l)
                       (nth second l))))
      (setf sum (if val
                    (+ sum val)
                    (return-from get-val nil)))))
    sum))

(defun dijkstra (start &optional (m *Example))
  "start and end are the index in matrix"
  (let ((s)  ;s is the smallest index in each step, translate s from l2 to l1
        (u (loop for i from 0 to (1- (car (array-dimensions m))) collect i))
        (storeList '())  ;store all paths in the proccessing
        (smallest '()))  ;store the smallest path in each step
    (do ((l1 (list start)) ;the path set for next step, updated by "smallest"
         (l2 (delete start u) (delete s l2)))  ;the set including points which not been searched
        ((not l2) (print "done"))
      (let ((tempStore)
            (smallOne))
        ;(print "start") (print "This is L1") (print l1) (print "This is L2") (print l2)
        (loop for el in l2 do
             (let ((val (get-val (append l1 (list el)) m)))
               ;(print (append l1 (list el))) (print val)
               (if val (progn
                         (loop for i in storeList for vv = (cdr i) do
                              (if (and (= (car (last (car i))) el) (< vv val))
                                  (setf smallOne i)))
                         (aappend tempStore (cons (append l1 (list el)) val))))
               (if (and smallOne val)
                   (if (< val (cdr smallOne))
                       (setf smallOne (cons (append l1 (list el)) val)))
                   (if val
                       (setf smallOne (cons (append l1 (list el)) val))))))
        (setf s (car (last (car smallOne))))
        #|(print "here is tempstore")
        (print tempStore)
        (print "this is smallone")
        (print smallOne)        
        (print "this is s")
        (print s)|#
        (setf l1 (car smallOne))
        (setf storeList (append storeList tempStore))
        (aappend smallest smallOne)
        ))
    ;(print storeList)
    smallest))
