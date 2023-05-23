(ql:quickload "str")

(defun longest-common-prefix (strs)
  (if (zerop (length strs)) (return-from longest-common-prefix ""))

  (loop
	with last = (nth 0 strs)
	for i from 1 below (length strs)
	do (setf last (helper last (nth i strs)))
	;;do (print last)
	if (string= "" last)
	   return ""
	finally (return last)
	))

(defun helper (a b)
  (let ((min-boundray (min (length a) (length b))))
	(loop for i from 0 below min-boundray
		  if (char/= (elt a i) (elt b i))
			return (str:substring 0 i a)
		  finally (return (str:substring 0 min-boundray a))))
  )

(defun main ()
  (assert (string= (longest-common-prefix '("flower" "flow" "flighta")) "fl"))
  (assert (string= (longest-common-prefix '("dog" "racecar" "car")) ""))
  (assert (string= (longest-common-prefix '()) ""))
  )
