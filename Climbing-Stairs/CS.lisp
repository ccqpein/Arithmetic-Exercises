(defpackage :Climbing-Stairs
  (:use :cl)
  (:export))

(defun factorial (n &optional m &key (c 'nil))
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
    
