(ql:quickload "alexandria")

(defun max-product (words)
  (let (words-set)
	(setf words-set
		  (loop for word in words
				collect (let ((s (make-hash-table :test 'equal)))
						  (dolist (c (concatenate 'list word))
							(setf (gethash c s) t))
						  s)))

	(let ((most 0))
	  (loop for i from 0 below (length words-set)
			do (loop for j from (1+ i) below (length words-set)
					 if (not (if-intersection (nth i words-set)
											  (nth j words-set)))
					   do (setf most (max most (* (length (nth i words))
												  (length (nth j words)))))))
	  most
	  )))

(defun if-intersection (set1 set2)
  (intersection (alexandria:hash-table-keys set1)
				(alexandria:hash-table-keys set2)
				:test 'equal))


(assert (= 16 (max-product '("abcw"
							 "baz"
							 "foo"
							 "bar"
							 "xtfn"
							 "abcdef"))))

(assert (= 4 (max-product '("a"
							"ab"
							"abc"
							"d"
							"cd"
							"bcd"
							"abcd"))))
