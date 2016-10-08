(defvar test1 "hello world")
(defvar test1 "a ")

(defun delete-ending-space (str)

  )

(defun lolw (str)
  (length (subseq
           str
           (1+ (position #\  str :from-end t)))))
