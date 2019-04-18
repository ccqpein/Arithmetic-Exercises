(defun longest-substring (s k)
  (let ((hashT (loop
                  with temp = (make-hash-table)
                  for c across s
                  do (incf (gethash c temp 0))
                  finally (return temp))))
    hashT))
