(quicklisp:quickload 'split-sequence)

(defun length-of-longest-substring (s)
  (let ((result 0)
        (temp-cache '())
        )
    (loop
       for c across s
       if (member c temp-cache)
       do (progn
            (if (> (length temp-cache) result)
                (setf result (length temp-cache)))
            (let ((ind (position c temp-cache)))
              (setf temp-cache
                    (append (caar (multiple-value-list
                                   (split-sequence:split-sequence 0 temp-cache :start (+ 1 ind))))
                            (list c)))))
       else do
         (setf temp-cache (append temp-cache (list c))))
    (if (> (length temp-cache) result)
        (length temp-cache)
        result)
    ))

(length-of-longest-substring "abcabcb") ;; => 3
(length-of-longest-substring "bbb") ;; => 1
(length-of-longest-substring "pwwkew") ;; => 3
(length-of-longest-substring "dvdf") ;; => 3
