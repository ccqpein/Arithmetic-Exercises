(ql:quickload "str")
(defun defang-i-paddr (address)
  (let ((chars (concatenate 'list address)))
    (str:join ""
              (loop for c in chars
                    append (if (char= #\. c) '("[" "." "]") `(,(concatenate 'string `(,c))))))))
