(defun sort-by-bites (nums)
  (loop
    with result = (make-list 16 :initial-element (list))
    for num in nums
    do (setf (nth (logcount num) result)
             (cons num (nth (logcount num) result)))
    finally (return (apply 'concatenate 'list
                           (mapcar (lambda (l) (sort l #'<))
                                   (remove nil result))))))
