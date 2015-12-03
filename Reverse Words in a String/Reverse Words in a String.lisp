(defun putWordsInList (s)
  (do* ((i 0 (incf i))
	(c (nth i (coerce s 'list))
	   (nth i (coerce s 'list)))
	(word 
