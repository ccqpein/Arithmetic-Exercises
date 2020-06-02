(defun summary-ranges (nums)
  (if (not nums) (return-from summary-ranges nil))
  (labels ((make-result-string (start end)
             (if (= start end)
                 (format nil "~a" start)
                 (format nil "~a->~a" start end))))
    (loop
      with start = (car nums)
      with end = (car nums)
      for i in (cdr nums)
      if (> i (1+ end))
        collect (make-result-string start end) into result
        and do (setf start i
                     end i)
      else
        do (setf end i)
      finally (return (nconc result (list (make-result-string start end)))))))
