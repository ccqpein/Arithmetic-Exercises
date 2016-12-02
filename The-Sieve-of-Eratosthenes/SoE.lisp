
;; this code come from ../Sqrt-x
(defun my-sqrt (x)
  (cond ((= x 1) (return-from my-sqrt 1))
        ((= x 0) (return-from my-sqrt 0)))
  
  (do* ((r1 0.0d0 r2)
        (r2 1.0d0 (/ (+ r1 (/ x r1)) 2)))
       ((= r2 r1) (return-from my-sqrt r2))
    ))


