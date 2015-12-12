(defun changeStringtoChar (str)
  (let ((charList nil))
    (setf charList (coerce str 'list))
    charList))

(defun judgeChar (char word wordsList positionNum)
  (cond ((and (char/= char #\ ) (/= positionNum 2)) 
         (push char word))
        ((and (char/= char #\ ) (= positionNum 2))
         ((push char word) (setf word (reverse word)) (push word wordsList)))
        ((and (char= char #\ ) (/= positionNum 0))
         ((push char word) (setf word (reverse word)) (push word wordsList))
        ))

(defun pushCharToList (charList)
  (let ((wordsList)
        (word)
        (positionNum))
    (dolist (char charList)
      (judgeChar char word wordsList)
            
