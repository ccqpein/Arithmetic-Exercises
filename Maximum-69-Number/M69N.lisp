(defun maximum69-number (num)
  (let* ((ss (write-to-string num))
         (s-l (str:split "6" ss :limit 2)))
    (cond ((= 1 (length s-l)) num)
          ((string= "" (nth 1 s-l))
           (if (string= (nth 0 s-l) ss)
               (parse-integer ss)
               (parse-integer (str:concat (nth 0 s-l) "9"))))
          (t (parse-integer (str:concat (nth 0 s-l) "9" (nth 1 s-l)))))))
