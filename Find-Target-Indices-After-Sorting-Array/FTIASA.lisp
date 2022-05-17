(defun target-indices (arr target)
  (let ((start 0)
        (count 0))
    (dolist (n arr (if (eq count 0) nil
                       (loop for i from start below (+ start count)
                             collect i)))
      (cond ((eq n target) (incf count))
            ((< n target) (incf start))))))

(defun main ()
  (assert (equal (target-indices '(1 2 5 2 3) 2) '(1 2)))
  (assert (equal (target-indices '(1 2 5 2 3) 3) '(3)))
  (assert (equal (target-indices '(1 2 5 2 3) 5) '(4)))
  )
