(defun removeElement (v vl)
  (loop for i in vl
     when (/= i v)
     collect i))
