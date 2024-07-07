(ql:quickload "alexandria")

(defparameter *tests* `(((4 6 7) 4)
                        ((4 6 7 7) 8)
                        ((4 4 3 2 1) 1)
                        (,(loop for i from 1 to 15 collect i) 32752)))

(defun find-subsequences (c)
  (let ((set (make-hash-table :test 'equal))
        (cache '()))
    (labels ((dfs (i ret cache nums)
               (when (= i (length nums))
                 (when (> (length cache) 1)
                   (setf (gethash cache ret) t))
                 (return-from dfs))

               (dfs (1+ i) ret cache nums)

               (unless (and (car cache) (> (car cache) (nth i nums)))
                 (dfs (1+ i) ret (cons (nth i nums) cache) nums))
               ))
      (dfs 0 set cache c)
      (alexandria:hash-table-keys set)))
  )

(defun main ()
  (dolist (c *tests*)
    (assert (= (second c)
               (length (find-subsequences (car c)))))))
