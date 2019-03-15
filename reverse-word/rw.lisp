(defun reverse-word (s)
  (let ((result '())
		(cache '()))
	(loop
	   for c across s
	   if (char= c #\ )
	   do (setf result (append result cache)
				cache '())
	   and do (setf result (append result (list #\ )))

	   else	do (setf cache (cons c cache))

	   finally (setf result (append result cache)))
	(concatenate 'string result))
  )

(reverse-word "  hello world   ")
(reverse-word "hello world")
(reverse-word "  hello world")
