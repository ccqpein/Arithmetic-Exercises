(defpackage #:matrix-operate
  (:use #:CL)
  (:nicknames #:MX)
  (:export #:do-around
           #:sum-list
           #:max-list
           #:longest-list
           #:*matrix*
           #:*matrix2*))

(in-package #:matrix-operate)

(load "/Users/ccQ/quicklisp/setup.lisp")
(ql:quickload "chanl")

(defvar *matrix* '((9 9 4) (6 6 8) (2 1 1)))
(defvar *matrix2* '((3 4 5) (3 2 2) (2 2 1)))

(defun find-Ele (matrix row col)
  (let ((ele))
    (if  (and (>= row 0) (>= col 0))
         (progn (setf ele  (nth col (nth row matrix)))
                (if ele
                    (return-from find-Ele (list ele (list row col) T)))
                (return-from find-Ele nil)))))

(defun do-around (func matrix row col)
  (let ((rowlist (list (1+ row) (1- row)))
        (collist (list (1+ col) (1- col)))
        (result))
    (setf result (remove 'nil
          (list
           (funcall func matrix row col)
           (funcall func matrix (nth 0 rowlist) col)
           (funcall func matrix (nth 1 rowlist) col)
           (funcall func matrix row (nth 0 collist))
           (funcall func matrix row (nth 1 collist)))))
    result))

(defun how-many-larger (resultFromDoAround)
  "return lagerest number and the index"
  (let ((thisNum (caar resultFromDoAround))
        (lastList (cdr resultFromDoAround))
        (largerNum 0)
        (largerIndex '()))
    (dolist (i lastList (list largerNum largerIndex))
      (if (> (car i) thisNum)
          (progn ;(print i)
            (incf largerNum)
                (setf largerIndex (append largerIndex (list (cadr i))))
          )))))

(defmacro change-index-to-string (r c)
  "return value type is r-c"
  `(intern (coerce (list (code-char (+ 48 ,r)) #\- (code-char (+ 48 ,c))) 'string)))

(defun gen-index-table (matrix)
  (let ((rowNum (length matrix))
        (colNum (length (car matrix)))
        (indexHash (make-hash-table)))
    (loop for r from 0 to (1- rowNum) do
       (loop for c from 0 to (1- colNum) do
            (progn ;(print (change-index-to-string r c))
              (setf (gethash (change-index-to-string r c) indexHash)
                    (how-many-larger (do-around 'find-ele matrix r c)))
              ;(print (gethash (change-index-to-string r c) indexHash))
              )))
    indexHash))

(defun sum-list (list)
  (let ((summation 0))
    (dolist (i list summation)
      (setf summation (+ i summation)))))

(defun max-list (list)
    (loop for element in list maximizing element))

(defun longest-list ( &rest llist)
  (let ((length-list '()))
    ;(print llist)
    (loop for i in llist do (progn
                              ;(print i)
                              (setf length-list (append length-list (list (length i))))
                              ;(print length-list)
                              ))
    (let ((largerNum (max-list length-list)))
      (loop for i from 0 to (1- (length length-list)) do
           (if (= largerNum (nth i length-list))
               (return-from longest-list (nth i llist)))))))

(defun tree-search-matrix (indexTable r c)
  (let ((this-hash (gethash (change-index-to-string r c) indexTable))
        (result))
    ;(print (nth 1 this-hash))
  (cond ((= 0 (car this-hash)) (progn ;(print (list r c))
         (return-from tree-search-matrix '(1) )))
        ((= 1 (car this-hash)) (progn ;(print (list r c))
         (return-from tree-search-matrix (setf result
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 0 (nth 0 (nth 1 this-hash)))
                                                        (nth 1 (nth 0 (nth 1 this-hash)))))))))
        ((= 2 (car this-hash)) (progn ;(print (list r c))
         (return-from tree-search-matrix (setf result (longest-list 
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 0 (nth 0 (nth 1 this-hash)))
                                                        (nth 1 (nth 0 (nth 1 this-hash)))))
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 0 (nth 1 (nth 1 this-hash)))
                                                        (nth 1 (nth 1 (nth 1 this-hash))))))))))
        ((= 3 (car this-hash)) (progn ;(print (list r c))
         (return-from tree-search-matrix (setf result (longest-list 
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 0 (nth 0 (nth 1 this-hash)))
                                                        (nth 1 (nth 0 (nth 1 this-hash)))))
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 0 (nth 1 (nth 1 this-hash)))
                                                        (nth 1 (nth 1 (nth 1 this-hash)))))
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 0 (nth 2 (nth 1 this-hash)))
                                                        (nth 1 (nth 2 (nth 1 this-hash)))))))))))
  result))
        
#|
(defun main (matrix)
  (let ((resultList '())
        (indexTable (gen-index-table matrix))
        (rowNum (length matrix))
        (colNum (length (car matrix))))
    (loop for r from 0 to (1- rowNum) do
         (loop for c from 0 to (1- colNum) do
                (setf resultList (append resultList (list (sum-list
                                                           (tree-search-matrix indexTable r c)))))))
    (return-from main (max-list resultList))))|#

(defun foreplay (chan indexTable r c)
  (chanl:send chan (sum-list (tree-search-matrix indexTable r c))))

(defun main-concurrency (matrix)
  (let ((resultList '())
        (indexTable (gen-index-table matrix))
        (rowNum (length matrix))
        (colNum (length (car matrix)))
        (chan (make-instance 'chanl:channel)))
    (loop for r from 0 to (1- rowNum) do
         (loop for c from 0 to (1- colNum) do
              (chanl:pexec () (foreplay chan indexTable r c))))
    (loop for i from 1 to (* rowNum colNum) do
         (progn 
           (setf resultList (append resultList (list (chanl:recv chan))))
           (print resultList)))
    (return-from main-concurrency (max-list resultList))))
