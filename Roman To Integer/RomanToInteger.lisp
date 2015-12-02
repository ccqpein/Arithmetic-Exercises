(defun switchNumber (s)
  (do* ((char2 0 (car charToNumber )


(defun string-to-list (string)
  "Returns a list of the data items represented in the given list."
  (let ((the-list nil) ;; we'll build the list of data items here
	(end-marker (gensym))) ;; a unique value to designate "done"
    (loop (multiple-value-bind (returned-value end-position)
	      (read-from-string string nil end-marker)
	    (when (eq returned-value end-marker)
	      (return the-list))
	    ;; if not done, add the read thing to the list
	    (setq the-list
		  (append the-list (list returned-value)))
	    ;; and chop the read characters off of the string
	                (setq string (subseq string end-position))))))
