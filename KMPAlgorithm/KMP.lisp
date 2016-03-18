(defpackage #:KMP
  (:use #:CL))

(in-package #:KMP)

;;; make the string index table
(defun get-pattern-from-back (str)
  (let ((result '())
        (strList (coerce str 'list)))
    (loop for i on strList do
         ;(print i)
         (setf result (append result (list i))))
    (cdr result)))

(defun get-pattern-from-head (str)
  (let ((result '())
        (strList (coerce str 'list)))
    (loop for i from 1 to (1- (length strList)) do
         ;(print strList)
         (setf result (append result (list (subseq strList 0 i)))))
    result))

(defun max-list (list)
      (loop for element in list maximizing element))

(defun get-index-number (str)
  (let ((pre-table (get-pattern-from-head str))
        (bot-table (reverse (get-pattern-from-back str)))
        (indexNum 0))
    (print pre-table)
    (print bot-table)
    (setf indexNum
          (mapcar #'(lambda (list1 list2)
                      (let ((num 0))
                        (if (equal list1 list2)
                            (setf num (length list1)))
                  num)) pre-table bot-table))
    (print indexNum)
    (setf indexNum (max-list indexNum))))

(defun make-index-table (p)
  (let ((pList (coerce p 'list))) 
    (do ((subStr (car pList) (list subStr (car restStr)))
         (restStr (cdr pList) (cdr restStr)))
        ((

(defun do-kmp (s p)
  (let* ((strList (coerce s 'list))
         (patList (coerce p 'list))
         (indexTable (get-index-number p)))
    (print strList)
    (print patList)
    (print indexTable)#|
    (do ((sPoint (car strList))
         (pPoint (car |#))
