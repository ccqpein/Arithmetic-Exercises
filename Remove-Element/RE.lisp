(defun removeElement (nums val)
  (let ((newNums))
    (loop for i in nums
       when (/= i val) do (setf newNums (append newNums (list i))))
    (return-from removeElement newNums)))
