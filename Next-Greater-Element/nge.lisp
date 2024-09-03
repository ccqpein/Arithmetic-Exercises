(defun next-greater-elements (nums)
  (if (null nums) (return-from next-greater-elements nil))

  (let ((bucket nums)
        res)
    (loop for n in (reverse nums)
          do (loop for nn = (pop bucket)
                   if nn
                     do (when (> nn n)
                          (push nn bucket)
                          (push n bucket)
                          (push nn res)
                          (return)
                          )
                   else
                     do (progn (push -1 res)
                               (push n bucket)
                               (return))))
    res
    ))

(defun main ())
