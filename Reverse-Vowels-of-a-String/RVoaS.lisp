(defun reverse-vowels (s)
  (labels ((is-vowel (c) (not (null (member c (concatenate 'list "aeiouAEIOU") :test #'char=)))))
    (let ((sl (concatenate 'list s))
          (i 0)
          (j (1- (length s)))
          b)
      (loop while (< i j)
            for x = `(,(is-vowel (nth i sl)) ,(is-vowel (nth j sl)))
            do (cond ((equal x '(t t))
                      (setf b (nth i sl)
                            (nth i sl) (nth j sl)
                            (nth j sl) b)
                      (incf i) (decf j))
                     ((equal x '(nil t))
                      (incf i))
                     ((equal x '(t nil))
                      (decf j))
                     (t (incf i) (decf j))
                     ))
      (concatenate 'string sl))
    ))

(defun main ()
  (assert (string= "holle" (reverse-vowels "hello")))
  (assert (string= "leotcede" (reverse-vowels "leetcode")))
  (assert (string= "aA" (reverse-vowels "Aa")))
  )
