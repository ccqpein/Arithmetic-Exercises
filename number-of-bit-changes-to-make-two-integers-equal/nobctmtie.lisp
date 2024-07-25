(defun min-changes (n k)
  (let ((nn (concatenate 'list (format nil "~B" n)))
        (kk (concatenate 'list (format nil "~B" k))))
    ;;(format t "~a ~a~%" nn kk)
    (let ((nn (fill-prefix nn (max (length nn) (length kk)) #\0))
          (kk (fill-prefix kk (max (length nn) (length kk)) #\0)))
      (loop for n in nn
            for k in kk
            when (and (char= n #\1) (char= k #\0))
              count 1 into result
            when (and (char= n #\0) (char= k #\1))
              do (return-from min-changes -1)
            finally (return result)
            ))))

(defun fill-prefix (l length ele)
  (if (>= (length l) length) (return-from fill-prefix l))
  (append (make-list (- length (length l)) :initial-element ele) l)
  )

(defun main ()
  (assert (= (min-changes 13 4) 2))
  (assert (= (min-changes 13 14) -1))
  (assert (= (min-changes 21 21) 0))
  )
