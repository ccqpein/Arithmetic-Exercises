(defun thousand-separator (num)
  (loop
    with count = 0
    for c across (reverse (write-to-string num))
    if (= 3 count)
      do (setf count 1)
      and collect #\. into result
    else
      do (incf count 1)
    end
    collect c into result
    finally (return (concatenate 'string (reverse result)))
    ))

(defun main ()
  (assert (string= "997" (thousand-separator 997)))
  (assert (string= "1.234" (thousand-separator 1234)))
  (assert (string= "123.456.789" (thousand-separator 123456789)))
  (assert (string= "0" (thousand-separator 0))))
