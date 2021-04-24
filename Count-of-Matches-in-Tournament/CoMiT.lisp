(defun number-of-matches (n)
  (loop
    with s = 0
    until (= n 1)
    do (progn (incf s (floor (/ n 2)))
              (setf n (- n (floor (/ n 2))))
              ;;(print n)
              )
    finally (return s)
    ))
