(ql:quickload "str")

(defun length-after-transformations (s times)
  (let ((cache (make-list 26 :initial-element 0))
        (s (mapcar (lambda (c) (- c (char-code #\a)))
                   (mapcar #'char-code (concatenate 'list s)))))
    (loop for ind in s
          do (incf (nth ind cache)))

    (let ((last-c 0)
          (cache-c 0))
      (dotimes (x times)
        (setf last-c (nth 0 cache))
        (loop for ind from 1 to 25
              do (setf cache-c (nth ind cache)
                       (nth ind cache) last-c
                       last-c cache-c))
        (setf (nth 0 cache) last-c)
        (incf (nth 1 cache) last-c)
        (setf cache (mapcar (lambda (x) (mod x (+ 1000000000 7))) cache))))

    (mod (apply #'+ cache) (+ 1000000000 7))))

(defun main ()
  (assert (= 7 (length-after-transformations "abcyy" 2)))
  (assert (= 5 (length-after-transformations "azbk" 1)))
  (assert (= 79033769 (length-after-transformations "jqktcurgdvlibczdsvnsg" 7517))))
