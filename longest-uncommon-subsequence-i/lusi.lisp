(defun find-lu-slength (a b)
  (if (string= a b) -1 (max (length a) (length b))))
