(defpackage #:KMP
  (:export #:do-kmp
           #:*test-string*
           #:*test-pattern*)
  (:use #:CL))

(in-package #:KMP)

(load "../Longest-Increasing-Path-in-a-Matrix/LIPiaM.lisp")

(defvar *test-string* "bbc abcdab abcdabcdabde")
(defvar *test-pattern* "abcdabd")

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
    ;(progn (print pre-table) (print bot-table))
    (setf indexNum
          (mapcar #'(lambda (list1 list2)
                      (let ((num 0))
                        (if (equal list1 list2)
                            (setf num (length list1)))
                  num)) pre-table bot-table))
    ;(print indexNum)
    (setf indexNum (MX:max-list indexNum))))

(defun make-index-table (p)
  (let ((pList (coerce p 'list))
        (indexTable '(-1)))
    (loop for i from 1 to (1- (length pList)) do
         (setf indexTable (append indexTable
                                  (list (get-index-number (subseq p 0 i))))))
    indexTable))

(defun matchp (char1 char2)
  (cond ((char= char1 char2) 0)
        (t 1)))

(defun do-kmp (s p)
  (let* ((strList (coerce s 'list))
         (patList (coerce p 'list))
         (indexTable (make-index-table p))
         (lenPatList (length patList)))
    ;(progn (print strList) (print patList) (print indexTable) (print lenPatList))
    (do* ((strPoint 0 (cond ((= tellStat 0) (incf strPoint))
                            ((and (= tellStat 1) (= patPoint 0))
                             (incf strPoint))
                            ((= tellStat 1) strPoint)))
          (patPoint 0 (cond ((= tellStat 0) (incf patPoint))
                            ((and (= tellStat 1) (/= (nth patPoint indexTable) -1))
                             (nth patPoint indexTable))
                            (t 0)))
          (tellStat (matchp (nth strPoint strList) (nth patPoint patList))
                    (matchp (nth strPoint strList) (nth patPoint patList)))
          )
         ((and (= patPoint (1- lenPatList)) (= tellStat 0))
          (print strList) (print patList)
          (print (+ (- strPoint lenPatList) 2))
          ))))
