(defun decode-at-index (str k)
  ;;(format t "str:~a k:~a ~%" str k)
  (let ((l 0))
    (loop
      for c across str
      for this-num = (parse-integer (string c) :junk-allowed t)
      if this-num
        do (if (<= k (* l this-num))
               (if (= 0 (mod k l))
                   (return-from decode-at-index (decode-at-index str l))
                   (return-from decode-at-index (decode-at-index str (mod k l)))
                   ))
        and do (setf l (* l this-num))
      else
        do (incf l)
        and do (if (= l k)
                   (return-from decode-at-index (string c))))))

(defun main ()
  (assert (string= (decode-at-index "decode-float" 10) "o"))
  (assert (string= (decode-at-index "ha22" 5) "h"))
  (assert (string= (decode-at-index "a2345678999999999999999" 1) "a"))
  (assert (string= (decode-at-index "a23" 6) "a"))
  )

(main)
