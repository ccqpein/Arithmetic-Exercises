(defvar test1 "hello world")

(defun lolw (str)
  (length (subseq
           str
           (1+ (position #\  str :from-end t)))))
