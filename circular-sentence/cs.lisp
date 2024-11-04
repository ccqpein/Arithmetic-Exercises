(defun is-circular-sentence (sentence)
  (let ((sentence (concatenate 'list sentence)))
    (loop for (a b c) on sentence
          when (and a b c)
            when (and (char= b #\ ) (char/= a c))
              return nil
          finally (return (and t (char= (first sentence) (car (last sentence))))))))

(defun main ()
  (assert (not (is-circular-sentence "leetcode")))
  (assert (is-circular-sentence "eetcode"))
  (assert (not (is-circular-sentence "Leetcode eisc cool"))))
