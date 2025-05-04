(defun rotate-right (head k)
  (let* ((len (length head))
         (k (mod k len)))
    (if (zerop len) (return-from rotate-right nil))
    (if (zerop k) (return-from rotate-right head))

    (append (loop for i from (- len k) below len collect (nth i head))
            (loop for i from 0 below (- len k) collect (nth i head)))
    ))

(defun main ()
  (format t "~a~%" (rotate-right '(1 2 3 4 5) 2)))
