(defun gcd-of-strings (s1 s2)
  (if (string/= (str:concat s1 s2) (str:concat s2 s1))
      ""
      (str:substring 0 (gcd (length s1) (length s2)) s1)))

(defun main ()
  (assert (string= (gcd-of-strings "ABCABC" "ABC") "ABC"))
  (assert (string= (gcd-of-strings "ABABAB" "ABAB") "AB"))
  (assert (string= (gcd-of-strings "LEET" "CODE") ""))
  )
