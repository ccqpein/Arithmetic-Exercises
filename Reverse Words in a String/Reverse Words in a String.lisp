(defun putWordsInList (s)
  (let ((stringList (coerce s 'list))
	(stringR nil))
    (do* ((i 0 (incf i))
	  (c1 (nth i stringList)
	      (nth i stringList))
	  (c2 (nth (+ i 1) stringList)
	      (nth (+ i 1) stringList))
	  (word1 (wordTell c1 c2) (wordTell c1 c2))
	  (word2 (nil) (nil))
	  (wordList () ()))
	 ((equal i (- (length stringList) 1))
	  (setf wordList (setf stringR wordList))
	  (format t "~s" wordList)))))

(defun wordTell (c1 c2 &optional word)
  (cond ((and (string/= c1 #\ ) (string/= c2 #\ ))
	 (return-from wordTell (concatenate 'string word c2)))
	((and (string/= c1 #\ ) (string= c2 #\ ))
	 (return-from wordTell #\ ))
	((and (string= c1 #\ ) (string/= c2 #\ ))
	 (return-from wordTell c2))
	((and (string= c1 #\ ) (string= c2 #\ ))
	 (return-from wordTell nil))
