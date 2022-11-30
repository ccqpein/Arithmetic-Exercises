(defvar *max-ind* 0)

(defun find-k-distant-indices (ll key k)
  (let ((*max-ind* (length ll)))
	(sort
	 (remove-duplicates
	  (loop
		for i from 0 below (length ll)
		for v = (nth i ll)
		append (if (= v key) (give-range i k))))
	 #'<)))

(defun give-range (i k)
  (loop for ii from (max 0 (- i k)) to (min (1- *max-ind*) (+ i k))
		collect ii))

(find-k-distant-indices '(3 4 9 1 3 9 5) 9 1)
(find-k-distant-indices '(2 2 2 2 2) 2 2)
