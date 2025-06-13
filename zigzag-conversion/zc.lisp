(ql:quickload '("str" "alexandria"))

(defun convert (s num-rows)
  (let ((s (concatenate 'list s))
        (f t))
    (labels ((ind-find (r)
               (cond ((= 1 num-rows)
                      r)
                     ((and f
                           (= (1- num-rows) r))
                      (setf f (not f))
                      (1- r))
                     ((and (not f)
                           (= 0 r))
                      (setf f (not f))
                      (1+ r))
                     (t (if f (1+ r) (1- r))))))
      (loop with r = 0
            and res = (loop for i from 0 below num-rows collect '())
            for b in s
            do (push b (nth r res))
               (setf r (ind-find r))
            finally (return (concatenate 'string
                                         (alexandria:flatten
                                          (mapcar #'reverse res))))))))
