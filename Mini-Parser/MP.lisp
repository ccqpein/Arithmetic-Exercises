(defvar test1 "324")
(defvar test2 "[123,[456,[789]]]")

(defun unpack (s)
  "unpack the string, return list"
  (let ((sl (coerce s 'list)))
    (if (eql (first sl) #\[)
        (cdr (butlast sl))
        sl)
    ))

(defun get-number-string (sl)
  "get the first numbers string from unpacking parser"
  (coerce
   (loop for i in sl
      while (digit-char-p i)
      collect i) 'string))

(defun get-rest-string (sl)
  (do ((ii 0 (incf ii)))
      ((or (eql (nth ii sl) #\[) (= ii (length sl)))
       (coerce (subseq sl ii) 'string))))

(defstruct NestedInteger
  value
  (Nested nil)
  )

(defun deserialize (s)
  (let ((sl (unpack s))
        (aa (make-NestedInteger)))
    (setf (NestedInteger-value aa) (get-number-string sl))
    (if (eql (car (last sl)) #\])
        (setf (NestedInteger-Nested aa)
              (deserialize (get-rest-string sl))))
    aa
    ))
