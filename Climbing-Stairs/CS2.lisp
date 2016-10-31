(defun factorial (m &optional (n m) &key (c 'nil))
  (let ((result 1))
    (loop for i from m downto (- m n -1)
       do (setf result (* result i))
         )
    (if c
        (/ result (factorial n))
        result)))

