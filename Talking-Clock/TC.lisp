(ql:quickload "str")

(defun run (input)
  (let ((x (str:split ":" input)))
    (let (fst snd am-or-pm argvs)
      (setf fst (parse-integer (car x))
            snd (parse-integer (cadr x)))
      
      (if (< fst 12)
          (setf am-or-pm "am"
                argvs (list (if (= 0 fst) 12 fst)))
          (setf am-or-pm "pm"
                argvs (list (if (= 12 fst) fst (- fst 12)))))

      (if (< snd 10)
          (setf argvs (append argvs (if (= snd 0) '() (list "oh" (format nil "~R" snd)))))
          (setf argvs (append argvs (str:split "-" (format nil "~R" snd)))))
      (setf argvs (append argvs (list am-or-pm)))

      ;;(print argvs)
      
      (format nil "It's ~{~R ~#[~;~a~;~a ~a~;~a ~a ~a~]~}"
              argvs))))

(defun main ()
  (let ((ll '(
              ("00:00" "It's twelve am")
              ("01:30" "It's one thirty am")
              ("12:05" "It's twelve oh five pm")
              ("14:01" "It's two oh one pm")
              ("20:29" "It's eight twenty nine pm")
              ("21:00" "It's nine pm")
              )))
    (loop for (in out) in ll
          do (assert (string= out (run in))))))
