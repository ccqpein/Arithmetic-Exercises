(defvar test1 "hello world")
(defvar test2 "a ")

(defun delete-ending-space (str)
  (do ((ind (1- (length str)) (1- ind)))
      ((or (= -1 ind)
           (char/= #\  (elt str ind)))
       (subseq str 0 (1+ ind))))
  )

(defun lolw (str)
  (let* ((sstr (delete-ending-space str))
         (ind (position #\  sstr :from-end t)))
    (if (eql ind nil)
        (length sstr)
        (length (subseq
                    sstr
                    (1+ ind))))))
