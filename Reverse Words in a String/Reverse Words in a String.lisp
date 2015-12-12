(defun changeStringtoChar (str)
  (let ((charList nil))
    (setf charList (coerce str 'list))
    charList))

(defun judgeChar (char word)
  (cond ((char/= char #\ ) 
         (progn
           (push char word)
           (return-from judgeChar word)))
        ((char= char #\ )
         (progn (push char word) (setf word (reverse word))
                (return-from judgeChar word)))))

(defun deleteSpacesBeginning (charList)
  (let ((charList2 charList))
    (loop for i in charList do
         (if (char= i #\ )
             (setf charList2 (cdr charList2))))
    (loop for i in (reverse charList) do
         (if (char= i #\ )
             (setf charList2 (butlast charList2))))
    (return-from deleteSpacesBeginning charList2)))

(defun pushCharToList (charList)
  (let ((wordsList)
        (word)
        (charList (deleteSpacesBeginning charList)))
    (dolist (char charList)
      (judgeChar char word wordsList))
    wordsList))

(defun pushCharToList (charList)
  "charlist cannot be empty"
  (do* ((charList (deleteSpacesBeginning charList)
                  (cdr charList))
        (char (car charList)
              (car charList))
        (word char
              (judgeChar char word))
        (wordsList nil
                   (if (and (char= char #\ ) (char= char nil))
                       (push word wordsList)))
