(defun cut-string (str char)
  (if (eql nil char)
      (list str)
      (let ((result '()))
        (loop
           with temp = '()
           for c across str
           if (char= c char)
           do (setf result (append result (list (concatenate 'string (reverse temp))))
                    temp '())
           else
           do (setf temp (cons c temp))
           finally (setf result (append result (list (concatenate 'string (reverse temp))))))
        (remove-if #'(lambda (x) (equal "" x)) result)
        )))

(defun longest-substring (s k)
  (let* ((hashT (loop
                  with temp = (make-hash-table)
                  for c across s
                  do (incf (gethash c temp 0))
                  finally (return temp)))
        (cut (loop
                for key being each hash-key in hashT
                using (hash-value v)
                when (< v k)
                return key))
         (temp-list (cut-string s cut)))
    ;;(print temp-list)
    (if (= 1 (length temp-list))
        (length (car temp-list))
        (reduce #'max (mapcar #'(lambda (x) (longest-substring x k)) temp-list)))
    ))

(longest-substring "ababbc" 2)
(longest-substring "aaabb" 3)
(longest-substring "aaabbb" 3)
