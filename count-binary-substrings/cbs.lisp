(defparameter *tests*
  '(("00110011" 6)
    ("10101" 4)
    ("000111000" 6)))

(defun count-binary-substrings (s)
  (let* ((all-chars (loop for c across s
                          collect c))
         (all-chars-gourp (chunk-list all-chars)))
    (loop for (x y) on (mapcar #'length all-chars-gourp) by #'cdr
          when (and x y)
            sum (min x y))
    ))

(defun chunk-list (l)
  (loop
    with init = (car l)
    and bucket = '()
    for c in l
    if (equal c init)
      do (push c bucket)
    else
      collect bucket into result
      and do (setf bucket '()
                   init c)
      and do (push c bucket)
    finally (if bucket
                (return (append result (list bucket)))
                (return result))))

(defun main ()
  (loop for (s r) in *tests*
        do (assert (= r (count-binary-substrings s)))))
