(defun repeated-substring-pattern (s)
  (labels ((helper (s len count)
             (let ((a (subseq s 0 len)))
               (loop for i from 1 below count
                     when (string/= a (subseq s (* len i) (+ len (* len i))))
                       do (return-from helper nil)
                     finally (return t)))))
    (loop
      for len from 1 to (floor (/ (length s) 2))
      if (= 0 (mod (length s) len))
        when (helper s len (floor (/ (length s) len)))
          do (return-from repeated-substring-pattern t)
      end
      finally (return nil)
      )))
