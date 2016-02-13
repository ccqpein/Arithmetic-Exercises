(defpackage :Climbing-Stairs
  (:use :cl)
  (:export :factorial))

(defun factorial (n &optional m &key (c 'nil))
  "C n qu m"
  (let ((result 1)
        (times (if m
                   m
                   n))
        (cOption 1))
    (dotimes (i times (if (not c)
                          result
                          (/ result cOption)))
      (setf result (* result n))
      (setf n (- n 1))
      (setf cOption (* cOption (+ i 1))))))

(defun genList (n)
  (let ((resultList '()))
    (loop for i from 0 to n
       do (if (= (mod (- n i) 2) 0)
              (setf resultList
                    (append resultList (list (list i (/ (- n i) 2)))))))
    (return-from genList resultList)))

(defun CS (n)
  (let ((resultList (genList n))
        (allWaysSum 0))
    (loop for i in resultList
         do (factorial (sum (first     
