(defun changeStringtoChar (str)
  (let ((charList nil))
    (setf charList (coerce str 'list))
    charList))

(defun judgeChar (char word)
  (cond ((char/= char #\Space)
         (progn
           (push char word)
           (return-from judgeChar word)))
        ((char= char #\Space)
         (progn (push char word) (setf word (reverse word))
                (return-from judgeChar word)))
        ((eql char nil)
         (progn (push #\Space word) (setf word (reverse word))
                  (return-from judgeChar word)))))

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

(defun changeWordslistToString (wordsList)
  (return-from changeWordslistToString wordsList))
;  (let ((FinalString))
;    (loop for word in wordsList do
       

(defun pushCharToList (charList)
  "charlist cannot be empty"
  (do* ((charList (deleteSpacesBeginning (changeStringtoChar charList))
                  (cdr charList))
        (char (car charList)
              (car charList))
        (word (list char)
              (judgeChar char word))
        (wordsList (list )
                   (if (or (char= char #\Space) (eql char nil))
                       (push word wordsList))))
       ((eql char nil) (let ((FinalString))
                           (setf FinalString (changeWordslistToString wordsList))
                           (return-from pushCharToList FinalString)))))
;;;;The value NIL is not of type CHARACTER.
