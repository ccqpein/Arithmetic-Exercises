(defun longest-palindrome (s)
  (let* ((ss (coerce s 'list))
         (len (length ss)))
    (if (<= len 1) (return-from longest-palindrome s))

    (loop
      with record-start = 0
      with largest-size = 0

      for start from 0 to (- len 2)
      when (>= (- len start) largest-size)
        do (loop
             for end from (1- len) downto (1+ start)
             when (>= (- end start -1) largest-size)
               do (multiple-value-bind (b size)
                      (inner-loop ss start end)
                    (if (and b (> size largest-size))
                        (setf largest-size size
                              record-start start) 
                        )))
      finally (progn (format t "~a" (list record-start largest-size))
                     (return (coerce (subseq ss record-start (+ record-start largest-size 1)) 'string))))))

(defun inner-loop (ss start end)
  ;;(format t "~a; ~a~%" start end)
  (if (= 0 (- end start))
      (return-from inner-loop (values t (- end start))))

  (loop
    for i from 0 to (- end start 1)
    when (char/= (nth (+ start i) ss)
                 (nth (- end i) ss))
      do (return-from inner-loop (values nil 0)))
  
  (values t (- end start)))

(defun main ()
  (values
   (string= "bab" (longest-palindrome "babad"))
   (string= "" (longest-palindrome ""))
   (string= "bb" (longest-palindrome "cbbd"))))
