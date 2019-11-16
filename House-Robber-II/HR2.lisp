(defun rob (nums)
  (let ((len (length nums)))
    (if (= 0 len) (return-from rob 0))
    (if (<= len 3) (return-from rob (apply #'max nums))))

  (labels ((max-inner (nums)
             (let ((before-max 0)
                   (now-max 0)
                   (temp 0))
               (loop
                  for n in nums
                  do (setf temp now-max
                           now-max (max (+ n before-max) now-max)
                           before-max temp))
               now-max)))
    (max (max-inner (cdr nums)) (max-inner (nbutlast nums))))
  )
