(defun matrix-sum (nums)
  (let ((nums (loop for num in nums collect (sort num #'<))))
    (loop for ind from 0 below (length (nth 0 nums))
          sum (apply #'max (mapcar (lambda (l) (nth ind l)) nums)))))

(defun main ()
  (assert (= 15 (matrix-sum '((7 2 1) (6 4 2) (6 5 3) (3 2 1)))))
  (assert (= 1 (matrix-sum '((1)))))
  )
