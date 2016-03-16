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

(defun get-index-number (str)
  (let ((pre-table (get-pattern-from-head str))
        (bot-table (reverse (get-pattern-from-back str)))
        (indexNum 0))
    ;(print pre-table)
    ;(print bot-table)
    (mapcar #'equal pre-table bot-table)
    ())
