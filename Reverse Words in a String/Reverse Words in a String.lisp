(defun changeStringtoChar (str)
  (let ((charList nil))
    (setf charList (coerce str 'list))
    charList))

(defun judgeChar (char word)
  (cond ((typep char 'null)
         (progn (push #\Space word) (setf word (reverse word))
                  (return-from judgeChar word)))
        ((char/= char #\Space)
         (progn
           (push char word)
           (return-from judgeChar word)))
        ((char= char #\Space)
         (progn (push char word) (setf word (reverse word))
                (return-from judgeChar word)))
        ))

(defun deleteSpacesBeginning (charList)
  (let ((charList2 charList))
    (loop for i in charList
       until (char/= i #\Space )
       do
         (setf charList2 (cdr charList2)))
    (loop for i in (reverse charList)
       until (char/= i #\Space )
       do
         (setf charList2 (butlast charList2)))
    (return-from deleteSpacesBeginning charList2)))
       
(defun pushCharToList (charList)
  (do* ((charList2 (deleteSpacesBeginning (changeStringtoChar charList))
                  (cdr charList2))
        (char (car charList2)
              (car charList2))
        (word (list char)
              (judgeChar char word))
        (wordsList (list )
                   (if (or (char= char #\Space) (typep char 'null))
                       (push word wordsList))))
       ((typep char 'null) (let ((FinalString))
                           (setf FinalString (changeWordslistToString wordsList))
                           (return-from pushCharToList FinalString)))))

(defun changeWordslistToString (wordsList)
  (format t wordsList))
