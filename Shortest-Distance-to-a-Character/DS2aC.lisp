(defun shortest-to-char (s c)
  (let ((from-head (loop
                     with current = (length s)
                     and flag = -1
                     for ch across s
                     if (char= ch c)
                       do (setf flag 1
                                current 0)
                     else
                       do (incf current flag)
                     end
                     collect current))

        (from-tail (reverse
                    (loop
                      with current = (length s)
                      and flag = -1
                      for ch across (reverse s)
                      if (char= ch c)
                        do (setf flag 1
                                 current 0)
                      else
                        do (incf current flag)
                      end
                      collect current))))
    (format t "head: ~a~%head:~a~%" from-head from-tail)
    (mapcar #'min from-head from-tail)
    ))

(defun main ()
  (assert (equal '(3 2 1 0 1 0 0 1 2 2 1 0) (shortest-to-char "loveleetcode" #\e)))
  (assert (equal '(3 2 1 0) (shortest-to-char "aaab" #\b)))
  )
